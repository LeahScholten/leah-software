const picoVideo = ["/raspberryPico/7segmentCounter.mp4", "/raspberryPico/binaryAnalogLeds.mp4", "/raspberryPico/binaryLedCounter.mp4"]
const zumo32u4Video = ["/ZUMO32U4/objectTracing.mp4", "/ZUMO32U4/rotatingInPlace.mp4"]
const atmega328pVideo = ["/ATmega328P/hapticWire.mp4", "/ATmega328P/lightDensityMeter.mp4", "/ATmega328P/quadWalkingLightShow.mp4", "/ATmega328P/rgbTraficLight.mp4",
                         "/ATmega328P/walkingLight.mp4"]
const arduino = ["/Arduino/automaticLight.mp4", "/Arduino/rgbLightShow.mp4"]

const createVideo = (video) => {
    const videoElement = document.createElement("video")
    videoElement.source = video
    return videoElement
}

let pico = 0
let picoView

const appendPicoVideo = () => {
    const video = createVideo(picoVideo[pico++])
    if(pico < picoVideo.length){
        video.onLoad = appendPicoVideo
    }
    picoView.appendChild(video)
    console.log(pico)
}

window.onload = () => {
    picoView = document.getElementById("picoView")
    appendPicoVideo()
}