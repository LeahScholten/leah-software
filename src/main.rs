#![warn(clippy::pedantic, clippy::nursery)]
#![feature(async_closure)]

use std::net::SocketAddr;

use axum::{Router, routing::get, response::Html};
use tokio::fs::read_to_string;

fn add_html_pages(mut app: Router) -> Router{
    let routes = vec!["/", "/zakelijk.html", "/technisch.html", "/algemeen.html", "/christmas.html"];
    for route in routes{
        if route == "/"{
            app = app.route(route,  get(async || Html(read_to_string("src/html/index.html").await.unwrap_or_else(|error| error.to_string()))));
        }else{
            app = app.route(route,  get(async || Html(read_to_string("src/html".to_owned() + route).await.unwrap_or_else(|error| error.to_string()))));
        }
    }
    app
}

fn add_javascript(mut app: Router) -> Router{
    let routes = vec!["/countdown.js"];
    for route in routes{
        app = app.route(route,  get(async || read_to_string("src/js".to_owned() + route).await.unwrap_or_else(|error| error.to_string())));
    }
    app
}

fn add_css(mut app: Router) -> Router{
    let routes = vec!["/standard.css"];
    for route in routes{
        app = app.route(route,  get(async || read_to_string("src/css".to_owned() + route).await.unwrap_or_else(|error| error.to_string())));
    }
    app
}

fn add_images(mut app: Router) -> Router{
    let routes = vec!["/favicon.ico"];
    for route in routes{
        app = app.route(route,  get(async || read_to_string("src/img".to_owned() + route).await.unwrap_or_else(|error| error.to_string())));
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
        app = app.route(route,  get(async || read_to_string("src/video".to_owned() + route).await.unwrap_or_else(|error| error.to_string())));
    }
    app
}

fn add_pdf(mut app: Router) -> Router{
    let routes = vec!["/cv.pdf"];
    for route in routes{
        app = app.route(route,  get(async || read_to_string("src/pdf".to_owned() + route).await.unwrap_or_else(|error| error.to_string())));
    }
    app
}

/*
    "/robots.txt":"src/robots.txt",

    "/cv.pdf":"src/pdf/cv.pdf",
*/
fn add_others(mut app: Router) -> Router{
    let routes = vec!["/robots.txt"];
    for route in routes{
        app = app.route(route,  get(async || read_to_string("src".to_owned() + route).await.unwrap_or_else(|error| error.to_string())));
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
