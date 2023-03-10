use axum::response::IntoResponse;
use hyper::{header, HeaderMap};

pub fn css(content: Vec<u8>) -> impl IntoResponse {
    // Create a new header
    let mut headers = HeaderMap::new();

    // Set the content type to css
    headers.insert(header::CONTENT_TYPE, "text/css".parse().unwrap());

    // Return the headers with the content
    (headers, content)
}

pub fn image(content: Vec<u8>) -> impl IntoResponse {
    // Create a new header
    let mut headers = HeaderMap::new();

    // Set the content type to image
    headers.insert(header::CONTENT_TYPE, "image/*".parse().unwrap());

    // Return the headers with the content
    (headers, content)
}

pub fn pdf(content: Vec<u8>) -> impl IntoResponse {
    // Create a new header
    let mut headers = HeaderMap::new();

    // Set the content type to pdf
    headers.insert(header::CONTENT_TYPE, "application/pdf".parse().unwrap());

    // Return the header with the content
    (headers, content)
}

pub fn js(content: Vec<u8>) -> impl IntoResponse {
    // Create a new header
    let mut headers = HeaderMap::new();

    // Set the content type to JavaScript
    headers.insert(
        header::CONTENT_TYPE,
        "application/javascript".parse().unwrap(),
    );

    // Return the header with the content
    (headers, content)
}

pub fn zip(content: Vec<u8>) -> impl IntoResponse {
    // Create a new header
    let mut headers = HeaderMap::new();

    // Set the content type to zip
    headers.insert(header::CONTENT_TYPE, "application/zip".parse().unwrap());

    // Return the header with the content
    (headers, content)
}
