use crate::routing::{
    add_css, add_games, add_html_pages, add_images, add_js, add_others, add_pdf, add_videos,
};
use axum::extract::connect_info::IntoMakeServiceWithConnectInfo;
use axum::Router;
use hyper::server::conn::{AddrStream, Http};
use rustls_pemfile::{certs, pkcs8_private_keys};
use std::net::SocketAddr;
use std::path::{Path, PathBuf};
use std::sync::Arc;
use std::{fs::File, io::BufReader};
use tokio_rustls::rustls::{Certificate, PrivateKey, ServerConfig};
use tokio_rustls::TlsAcceptor;
use tower::MakeService;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

pub fn rustls_server_config(key: impl AsRef<Path>, cert: impl AsRef<Path>) -> Arc<ServerConfig> {
    // Read the key and certificate files
    let mut key_reader = BufReader::new(File::open(key).unwrap());
    let mut cert_reader = BufReader::new(File::open(cert).unwrap());

    // Extract the key and certificate(s)
    let key = PrivateKey(pkcs8_private_keys(&mut key_reader).unwrap().remove(0));
    let certs = certs(&mut cert_reader)
        .unwrap()
        .into_iter()
        .map(Certificate)
        .collect();

    // Create a configuration with the key and certificate
    let mut config = ServerConfig::builder()
        .with_safe_defaults()
        .with_no_client_auth()
        .with_single_cert(certs, key)
        .expect("bad certificate/key");

    // Set the protocols
    config.alpn_protocols = vec![b"h2".to_vec(), b"http/1.1".to_vec()];

    // Return a thread-safe shared pointer to the configuration
    Arc::new(config)
}

pub fn load_certificate() -> Arc<ServerConfig> {
    // Create a registry for the key and certificate
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "example_tls_rustls=debug".into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    // Read the key and certificate
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

    // Return the resulting configuration
    rustls_config
}

pub fn create_app() -> Router {
    // Create a router with every file in the server
    let mut app = Router::new();
    app = add_html_pages(app);
    app = add_css(app);
    app = add_images(app);
    app = add_videos(app);
    app = add_pdf(app);
    app = add_js(app);
    app = add_games(app);
    app = add_others(app);
    app
}

pub fn handle_request(
    app: &mut IntoMakeServiceWithConnectInfo<Router, SocketAddr>,
    stream: AddrStream,
    acceptor: TlsAcceptor,
    protocol: Arc<Http>,
) {
    //Create a new service from the app
    let svc = app.make_service(&stream);

    // Spawn a new future to handle the connection further
    tokio::spawn(async move {
        // Accept the request
        let stream = match acceptor.accept(stream).await {
            Ok(stream) => stream,
            Err(error) => {
                eprintln!("Failed to accept the connection: {error}");
                return;
            }
        };

        // Await creation of app
        let svc = match svc.await {
            Ok(svc) => svc,
            Err(error) => {
                eprintln!("Failed to create service: {error}");
                return;
            }
        };

        // Serve the connection
        if let Err(error) = protocol.serve_connection(stream, svc).await {
            eprintln!("Failed to serve connection: {error}");
        }
    });
}
