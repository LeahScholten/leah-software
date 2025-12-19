let username = window.location.search.slice(1);

const calculate_year = () => {
    const now = new Date();
    if (now.getMonth() >= 11) {
        return now.getFullYear();
    }
    return now.getFullYear() - 1;
};

const this_year = calculate_year();
const firstChristmasDay = new Date(this_year, 11, 25, 0, 0, 0, 0);
const secondChristmasDay = new Date(this_year, 11, 26, 0, 0, 0, 0);
const nextYear = new Date(this_year + 1, 0, 1, 0, 0, 0, 0);

const setup = () => {
    if (username.length == 0) {
        document.body.innerHTML = "";
        return false;
    }

    const USERNAME_LOWERCASE = username.toLowerCase();
    let relatie;
    let convert_to_lower_case = false;
    switch (USERNAME_LOWERCASE) {
        case "opa": case "oma":
            relatie = `mijn ${USERNAME_LOWERCASE}`;
            document.getElementById("ending").innerText = "Knuffels,";
            convert_to_lower_case = true;
            break;
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
        case "els": case "tom": case "kelly": case "hanneke": case "ruud": case "jeroen":
            relatie = "familie";
            break;
        case "nathalja": case "catharina": case "mike":
            relatie = "een van mijn beste vrienden";
            break;
        case "jolinda":
            relatie = "zo'n goede moeder voor Levy";
            break;
        case "Angelique": case "Esme":
            relatie = "een vriendin van oma";
            break;
        default:
            document.body.innerHTML = "";
            return false;
    }

    if (convert_to_lower_case) {
        username = USERNAME_LOWERCASE;
    } else {
        username = username[0].toUpperCase() + username.slice(1);
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
    if (hours < 10) {
        hours = "0" + hours;
    }
    if (minutes < 10) {
        minutes = "0" + minutes;
    }
    if (seconds < 10) {
        seconds = "0" + seconds;
    }
    return `${hours}:${minutes}:${seconds}`;
};

const loop = () => {
    let fast_countdown = false;
    const now = new Date();
    let content = "";
    if (now < firstChristmasDay) {
        content += `Dagen tot eerste kerstdag: ${millisecondsToDays(firstChristmasDay - now)}<br/>`;
    }
    if (now < secondChristmasDay) {
        content += `Dagen tot tweede kerstdag: ${millisecondsToDays(secondChristmasDay - now)}<br/>`;
    }
    if (millisecondsToDays(nextYear - now) > 0) {
        content += `Dagen tot nieuwjaar ${millisecondsToDays(nextYear - now)}`;
    } else if (now < nextYear) {
        content += `${millisecondsToHMS(now - nextYear)} tot 2024`;
        fast_countdown = true;
    } else {
        content += $`Gelukkig ${this_year}!`;
    }
    document.getElementById("countdown").innerHTML = content;
    return fast_countdown;
};

async function main() {
    if (!setup()) {
        return;
    }
    while (true) {
        if (loop()) {
            await new Promise(r => setTimeout(r, 1000 - (new Date).getMilliseconds()));
        } else {
            const NOW = (new Date);
            const DELAY = 24 * 3600 - ((NOW.getHours() * 60 + NOW.getMinutes()) * 60 + NOW.getSeconds());
            console.log(DELAY);
            await new Promise(r => setTimeout(r, DELAY * 1000));
        }
    }
}
