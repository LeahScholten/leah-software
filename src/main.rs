#![deny(clippy::panic, clippy::panic_in_result_fn,clippy::panicking_unwrap,
    non_fmt_panics, unconditional_panic, unsafe_code)]
#![warn(clippy::pedantic, clippy::nursery)]
#![feature(async_closure)]

use std::net::SocketAddr;

use axum::{Router, routing::get, response::{Html, IntoResponse}, http::{HeaderMap, header}};
use tokio::fs::read;
use chrono::prelude::*;

async fn read_file(filename: String) -> Vec<u8>{
    read(filename).await.unwrap_or_else(|error| error.to_string().bytes().collect())
}

fn christmas() -> String{
    let now = chrono::Local::now();
    let first_christmas_day = chrono::Local.with_ymd_and_hms(2022, 12, 25, 0, 0, 0).unwrap();
    let second_christmas_day = chrono::Local.with_ymd_and_hms(2022, 12, 26, 0, 0, 0).unwrap();
    let next_year = chrono::Local.with_ymd_and_hms(2023, 1, 1, 0, 0, 0).unwrap();
    let mut page = r##"<!DOCTYPE html>
<html lang="nl">
    <head>
        <title>Merry Christmas and a happy new year</title>
        <link rel="stylesheet" href="/standard.css">
    </head>
    <body>
        <div id="content">
            <h1>Merry Christmas and a happy new year!</h1>
            <p>Als je dit leest, wil ik zeggen dat je een plek in mijn hart hebt.<br/>
                Dit is omdat je een goede vriend of naast familielid bent.<br/>
                Daarom wens ik je een fijne kerst en een gelukkig en gezond nieuw jaar.<br/>
            <strong>"##.to_owned();

    if now < first_christmas_day{
        page += &format!("Dagen tot eerste kerstdag: {}<br/>", (first_christmas_day - now).num_days() + 1);
    }
    if now < second_christmas_day{
        page += &format!("Dagen tot tweede kerstdag: {}<br/>", (second_christmas_day - now).num_days() + 1);
    }
    if (next_year - now).num_days() > 0{
        page += &format!("Dagen tot 2023: {}<br/>", (next_year - now).num_days() + 1);
    }else if now < next_year{
        let time_till_next_year = next_year - now;
        page += &format!("{}:{}:{} tot 2023<br/>", time_till_next_year.num_hours(), time_till_next_year.num_minutes(), time_till_next_year.num_seconds());
    }else{
        page += &format!("Gelukkig 2023!<br/>");
    }
    page += r"</strong>
            <p>groetjes,</p>
            <p>Michael Scholten en Joy</p>
        </div>
    </body>
</html>";

    page
}

fn css(content: Vec<u8>) -> impl IntoResponse{
    let mut headers = HeaderMap::new();
    headers.insert(header::CONTENT_TYPE, "text/css".parse().unwrap());
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
    let routes = vec!["/", "/zakelijk.html", "/technisch.html", "/algemeen.html", "/kerst.html"];
    for route in routes{
        if route == "/"{
            app = app.route(route,  get(async || Html(read_file("src/html/index.html".to_owned()).await)));
        }else if route == "/kerst.html"{
            app = app.route(route,  get(async || Html(christmas())));
        }else{
            app = app.route(route,  get(async || Html(read_file("src/html".to_owned() + route).await)));
        }
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
    app = add_css(app);
    app = add_images(app);
    app = add_videos(app);
    app = add_pdf(app);
    app = add_others(app);

    let address = SocketAddr::from(([0, 0, 0, 0], 8000));
    axum::Server::bind(&address)
        .serve(app.into_make_service()).await.unwrap();
}
