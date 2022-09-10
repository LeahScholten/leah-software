const countdown = () => {
    const eersteKerstdag = new Date(2022, 11, 25)
    const tweedeKerstdag = new Date(2022, 11, 26)
    const nieuwjaar = new Date(2023, 0)
    const oudjaar = new Date(2022, 11, 31)
    const now = new Date()
    const countDown = document.getElementById("countdown")
    if(now.getTime() < eersteKerstdag.getTime()){
        countDown.innerHTML = `dagen tot eerste kerstdag: ${Math.ceil((eersteKerstdag.getTime() - now.getTime()) / 1000 / 3600 / 24)}<br/>` +
                            `dagen tot tweede kerstdag: ${Math.ceil((tweedeKerstdag.getTime() - now.getTime()) / 1000 / 3600 / 24)}<br/>` +
                            `dagen tot 2023: ${Math.ceil((nieuwjaar.getTime() - now.getTime()) / 1000 / 3600 / 24)}`
    }else if(now.getTime() < tweedeKerstdag.getTime()){
        countDown.innerHTML = `Fijne eerste kerstdag.<br/>` +
                            `dagen tot tweede kerstdag: ${Math.ceil((tweedeKerstdag.getTime() - now.getTime()) / 1000 / 3600 / 24)}<br/>` +
                            `dagen tot 2023: ${Math.ceil((nieuwjaar.getTime() - now.getTime()) / 1000 / 3600 / 24)}`
    }else if(now.getTime() < tweedeKerstdag.getTime() + 3600 * 24){
        countDown.innerHTML = `Prettige tweede kerstdag.<br/>` +
                            `dagen tot 2023: ${Math.ceil((nieuwjaar.getTime() - now.getTime()) / 1000 / 3600 / 24)}`
    }else if(now.getTime() < oudjaar){
        countDown.innerHTML = `dagen tot 2023: ${Math.ceil((nieuwjaar.getTime() - now.getTime()) / 1000 / 3600 / 24)}`
    }else if(now.getTime() < nieuwjaar){
        const tijd = nieuwjaar.getTime() - now.getTime()
        countDown.innerHTML = `tijd tot 2023: ${Math.floor(tijd / 3600 / 1000)}:${Math.floor(tijd / 1000 / 60 % 60)}:${Math.floor(tijd / 1000 % 60)}`
    }else if(now.getTime() - nieuwjaar < 3600 * 24){
        countDown.innerHTML = "Gelukkig nieuwjaar!"
    }else{
        countDown.innerHTML = "Helaas is de timer verlopen, we hopen later dit jaar terug te zijn met een nieuwe aftelling."
    }
}