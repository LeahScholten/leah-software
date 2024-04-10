let thermometer;

function preload(){
    thermometer = loadImage("/thermometer.png");
}

function setup(){
    createCanvas(512, 1024);
    thermometer.resize(512, 0);
    image(thermometer, 0, 0);
    // Min 550
    // Max 50
    let y = httpGet("/mood", "text").then((value) => {
        let y = 50 + 5 * Number(value);
        line(0, y, 100, y);
    });
    console.log(y);
}
