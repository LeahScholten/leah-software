#![deny(
    clippy::panic,
    clippy::panic_in_result_fn,
    clippy::panicking_unwrap,
    clippy::unwrap_in_result,
    clippy::unwrap_in_result,
    clippy::unwrap_used,
    non_fmt_panics,
    unconditional_panic,
    unsafe_code
)]
#![warn(clippy::pedantic, clippy::nursery)]
#![feature(async_closure)]

use hyper::{
    body::HttpBody,
    header::{self, HeaderValue},
    server::conn::AddrIncoming,
    service::{make_service_fn, service_fn},
    Body, Method, Request, Response, Server, StatusCode,
};
use hyper_rustls::TlsAcceptor;
use std::{
    fmt::Write as _,
    fs, io,
    net::{Ipv4Addr, SocketAddr},
    num::ParseFloatError,
    sync::atomic::{AtomicU8, Ordering},
    time::{Duration, SystemTime},
};
use tokio::{
    fs as tokio_fs,
    io::{AsyncBufRead, AsyncBufReadExt},
    select,
};

/// Test key and certificate
#[cfg(target_arch = "x86_64")]
const CERT_KEY: (&str, &str) = (
    "../michaeljoy_certificates/certificate.pem",
    "../michaeljoy_certificates/key.pem",
);

#[cfg(not(target_arch = "x86_64"))]
const CERT_KEY: (&str, &str) = (
    "/etc/letsencrypt/live/michaeljoy.nl/fullchain.pem",
    "/etc/letsencrypt/live/michaeljoy.nl/privkey.pem",
);

fn error(err: impl std::fmt::Display) -> io::Error {
    io::Error::new(io::ErrorKind::Other, err.to_string())
}

async fn find_path<T: AsyncBufRead + Unpin + Send>(
    mut lines: tokio::io::Lines<T>,
    expected_uri: &str,
) -> Option<String> {
    // Look for the file corresponding to the requested path
    let mut path = None;
    while let Ok(Some(line)) = lines.next_line().await {
        // Split the line in sections
        let mut sections = line.split(',');

        // Take the uri part, skip this line if there is none
        let Some(uri) = sections.next().map(str::trim) else {
            continue;
        };

        // Skip this line if the uri wasn't the expected uri
        if uri != expected_uri {
            continue;
        }

        // Take the path corresponding to the uri
        path = sections.next().map(|path| path.trim().to_owned());

        // If there was a path, break out of the loop
        if path.is_some() {
            break;
        }
    }
    path
}

async fn generate_temperature_page() -> String {
    let mut page = "<!DOCTYPE html><html><head><title>temperature</title></head><body>".to_owned();
    match fs::read_to_string("/sys/class/thermal/thermal_zone0/temp")
        .map(|temperature| Ok::<f32, ParseFloatError>(temperature.trim().parse::<f32>()? / 1000.0))
    {
        Ok(Ok(temperature)) => {
            write!(&mut page, "Temperature: {temperature} degrees Celsius").unwrap()
        }
        Ok(Err(e)) => write!(&mut page, "Failed to parse temperature: {e:?}").unwrap(),
        Err(e) => write!(&mut page, "Failed to read temperature: {e:?}").unwrap(),
    }
    page += "</body></html>";
    page
}

fn content_type(file_name: &str) -> &'static str {
    let file_name = file_name.trim();
    match &file_name[file_name.len() - 3..] {
        "tml" => "text/html",
        "css" => "text/css",
        "ico" => "image/x-icon",
        "peg" => "image/jpeg",
        "png" => "image/png",
        "mp4" => "video/mp4",
        "pdf" => "application/pdf",
        ".js" => "application/javascript",
        "zip" => "application/zip",
        "txt" => "text/plain",
        _ => todo!(),
    }
}

async fn michaeljoy(req: Request<Body>) -> Result<Response<Body>, hyper::Error> {
    static MOOD: AtomicU8 = AtomicU8::new(70);
    // Create an empty response
    let mut response = Response::new(Body::empty());

    // Try to open the paths file, return an internal server error on failure
    let Ok(paths_file) = tokio_fs::File::open("files.csv").await else {
        *response.status_mut() = StatusCode::INTERNAL_SERVER_ERROR;
        *response.body_mut() = "Failed to load paths".into();
        return Ok(response);
    };

    // Create a buffer for reading the file
    let lines = tokio::io::BufReader::new(paths_file).lines();

    let now = chrono::Local::now();

    // Take the requested path
    let expected_uri = req.uri().path();
    println!("{now}\n{}\n{expected_uri}\n", req.method());

    match find_path(lines, expected_uri).await {
        // Otherwise, try to read the file
        Some(path) => match tokio_fs::read(&path).await {
            // If the file was read successfully
            Ok(content) => {
                // Set the body to the content of the file, use the accepted status code
                *response.body_mut() = content.into();
                *response.status_mut() = StatusCode::OK;
                response.headers_mut().append(
                    header::CONTENT_TYPE,
                    HeaderValue::from_str(content_type(&path)).unwrap(),
                );
            }
            // Otherwise
            Err(e) => {
                // Set the body to the error message with the request to send it to me
                // Set the status code to internal server error
                *response.body_mut() =
                    ("Send the following error to michael-scholten@hotmail.nl<br/>".to_owned()
                        + &e.to_string())
                        .into();
                *response.status_mut() = StatusCode::INTERNAL_SERVER_ERROR;
                response.headers_mut().append(
                    header::CONTENT_TYPE,
                    HeaderValue::from_str(content_type(".html")).unwrap(),
                );
            }
        },

        None if expected_uri == "/temperature.html" => {
            *response.body_mut() = generate_temperature_page().await.into();
            *response.status_mut() = StatusCode::OK;
            response.headers_mut().append(
                header::CONTENT_TYPE,
                HeaderValue::from_str(content_type("temperature.html")).unwrap(),
            );
        }

        None if expected_uri == "/mood" => match *req.method() {
            Method::GET => {
                *response.body_mut() = MOOD.load(Ordering::Relaxed).to_string().into();
            }
            Method::POST => {
                let data = req.into_body().collect().await.map(|body| {
                    String::from_utf8(body.to_bytes().into_iter().collect())
                        .map(|body| body.trim().parse::<u8>())
                });
                if let Ok(Ok(Ok(value))) = data {
                    if value <= 100 {
                        MOOD.store(value, Ordering::Relaxed);
                    }
                }
            }
            _ => {}
        },

        // If the requested page wasn't found
        None => {
            // Send a 404 page with the NOT FOUND status code
            *response.body_mut() = "<h1>404 page not found!</h1>".into();
            *response.status_mut() = StatusCode::NOT_FOUND;
            response.headers_mut().append(
                header::CONTENT_TYPE,
                HeaderValue::from_str(content_type("error.html")).unwrap(),
            );
        }
    }

    Ok(response)
}

async fn http(req: Request<Body>) -> hyper::Result<Response<Body>> {
    let mut response = Response::new(Body::empty());
    *response.status_mut() = StatusCode::PERMANENT_REDIRECT;
    if let Ok(header_value) = format!(
        "https://{}/{}",
        req.uri().host().unwrap_or("127.0.0.1:4430"),
        req.uri().path()
    )
    .parse()
    {
        response.headers_mut().append("Location", header_value);
    }
    Ok(response)
}

async fn wait_for_cert_update() {
    let start = SystemTime::now();
    let second = Duration::from_secs(1);
    loop {
        tokio::time::sleep(second).await;
        let Ok(file) = tokio_fs::File::open(CERT_KEY.0).await else {
            continue;
        };
        let Ok(metadata) = file.metadata().await else {
            continue;
        };
        if metadata.modified().is_ok_and(|update| update > start) {
            break;
        }
    }
}

#[tokio::main]
async fn main() {
    let port = 4430;
    let address: SocketAddr = (Ipv4Addr::new(0, 0, 0, 0), port).into();
    loop {
        let https_server = {
            // Load public certificate
            let Ok(certs) = load_certs(CERT_KEY.0) else {
                eprintln!("Failed to load certificates");
                continue;
            };

            // Load private key
            let Ok(key) = load_private_key(CERT_KEY.1) else {
                eprintln!("Failed to load keys");
                continue;
            };

            // Build TLS configuration

            // Create a TCP listener via tokio
            let Ok(incoming) = AddrIncoming::bind(&address) else {
                eprintln!("Failed to bind to {address}");
                continue;
            };
            let Ok(acceptor) = TlsAcceptor::builder()
                .with_single_cert(certs, key)
                .map_err(error)
                .map(|acceptor| acceptor.with_all_versions_alpn().with_incoming(incoming))
            else {
                eprintln!("Failed to create acceptor");
                continue;
            };
            let service =
                make_service_fn(|_| async move { Ok::<_, io::Error>(service_fn(michaeljoy)) });
            Server::builder(acceptor).serve(service)
        };

        let http_redirect = {
            let address: SocketAddr = (Ipv4Addr::new(0, 0, 0, 0), 8080).into();
            let Ok(incoming) = AddrIncoming::bind(&address) else {
                eprintln!("Failed to bind http server to {address}");
                continue;
            };
            let service = make_service_fn(|_| async { Ok::<_, io::Error>(service_fn(http)) });
            Server::builder(incoming).serve(service)
        };

        // Run the future, keep going until an error occurs
        eprintln!("Starting to serve on https://{address}");
        #[allow(clippy::redundant_pub_crate)]
        {
            select! {_ = http_redirect => (), _ = https_server => (), () = wait_for_cert_update() => ()};
        }
    }
}

/// Load public certificate from file
fn load_certs(filename: &str) -> io::Result<Vec<rustls::Certificate>> {
    // Open certificate file
    let certificate =
        fs::File::open(filename).map_err(|e| error(format!("Failed to open {filename}:{e}")))?;
    let mut reader = io::BufReader::new(certificate);

    // Load and return certificate
    let certs =
        rustls_pemfile::certs(&mut reader).map_err(|_| error("Failed to load certificate"))?;

    Ok(certs.into_iter().map(rustls::Certificate).collect())
}

/// Load private key from file
fn load_private_key(filename: &str) -> io::Result<rustls::PrivateKey> {
    // Open keyfile
    let keyfile =
        fs::File::open(filename).map_err(|e| error(format!("Failed to open {filename}: {e}")))?;
    let mut reader = io::BufReader::new(keyfile);

    // Load and return a single private key
    let keys = rustls_pemfile::pkcs8_private_keys(&mut reader)
        .map_err(|_| error("Failed to load private key"))?;
    if keys.is_empty() {
        return Err(error("Expected atleast 1 private key"));
    }

    Ok(rustls::PrivateKey(keys[0].clone()))
}
