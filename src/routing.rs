use axum::{response::Html, routing::get, Router};
use tokio::fs::read;

use self::response::{css, image, js, pdf, wasm, zip};

mod response;

async fn read_file(filename: String) -> Vec<u8> {
    read(filename)
        .await
        .unwrap_or_else(|error| error.to_string().bytes().collect())
}

pub fn add_html_pages(mut app: Router) -> Router {
    let routes = [
        "/",
        "/zakelijk.html",
        "/technisch.html",
        "/algemeen.html",
        "/kerst.html",
    ];
    for route in routes {
        if route == "/" {
            app = app.route(
                route,
                get(async || Html(read_file("src/html/index.html".to_owned()).await)),
            );
        } else {
            app = app.route(
                route,
                get(async || Html(read_file("src/html".to_owned() + route).await)),
            );
        }
    }
    app
}

pub fn add_css(mut app: Router) -> Router {
    let routes = ["/standard.css"];
    for route in routes {
        app = app.route(
            route,
            get(async || css(read_file("src/css".to_owned() + route).await)),
        );
    }
    app
}

pub fn add_images(mut app: Router) -> Router {
    let routes = ["/favicon.ico"];
    for route in routes {
        app = app.route(
            route,
            get(async || image(read_file("src/img".to_owned() + route).await)),
        );
    }
    app
}

pub fn add_videos(mut app: Router) -> Router {
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
    for route in routes {
        app = app.route(
            route,
            get(async || read_file("src/video".to_owned() + route).await),
        );
    }
    app
}

pub fn add_pdf(mut app: Router) -> Router {
    let routes = ["/cv.pdf"];
    for route in routes {
        app = app.route(
            route,
            get(async || pdf(read_file("src/pdf".to_owned() + route).await)),
        );
    }
    app
}

pub fn add_js(mut app: Router) -> Router {
    let routes = ["/kerst-9919c4562d434f4c.js"];
    app = app.route(
        routes[0],
        get(async || js(read_file("src/js/kerst.js".to_owned()).await)),
    );
    app
}

pub fn add_wasm(mut app: Router) -> Router {
    let routes = ["/kerst-9919c4562d434f4c_bg.wasm"];
    app = app.route(
        routes[0],
        get(async || wasm(read_file("src/wasm/kerst.wasm".to_owned()).await)),
    );
    app
}

pub fn add_games(mut app: Router) -> Router {
    let games = ["pong"];
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
    let routes = ["/robots.txt"];
    for route in routes {
        app = app.route(
            route,
            get(async || read_file("src".to_owned() + route).await),
        );
    }
    app
}
