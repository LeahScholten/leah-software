#![deny(
    clippy::panic,
    clippy::panic_in_result_fn,
    clippy::panicking_unwrap,
    non_fmt_panics,
    unconditional_panic,
    unsafe_code
)]
#![warn(clippy::pedantic, clippy::nursery)]
#![feature(async_closure)]

use futures_util::future::poll_fn;
use hyper::server::{
    accept::Accept,
    conn::{AddrIncoming, Http},
};
use service::{load_certificate, create_app, process_request};
use std::{
    net::SocketAddr,
    pin::Pin,
    sync::Arc,
};
use tokio::net::TcpListener;
use tokio_rustls::TlsAcceptor;

mod routing;
mod service;

#[tokio::main]
async fn main() {
    let rustls_config = load_certificate();

    let acceptor = TlsAcceptor::from(rustls_config);

    #[cfg(target_arch = "aarch64")]
    let listener = TcpListener::bind("192.168.178.141:443").await.unwrap();
    #[cfg(target_arch = "aarch64")]
    println!("Running ARM server");
    #[cfg(not(target_arch = "aarch64"))]
    let listener = TcpListener::bind("[::]:443").await.unwrap();
    
    let mut listener = AddrIncoming::from_listener(listener).unwrap();

    let protocol = Arc::new(Http::new());

    // build the application
    let app = create_app();

    let mut app = app.into_make_service_with_connect_info::<SocketAddr>();

    loop {
        let Some(stream) = poll_fn(|cx| Pin::new(&mut listener).poll_accept(cx))
            .await else{
            println!("Failed to poll for a new request: no request found!");
            return;
        };

        let stream = match stream {
            Ok(stream) => stream,
            Err(error) => {
                println!("Failed to poll for a new request: {error:?}");
                return;
            }
        };

        process_request(stream, acceptor.clone(), protocol.clone(), &mut app).await;
    }
}
