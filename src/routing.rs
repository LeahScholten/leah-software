use axum::{response::{Html, IntoResponse, Response}, routing::get, Router};
use hyper::StatusCode;
use tokio::fs::read;

use self::response::{Css, Image, Js, Pdf, Zip, Mp4, Txt, TupleStruct};

mod response;

async fn read_file<T: IntoResponse + TupleStruct<Vec<u8>>>(filename: String) -> Response {
    // Read the file as bytes, return the error message as bytes if it fails
    match read(filename)
        .await{
        Err(error) =>{
            let mut response = Html(error.to_string()).into_response();
            *response.status_mut() = StatusCode::INTERNAL_SERVER_ERROR;
            response
        },
        Ok(content) => T::new(content).into_response()
    }
}

pub async fn add_html_pages(mut app: Router) -> Router {
    // Set the HTML page routes
    let routes = [
        "/",
        "/zakelijk.html",
        "/technisch.html",
        "/algemeen.html",
        "/kerst.html",
    ];

    // Add a path for the main page
    app = app.route(
        "/",
        get(async || read_file::<Html<Vec<u8>>>("src/html/index.html".to_owned()).await),
    );

    // Add the other routes
    for &route in routes.iter().skip(1) {
        app = app.route(
            route,
            get(async || read_file::<Html<Vec<u8>>>("src/html".to_owned() + route).await),
        );
    }
    app
}

pub async fn add_css(mut app: Router) -> Router {
    // Set the routes for css
    let routes = ["/standard.css"];

    // Add the routes
    for route in routes {
        app = app.route(
            route,
            get(async || read_file::<Css>("src/css".to_owned() + route).await),
        );
    }
    app
}

pub async fn add_images(mut app: Router) -> Router {
    // Set the routes to images
    let routes = ["/favicon.ico"];

    // Add the routes
    for route in routes {
        app = app.route(
            route,
            get(async || read_file::<Image>("src/img".to_owned() + route).await),
        );
    }
    app
}

pub async fn add_videos(mut app: Router) -> Router {
    // Set the routes to the videos
    let routes = [
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
        "/Arduino/rgbLightShow.mp4",
    ];

    // Add the routes
    for route in routes {
        app = app.route(
            route,
            get(async || read_file::<Mp4>("src/video".to_owned() + route).await),
        );
    }
    app
}

pub async fn add_pdf(mut app: Router) -> Router {
    // Set the routes for pdf files
    let routes = ["/cv.pdf"];

    // Add the routes
    for route in routes {
        app = app.route(
            route,
            get(async || read_file::<Pdf>("src/pdf".to_owned() + route).await),
        );
    }
    app
}

pub async fn add_js(mut app: Router) -> Router {
    // Set the routes for JavaScript
    let routes = ["/kerst.js"];

    // Add the routes
    app = app.route(
        routes[0],
        get(async || read_file::<Js>("src/js/kerst.js".to_owned()).await),
    );
    app
}

pub async fn add_games(mut app: Router) -> Router {
    // Set the names of the games
    let games = ["pong", "conwaysGameOfLife"];

    // Add a zip-file for Windows and Linux for every game
    for game in games {
        let route = format!("/games/{game}/windows.zip");
        app = app.route(
            &route,
            get(async move || read_file::<Zip>(format!("src/games/{game}/windows.zip")).await),
        );
        let route = format!("/games/{game}/linux.zip");
        app = app.route(
            &route,
            get(async move || read_file::<Zip>(format!("src/games/{game}/linux.zip")).await),
        );
    }
    app
}

pub async fn add_others(mut app: Router) -> Router {
    // Set the other routes
    let routes = ["/robots.txt"];

    // Add the routes
    for route in routes {
        app = app.route(
            route,
            get(async || read_file::<Txt>("src".to_owned() + route).await),
        );
    }
    app
}
