#![warn(clippy::pedantic, clippy::nursery)]
use warp::{Filter, fs};

#[tokio::main]
async fn main() {
    // html filepaths
    let routes =  warp::path::end()
        .and(fs::file("src/html/index.html"));
    let routes = routes.or(warp::path("algemeen.html")
        .and(fs::file("src/html/algemeen.html")));
    let routes = routes.or(warp::path("technisch.html")
        .and(fs::file("src/html/technisch.html")));
    let routes = routes.or(warp::path("zakelijk.html")
        .and(fs::file("src/html/zakelijk.html")));
    
    // css filepath
    let routes = routes.or(warp::path("standard.css")
        .and(fs::file("src/css/standard.css")));
    
    // script filepath
    let routes = routes.or(warp::path("countdown.js")
        .and(fs::file("src/js/countdown.js")));

    // pdf filepath
    let routes = routes.or(warp::path("cv.pdf")
        .and(fs::file("src/pdf/cv.pdf")));
    
    // image filepaths
    let routes = routes.or(warp::path("favicon.ico")
        .and(fs::file("src/img/favicon.ico")));
    
    // video filepaths
    let routes = routes.or(warp::path!("raspberryPico" / "7segmentCounter.mp4")
        .and(fs::file("src/video/raspberryPico/7SegmentCounter.mp4")));
    let routes = routes.or(warp::path!("raspberryPico" / "binaryAnalogLeds.mp4")
        .and(fs::file("src/video/raspberryPico/binaryAnalogLeds.mp4")));
    let routes = routes.or(warp::path!("raspberryPico" / "binaryLedCounter.mp4")
        .and(fs::file("src/video/raspberryPico/binaryLedCounter.mp4")));
    
    let routes = routes.or(warp::path!("ZUMO32U4" / "objectTracing.mp4")
        .and(fs::file("src/video/ZUMO32U4/objectTracing.mp4")));
    let routes = routes.or(warp::path!("ZUMO32U4" / "rotatingInPlace.mp4")
        .and(fs::file("src/video/ZUMO32U4/rotatingInPlace.mp4")));
    
    let routes = routes.or(warp::path!("ATmega328P" / "hapticWire.mp4")
        .and(fs::file("src/video/ATmega328P/hapticWire.mp4")));
    let routes = routes.or(warp::path!("ATmega328P" / "lightDensityMeter.mp4")
        .and(fs::file("src/video/ATmega328P/lightDensityMeter.mp4")));
    let routes = routes.or(warp::path!("ATmega328P" / "quadWalkingLightShow.mp4")
        .and(fs::file("src/video/ATmega328P/quadWalkingLightShow.mp4")));
    let routes = routes.or(warp::path!("ATmega328P" / "rgbTraficLight.mp4")
        .and(fs::file("src/video/ATmega328P/rgbTraficLight.mp4")));
    let routes = routes.or(warp::path!("ATmega328P" / "walkingLight.mp4")
        .and(fs::file("src/video/ATmega328P/walkingLight.mp4")));

    let routes = routes.or(warp::path!("Arduino" / "automaticLight.mp4")
        .and(fs::file("src/video/Arduino/automaticLight.mp4")));
    let routes = routes.or(warp::path!("raspberryPico" / "7segmentCounter.mp4")
        .and(fs::file("src/video/Arduino/rgbLightShow.mp4")));

    let routes = warp::get().and(routes);

    println!("Listening on port: 8000");
    warp::serve(routes).run(([0, 0, 0, 0], 8000)).await;
}
