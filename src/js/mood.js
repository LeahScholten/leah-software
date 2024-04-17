let thermometer;

function preload(){
    thermometer = loadImage("/thermometer.png");
}

function setup(){
    thermometer.resize(512, 0);
    createCanvas(thermometer.width, thermometer.height).parent("thermometer");
    image(thermometer, 0, 0);
    // Min 550
    // Max 50
    let y = httpGet("/mood", "text").then((value) => {
        let y = 50 + 5 * Number(value);
        line(0, y, 100, y);
    });
    console.log(y);
}
