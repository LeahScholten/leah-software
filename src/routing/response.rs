use axum::response::{IntoResponse, Html};
use hyper::{header, HeaderMap};

pub trait TupleStruct<T>{
    fn new(value: T) -> Self;
}

impl<T> TupleStruct<T> for Html<T>{
    fn new(value: T) -> Self {
        Self(value)
    }
}

pub struct Css(pub Vec<u8>);

impl TupleStruct<Vec<u8>> for Css{
    fn new(value: Vec<u8>) -> Self {
        Self(value)
    }
}

impl IntoResponse for Css{
    fn into_response(self) -> axum::response::Response {
        // Create a new header
        let mut headers = HeaderMap::new();

        // Set the content type to css
        headers.insert(header::CONTENT_TYPE, "text/css".parse().unwrap());

        // Return the headers with the content
        (headers, self.0).into_response()
    }
}

pub struct Image(pub Vec<u8>);

impl TupleStruct<Vec<u8>> for Image{
    fn new(value: Vec<u8>) -> Self {
        Self(value)
    }
}

impl IntoResponse for Image{
    fn into_response(self) -> axum::response::Response {
        // Create a new header
        let mut headers = HeaderMap::new();

        // Set the content type to image
        headers.insert(header::CONTENT_TYPE, "image/*".parse().unwrap());

        // Return the headers with the content
        (headers, self.0).into_response()
    }
}

pub struct Pdf(pub Vec<u8>);

impl TupleStruct<Vec<u8>> for Pdf{
    fn new(value: Vec<u8>) -> Self {
        Self(value)
    }
}

impl IntoResponse for Pdf{
    fn into_response(self) -> axum::response::Response {
        // Create a new header
        let mut headers = HeaderMap::new();

        // Set the content type to pdf
        headers.insert(header::CONTENT_TYPE, "application/pdf".parse().unwrap());

        // Return the header with the content
        (headers, self.0).into_response()
    }
}

pub struct Js(pub Vec<u8>);

impl TupleStruct<Vec<u8>> for Js{
    fn new(value: Vec<u8>) -> Self {
        Self(value)
    }
}

impl IntoResponse for Js{
    fn into_response(self) -> axum::response::Response {
        // Create a new header
        let mut headers = HeaderMap::new();

        // Set the content type to JavaScript
        headers.insert(
            header::CONTENT_TYPE,
            "application/javascript".parse().unwrap(),
        );

        // Return the header with the content
        (headers, self.0).into_response()
    }
}

pub struct Zip(pub Vec<u8>);

impl TupleStruct<Vec<u8>> for Zip{
    fn new(value: Vec<u8>) -> Self {
        Self(value)
    }
}

impl IntoResponse for Zip{
    fn into_response(self) -> axum::response::Response {
        // Create a new header
        let mut headers = HeaderMap::new();

        // Set the content type to zip
        headers.insert(header::CONTENT_TYPE, "application/zip".parse().unwrap());

        // Return the header with the content
        (headers, self.0).into_response()
    }
}

pub struct Mp4(pub Vec<u8>);

impl TupleStruct<Vec<u8>> for Mp4{
    fn new(value: Vec<u8>) -> Self {
        Self(value)
    }
}

impl IntoResponse for Mp4{
    fn into_response(self) -> axum::response::Response {
        // Create a new header
        let mut headers = HeaderMap::new();

        // Set the content type to zip
        headers.insert(header::CONTENT_TYPE, "video/mp4".parse().unwrap());

        // Return the header with the content
        (headers, self.0).into_response()
    }
}

pub struct Txt(pub Vec<u8>);

impl TupleStruct<Vec<u8>> for Txt{
    fn new(value: Vec<u8>) -> Self {
        Self(value)
    }
}

impl IntoResponse for Txt{
    fn into_response(self) -> axum::response::Response {
        // Create a new header
        let mut headers = HeaderMap::new();

        // Set the content type to zip
        headers.insert(header::CONTENT_TYPE, "text/plain".parse().unwrap());

        // Return the header with the content
        (headers, self.0).into_response()
    }
}
