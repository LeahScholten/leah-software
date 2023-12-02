let username = window.location.search.slice(1);
const firstChristmasDay = new Date(2023, 11, 25, 0, 0, 0, 0);
const secondChristmasDay = new Date(2023, 11, 26, 0, 0, 0, 0);
const nextYear = new Date(2024, 0, 1, 0, 0, 0, 0);

const setup = () => {
    let relatie;

    username = username[0].toUpperCase() + username.slice(1);
    console.log(username);
    switch(username.toLowerCase()){
        case "opa": case "oma":
            relatie = `mijn ${username}`;
            username[0] = username[0].toLowerCase();
            document.getElementById("ending").innerText = "Knuffels,";
            break;
        case "mama":
            relatie = `mijn moeder`;
            username[0] = username[0].toLowerCase();
        case "daisy":
            relatie = "mijn zus";
            break;
        case "chris":
            relatie = "mijn neef";
            break;
        case "iris":
            relatie = "mijn nicht";
            break;
        case "arjen":
            relatie = "mijn oom";
            break;
        case "nicole":
            relatie = "mijn tante";
            break;
        case "els": case "tom": case "kelly": case "hanneke": case "ruud":
            relatie += "je familie";
            break;
        case "nathalja": case "catharina": case "mike":
            relatie += "een van mijn beste vrienden";
            break;
        default:
            document.body.innerText = "";
            return false;
    }

    document.getElementById("greeting").innerText = `Hallo ${username},`;
    document.getElementById("message").innerHTML = "<p>Als je dit leest, wil ik zeggen dat je een plek in mijn hart hebt.<br/>" +
                  `Dit is omdat je ${relatie} bent.<br/>` +
                  "Daarom wens ik je een fijne kerst en een gelukkig en gezond nieuw jaar.</p>";
    return true;
};

const millisecondsToDays = (milliseconds) => Math.ceil(milliseconds / 1000 / 3600 / 24);
const millisecondsToHMS = (milliseconds) => {
    let seconds = Math.ceil(milliseconds / 1000);
    let minutes = Math.ceil(seconds / 60);
    let hours = Math.ceil(minutes / 60);
    minutes %= 60;
    seconds %= 60;
    if(hours < 10){
        hours = "0" + hours;
    }
    if(minutes < 10){
        minutes = "0" + minutes;
    }
    if(seconds < 10){
        seconds = "0" + seconds;
    }
    return `${hours}:${minutes}:${seconds}`;
};

const loop = () => {
    const now = new Date();
    let content = "";
    if(now < firstChristmasDay){
        content += `Dagen tot eerste kerstdag: ${millisecondsToDays(firstChristmasDay - now)}<br/>`;
    }
    if(now < secondChristmasDay){
        content += `Dagen tot tweede kerstdag: ${millisecondsToDays(secondChristmasDay - now)}<br/>`;
    }
    if(millisecondsToDays(nextYear - now) > 0){
        content += `Dagen tot nieuwjaar ${millisecondsToDays(nextYear - now)}`;
    }else if(now < nextYear){
        content += `${millisecondsToHMS(now - nextYear)} tot 2024`;
    }else{
        content += "Gelukkig 2024!";
    }
    document.getElementById("countdown").innerHTML = content;
};

async function main(){
    if(!setup()){
        return;
    }
    while(true){
        loop();
        await new Promise(r => setTimeout(r, 1000));
    }
}
