#![warn(clippy::pedantic, clippy::nursery)]
#![feature(async_closure)]

use std::net::SocketAddr;

use axum::{Router, routing::get, response::{Html, IntoResponse}, http::{HeaderMap, header}};
use tokio::fs::read;

async fn read_file(filename: String) -> Vec<u8>{
    read(filename).await.unwrap_or_else(|error| error.to_string().bytes().collect())
}

fn css(content: Vec<u8>) -> impl IntoResponse{
    let mut headers = HeaderMap::new();
    headers.insert(header::CONTENT_TYPE, "text/css".parse().unwrap());
    (headers, content)
}

fn javascript(content: Vec<u8>) -> impl IntoResponse{
    let mut headers = HeaderMap::new();
    headers.insert(header::CONTENT_TYPE, "text/javascript".parse().unwrap());
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

fn add_html_pages(mut app: Router) -> Router{
    let routes = vec!["/", "/zakelijk.html", "/technisch.html", "/algemeen.html", "/christmas.html"];
    for route in routes{
        if route == "/"{
            app = app.route(route,  get(async || Html(read_file("src/html/index.html".to_owned()).await)));
        }else{
            app = app.route(route,  get(async || Html(read_file("src/html".to_owned() + route).await)));
        }
    }
    app
}

fn add_javascript(mut app: Router) -> Router{
    let routes = vec!["/countdown.js"];
    for route in routes{
        app = app.route(route,  get(async || javascript(read_file("src/js".to_owned() + route).await)));
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

fn add_others(mut app: Router) -> Router{
    let routes = vec!["/robots.txt"];
    for route in routes{
        app = app.route(route,  get(async || read_file("src".to_owned() + route).await));
    }
    app
}

#[tokio::main]
async fn main() {
    // build the application
    let mut app = Router::new();
    app = add_html_pages(app);
    app = add_javascript(app);
    app = add_css(app);
    app = add_images(app);
    app = add_videos(app);
    app = add_pdf(app);
    app = add_others(app);

    let address = SocketAddr::from(([0, 0, 0, 0], 8000));
    axum::Server::bind(&address)
        .serve(app.into_make_service()).await.unwrap();
}
