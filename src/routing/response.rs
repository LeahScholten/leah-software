use axum::response::IntoResponse;
use hyper::{header, HeaderMap};

pub fn css(content: Vec<u8>) -> impl IntoResponse {
    let mut headers = HeaderMap::new();
    headers.insert(header::CONTENT_TYPE, "text/css".parse().unwrap());
    (headers, content)
}

pub fn image(content: Vec<u8>) -> impl IntoResponse {
    let mut headers = HeaderMap::new();
    headers.insert(header::CONTENT_TYPE, "image/*".parse().unwrap());
    (headers, content)
}

pub fn pdf(content: Vec<u8>) -> impl IntoResponse {
    let mut headers = HeaderMap::new();
    headers.insert(header::CONTENT_TYPE, "application/pdf".parse().unwrap());
    (headers, content)
}

pub fn js(content: Vec<u8>) -> impl IntoResponse {
    let mut headers = HeaderMap::new();
    headers.insert(
        header::CONTENT_TYPE,
        "application/javascript".parse().unwrap(),
    );
    (headers, content)
}

pub fn wasm(content: Vec<u8>) -> impl IntoResponse {
    let mut headers = HeaderMap::new();
    headers.insert(header::CONTENT_TYPE, "application/wasm".parse().unwrap());
    (headers, content)
}

pub fn zip(content: Vec<u8>) -> impl IntoResponse {
    let mut headers = HeaderMap::new();
    headers.insert(header::CONTENT_TYPE, "application/zip".parse().unwrap());
    (headers, content)
}
