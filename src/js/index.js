let lang = "";

function display_platform(){
    let platform = "";
    if(navigator.userAgent.indexOf("Win") != -1){
        platform = "Windows";
    }else if(navigator.userAgent.indexOf("Linux") != -1){
        platform = "Linux";
    }

    if(platform.length > 0){
        if(lang == "nl"){
            document.getElementById("platform").innerText = `Het lijkt er op dat u ${platform} gebruikt en dus de ${platform} en web versies kunt gebruiken.`;
        }else{
            document.getElementById("platform").innerText = `It seems like you are using ${platform} and thus can use the ${platform} and web versions.`;
        }
    }else if(lang == "nl"){
        document.getElementById("platform").innerText = `Het lijkt er op dat u geen Windows of Linux gebruikt op dit moment en u kunt dus alleen de web versies gebruiken.`;
    }else{
        document.getElementById("platform").innerText = `It seems like you aren't using Windows nor Linux, but the web versions should still work.`;
    }
}

function PayAlert(){
    if(lang == "nl"){
        alert("Vergeet niet om te doneren via petjeaf.nl");
    }else{
        alert("Don't forget to make a donation via petjeaf.nl");
    }
}
