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
use service::{create_app, handle_request, load_certificate};
use std::{net::SocketAddr, pin::Pin, sync::Arc, time::Instant};
use tokio::net::TcpListener;
use tokio_rustls::TlsAcceptor;

mod routing;
mod service;

const SECONDS_PER_DAY: u64 = 3600 * 24;

#[tokio::main]
async fn main() {
    // Load the HTTPS certificates
    let mut rustls_config = load_certificate();

    // Create a Tls acceptor from the certificates
    let mut acceptor = TlsAcceptor::from(rustls_config);

    // Used to calculate the time since the last update
    let mut last_tls_update = Instant::now();

    // Create a network listener for https
    #[cfg(target_arch = "aarch64")]
    let listener = TcpListener::bind("192.168.178.141:443").await.unwrap();
    #[cfg(not(target_arch = "aarch64"))]
    let listener = TcpListener::bind("[::]:443").await.unwrap();

    // Turn the network listener into a network stream
    let mut listener = AddrIncoming::from_listener(listener).unwrap();

    // Create a HTTP protocol instance
    let protocol = Arc::new(Http::new());

    // build the application
    let app = create_app();

    // Turn the application into a service
    let mut app = app.into_make_service_with_connect_info::<SocketAddr>();

    loop {
        // Wait for a connection
        let Some(stream) = poll_fn(|cx| Pin::new(&mut listener).poll_accept(cx)).await else {
            println!("Failed to poll for a new request: no request found!");
            return;
        };

        // Skip this connection if it is invalid
        let stream = match stream {
            Ok(stream) => stream,
            Err(error) => {
                println!("Failed to poll for a new request: {error:?}");
                return;
            }
        };

        // Update the certificate if the last update was more than a day ago
        if last_tls_update.elapsed().as_secs() > SECONDS_PER_DAY {
            rustls_config = load_certificate();
            acceptor = TlsAcceptor::from(rustls_config);
            last_tls_update = Instant::now();
        }

        // Handle the request
        handle_request(&mut app, stream, acceptor.clone(), protocol.clone());
    }
}
