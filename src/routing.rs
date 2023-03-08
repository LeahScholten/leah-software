use axum::{response::Html, routing::get, Router};
use tokio::fs::read;

use self::response::{css, image, js, pdf, wasm, zip};

mod response;

async fn read_file(filename: String) -> Vec<u8> {
    // Read the file as bytes, return the error message as bytes if it fails
    read(filename)
        .await
        .unwrap_or_else(|error| error.to_string().bytes().collect())
}

pub fn add_html_pages(mut app: Router) -> Router {
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
        get(async || Html(read_file("src/html/index.html".to_owned()).await)),
    );

    // Add the other routes
    for &route in routes.iter().skip(1) {
        app = app.route(
            route,
            get(async || Html(read_file("src/html".to_owned() + route).await)),
        );
    }
    app
}

pub fn add_css(mut app: Router) -> Router {
    // Set the routes for css
    let routes = ["/standard.css"];

    // Add the routes
    for route in routes {
        app = app.route(
            route,
            get(async || css(read_file("src/css".to_owned() + route).await)),
        );
    }
    app
}

pub fn add_images(mut app: Router) -> Router {
    // Set the routes to images
    let routes = ["/favicon.ico"];

    // Add the routes
    for route in routes {
        app = app.route(
            route,
            get(async || image(read_file("src/img".to_owned() + route).await)),
        );
    }
    app
}

pub fn add_videos(mut app: Router) -> Router {
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
            get(async || read_file("src/video".to_owned() + route).await),
        );
    }
    app
}

pub fn add_pdf(mut app: Router) -> Router {
    // Set the routes for pdf files
    let routes = ["/cv.pdf"];

    // Add the routes
    for route in routes {
        app = app.route(
            route,
            get(async || pdf(read_file("src/pdf".to_owned() + route).await)),
        );
    }
    app
}

pub fn add_js(mut app: Router) -> Router {
    // Set the routes for JavaScript
    let routes = ["/kerst-9919c4562d434f4c.js"];

    // Add the routes
    app = app.route(
        routes[0],
        get(async || js(read_file("src/js/kerst.js".to_owned()).await)),
    );
    app
}

pub fn add_wasm(mut app: Router) -> Router {
    // Set the routes to wasm files
    let routes = ["/kerst-9919c4562d434f4c_bg.wasm"];

    // Add the routes
    app = app.route(
        routes[0],
        get(async || wasm(read_file("src/wasm/kerst.wasm".to_owned()).await)),
    );
    app
}

pub fn add_games(mut app: Router) -> Router {
    // Set the names of the games
    let games = ["pong"];

    // Add a zip-file for Windows and Linux for every game
    for game in games {
        let route = format!("/games/{game}/windows.zip");
        app = app.route(
            &route,
            get(async move || zip(read_file(format!("src/games/{game}/windows.zip")).await)),
        );
        let route = format!("/games/{game}/linux.zip");
        app = app.route(
            &route,
            get(async move || zip(read_file(format!("src/games/{game}/linux.zip")).await)),
        );
    }
    app
}

pub fn add_others(mut app: Router) -> Router {
    // Set the other routes
    let routes = ["/robots.txt"];

    // Add the routes
    for route in routes {
        app = app.route(
            route,
            get(async || read_file("src".to_owned() + route).await),
        );
    }
    app
}
