use axum::Router;
use axum::extract::connect_info::IntoMakeServiceWithConnectInfo;
use hyper::server::conn::{AddrStream, Http};
use tokio_rustls::TlsAcceptor;
use crate::routing::{
    add_css, add_games, add_html_pages, add_images, add_js, add_others, add_pdf, add_videos,
    add_wasm,
};
use rustls_pemfile::{certs, pkcs8_private_keys};
use std::net::SocketAddr;
use std::sync::Arc;
use std::{
    fs::File,
    io::BufReader};
use std::path::{Path, PathBuf};
use tokio_rustls::rustls::{Certificate, PrivateKey, ServerConfig};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};
use tower::MakeService;

pub fn rustls_server_config(key: impl AsRef<Path>, cert: impl AsRef<Path>) -> Arc<ServerConfig> {
    let mut key_reader = BufReader::new(File::open(key).unwrap());
    let mut cert_reader = BufReader::new(File::open(cert).unwrap());

    let key = PrivateKey(pkcs8_private_keys(&mut key_reader).unwrap().remove(0));
    let certs = certs(&mut cert_reader)
        .unwrap()
        .into_iter()
        .map(Certificate)
        .collect();

    let mut config = ServerConfig::builder()
        .with_safe_defaults()
        .with_no_client_auth()
        .with_single_cert(certs, key)
        .expect("bad certificate/key");

    config.alpn_protocols = vec![b"h2".to_vec(), b"http/1.1".to_vec()];

    Arc::new(config)
}

pub fn load_certificate() -> Arc<ServerConfig> {
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "example_tls_rustls=debug".into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    #[cfg(target_arch = "aarch64")]
    let rustls_config = rustls_server_config(
        PathBuf::from("/etc/letsencrypt/live/michaeljoy.nl/privkey.pem"),
        PathBuf::from("/etc/letsencrypt/live/michaeljoy.nl/fullchain.pem"),
    );

    #[cfg(not(target_arch = "aarch64"))]
    let rustls_config = rustls_server_config(
        PathBuf::from("../michaeljoy_certificates/key.pem"),
        PathBuf::from("../michaeljoy_certificates/certificate.pem"),
    );

    rustls_config
}

pub fn create_app() -> Router {
    let mut app = Router::new();
    app = add_html_pages(app);
    app = add_css(app);
    app = add_images(app);
    app = add_videos(app);
    app = add_pdf(app);
    app = add_js(app);
    app = add_wasm(app);
    app = add_games(app);
    app = add_others(app);
    app
}

pub fn handle_request(app: &mut IntoMakeServiceWithConnectInfo<Router, SocketAddr>, stream: AddrStream, acceptor: TlsAcceptor, protocol: Arc<Http>){
    let svc = app.make_service(&stream);

    let acceptor = acceptor.clone();
    let protocol = protocol.clone();

    tokio::spawn(async move {
        if let Ok(stream) = acceptor.accept(stream).await {
            let _ = protocol.serve_connection(stream, svc.await.unwrap()).await;
        }
    });
}
