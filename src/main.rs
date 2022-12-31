#![deny(clippy::panic, clippy::panic_in_result_fn,clippy::panicking_unwrap,
    non_fmt_panics, unconditional_panic, unsafe_code)]
#![warn(clippy::pedantic, clippy::nursery)]
#![feature(async_closure)]

use axum::{routing::get, Router, response::{IntoResponse, Html}};
use futures_util::future::poll_fn;
use hyper::{server::{conn::{Http, AddrIncoming}, accept::Accept}, HeaderMap, header};
use rustls_pemfile::{certs, pkcs8_private_keys};
use std::{
    fs::File,
    io::BufReader,
    net::SocketAddr,
    path::{Path, PathBuf},
    pin::Pin,
    sync::Arc,
};
use tokio::{net::TcpListener, fs::read};
use tokio_rustls::{
    rustls::{Certificate, PrivateKey, ServerConfig},
    TlsAcceptor,
};
use tower::MakeService;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

async fn read_file(filename: String) -> Vec<u8>{
    read(filename).await.unwrap_or_else(|error| error.to_string().bytes().collect())
}

fn css(content: Vec<u8>) -> impl IntoResponse{
    let mut headers = HeaderMap::new();
    headers.insert(header::CONTENT_TYPE, "text/css".parse().unwrap());
    (headers, content)
}

fn image(content: Vec<u8>) -> impl IntoResponse{
    let mut headers = HeaderMap::new();
    headers.insert(header::CONTENT_TYPE, "image/*".parse().unwrap());
    (headers, content)
}

fn pdf(content: Vec<u8>) -> impl IntoResponse{
    let mut headers = HeaderMap::new();
    headers.insert(header::CONTENT_TYPE, "application/pdf".parse().unwrap());
    (headers, content)
}

fn js(content: Vec<u8>) -> impl IntoResponse{
    let mut headers = HeaderMap::new();
    headers.insert(header::CONTENT_TYPE, "application/javascript".parse().unwrap());
    (headers, content)
}

fn wasm(content: Vec<u8>) -> impl IntoResponse{
    let mut headers = HeaderMap::new();
    headers.insert(header::CONTENT_TYPE, "application/wasm".parse().unwrap());
    (headers, content)
}

fn add_html_pages(mut app: Router) -> Router{
    let routes = vec!["/", "/zakelijk.html", "/technisch.html", "/algemeen.html", "/kerst.html"];
    for route in routes{
        if route == "/"{
            app = app.route(route,  get(async || Html(read_file("src/html/index.html".to_owned()).await)));
        }else{
            app = app.route(route,  get(async || Html(read_file("src/html".to_owned() + route).await)));
        }
    }
    app
}

fn add_css(mut app: Router) -> Router{
    let routes = vec!["/standard.css"];
    for route in routes{
        app = app.route(route,  get(async || css(read_file("src/css".to_owned() + route).await)));
    }
    app
}

fn add_images(mut app: Router) -> Router{
    let routes = vec!["/favicon.ico"];
    for route in routes{
        app = app.route(route,  get(async || image(read_file("src/img".to_owned() + route).await)));
    }
    app
}

fn add_videos(mut app: Router) -> Router{
    let routes = vec![
        "/raspberryPico/7segmentCounter.mp4",
        "/raspberryPico/binaryAnalogLeds.mp4",
        "/raspberryPico/binaryLedCounter.mp4",

        "/ZUMO32U4/objectTracing.mp4",
        "/ZUMO32U4/rotatingInPlace.mp4",

        "/ATmega328P/hapticWire.mp4",
        "/ATmega328P/lightDensityMeter.mp4",
        "/ATmega328P/quadWalkingLightShow.mp4",
        "/ATmega328P/rgbTraficLight.mp4",
        "/ATmega328P/walkingLight.mp4",

        "/Arduino/automaticLight.mp4",
        "/Arduino/rgbLightShow.mp4"
    ];
    for route in routes{
        app = app.route(route,  get(async || read_file("src/video".to_owned() + route).await));
    }
    app
}

fn add_pdf(mut app: Router) -> Router{
    let routes = vec!["/cv.pdf"];
    for route in routes{
        app = app.route(route,  get(async || pdf(read_file("src/pdf".to_owned() + route).await)));
    }
    app
}

fn add_js(mut app: Router) -> Router{
    let routes = vec!["/kerst-9919c4562d434f4c.js"];
    app = app.route(routes[0],  get(async || js(read_file("src/js/kerst.js".to_owned()).await)));
    app
}

fn add_wasm(mut app: Router) -> Router{
    let routes = vec!["/kerst-9919c4562d434f4c_bg.wasm"];
    app = app.route(routes[0],  get(async || wasm(read_file("src/wasm/kerst.wasm".to_owned()).await)));
    app
}

fn add_others(mut app: Router) -> Router{
    let routes = vec!["/robots.txt"];
    for route in routes{
        app = app.route(route,  get(async || read_file("src".to_owned() + route).await));
    }
    app
}

fn rustls_server_config(key: impl AsRef<Path>, cert: impl AsRef<Path>) -> Arc<ServerConfig>{
    let mut key_reader = BufReader::new(File::open(key).unwrap());
    let mut cert_reader = BufReader::new(File::open(cert).unwrap());

    let key = PrivateKey(pkcs8_private_keys(&mut key_reader).unwrap().remove(0));
    let certs = certs(&mut cert_reader).unwrap()
        .into_iter().map(Certificate).collect();
    
    let mut config = ServerConfig::builder()
        .with_safe_defaults()
        .with_no_client_auth()
        .with_single_cert(certs, key)
        .expect("bad certificate/key");
    
    config.alpn_protocols = vec![b"h2".to_vec(), b"http/1.1".to_vec()];

    Arc::new(config)
}

#[tokio::main]
async fn main() {
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "example_tls_rustls=debug".into())
        ).with(tracing_subscriber::fmt::layer())
        .init();
    
    #[cfg(target_arch="aarch64")]
    let rustls_config = rustls_server_config(
        PathBuf::from("/etc/letsencrypt/live/michaeljoy.nl/privkey.pem"),
        PathBuf::from("/etc/letsencrypt/live/michaeljoy.nl/fullchain.pem")
    );

    #[cfg(not(target_arch="aarch64"))]
    let rustls_config = rustls_server_config(
        PathBuf::from("../michaeljoy_certificates/key.pem"),
        PathBuf::from("../michaeljoy_certificates/certificate.pem")
    );

    let acceptor = TlsAcceptor::from(rustls_config);
    let listener = TcpListener::bind("0.0.0.0:80").await.unwrap();
    let mut listener = AddrIncoming::from_listener(listener).unwrap();

    let protocol = Arc::new(Http::new());

    // build the application
    let mut app = Router::new();
    app = add_html_pages(app);
    app = add_css(app);
    app = add_images(app);
    app = add_videos(app);
    app = add_pdf(app);
    app = add_js(app);
    app = add_wasm(app);
    app = add_others(app);

    let mut app = app.into_make_service_with_connect_info::<SocketAddr>();

    loop{
        let stream = poll_fn(|cx| Pin::new(&mut listener).poll_accept(cx))
            .await.unwrap().unwrap();
        
        let acceptor = acceptor.clone();
        let protocol = protocol.clone();
        let svc = app.make_service(&stream);

        tokio::spawn(async move{
            if let Ok(stream) = acceptor.accept(stream).await{
                let _ = protocol.serve_connection(stream, svc.await.unwrap()).await;
            }
        });
    }
}
