#![deny(
    clippy::panic,
    clippy::panic_in_result_fn,
    clippy::panicking_unwrap,
    clippy::unwrap_in_result,
    non_fmt_panics,
    unconditional_panic,
    unsafe_code
)]
#![warn(clippy::pedantic, clippy::nursery)]

use std::io;
use std::net::SocketAddr;
use std::sync::Arc;
use std::time::SystemTime;

use http_body_util::{BodyExt, Full};
use hyper::{
    body::{Body, Bytes},
    header::{self, HeaderValue},
    service::service_fn,
    Method, Request, Response, StatusCode,
};
use hyper_util::{
    rt::{TokioExecutor, TokioIo},
    server::conn::auto::Builder,
};
use rustls::{
    pki_types::{CertificateDer, PrivateKeyDer},
    ServerConfig,
};
use std::{
    fmt::Write as _,
    fs,
    net::Ipv4Addr,
    num::ParseFloatError,
    sync::atomic::{AtomicU8, Ordering},
};
use tokio::{
    fs as tokio_fs,
    io::{AsyncBufRead, AsyncBufReadExt},
    net::TcpListener,
};
use tokio_rustls::TlsAcceptor;

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

fn generate_temperature_page() -> String {
    let mut page = "<!DOCTYPE html><html><head><title>temperature</title></head><body>".to_owned();
    match fs::read_to_string("/sys/class/thermal/thermal_zone0/temp")
        .map(|temperature| Ok::<f32, ParseFloatError>(temperature.trim().parse::<f32>()? / 1000.0))
    {
        Ok(Ok(temperature)) => {
            write!(&mut page, "Temperature: {temperature} degrees Celsius").unwrap();
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

async fn michaeljoy<Req: Body + Send>(
    req: Request<Req>,
) -> Result<Response<Full<Bytes>>, hyper::Error>
where
    Req::Data: Send,
{
    static MOOD: AtomicU8 = AtomicU8::new(70);
    // Create an empty response
    let mut response = Response::new(Full::new(Bytes::new()));

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

    #[allow(clippy::unwrap_used)]
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
            *response.body_mut() = generate_temperature_page().into();
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

async fn last_modification_time() -> Option<SystemTime> {
    tokio_fs::File::open(CERT_KEY.0)
        .await
        .ok()?
        .metadata()
        .await
        .ok()?
        .modified()
        .ok()
}

#[tokio::main]
async fn main() {
    let port = 4430;
    let address: SocketAddr = (Ipv4Addr::UNSPECIFIED, port).into();
    #[allow(clippy::unwrap_used)]
    let incoming = TcpListener::bind(address).await.unwrap();
    loop {
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

        let Ok(server_config) = ServerConfig::builder()
            .with_no_client_auth()
            .with_single_cert(certs, key)
            .map(|mut acceptor| {
                acceptor.alpn_protocols =
                    vec![b"h2".to_vec(), b"http/1.1".to_vec(), b"http/1.0".to_vec()];
                acceptor
            })
        else {
            eprintln!("Failed to create acceptor");
            continue;
        };

        let acceptor = TlsAcceptor::from(Arc::new(server_config));
        let service = service_fn(michaeljoy);

        let Some(used_update) = last_modification_time().await else {
            continue;
        };

        eprintln!("Listening on {address}");
        loop {
            let Ok((tcp_stream, _remote_address)) = incoming.accept().await else {
                continue;
            };
            let tls_acceptor = acceptor.clone();
            tokio::spawn(async move {
                let tls_stream = match tls_acceptor.accept(tcp_stream).await {
                    Ok(tls_stream) => tls_stream,
                    Err(err) => {
                        eprintln!("Failed to perform tls handshake: {err:?}");
                        return;
                    }
                };
                if let Err(err) = Builder::new(TokioExecutor::new())
                    .serve_connection(TokioIo::new(tls_stream), service)
                    .await
                {
                    eprintln!("Failed to serve connection: {err:#}");
                }
            });

            let Some(last_update) = last_modification_time().await else {
                continue;
            };

            if last_update > used_update {
                break;
            }
        }
    }
}

/// Load public certificate from file
fn load_certs(filename: &str) -> io::Result<Vec<CertificateDer<'static>>> {
    // Open certificate file.
    let certfile = fs::File::open(filename)?;
    let mut reader = io::BufReader::new(certfile);

    // Load and return certificate.
    rustls_pemfile::certs(&mut reader).collect()
}

/// Load private key from file
#[allow(clippy::unwrap_used)]
fn load_private_key(filename: &str) -> io::Result<PrivateKeyDer<'static>> {
    // Open keyfile.
    let keyfile = fs::File::open(filename)?;
    let mut reader = io::BufReader::new(keyfile);

    // Load and return a single private key.
    rustls_pemfile::private_key(&mut reader).map(|key| key.unwrap())
}
