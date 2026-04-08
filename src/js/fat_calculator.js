// The strings for both supported languages
const ENGLISH = "en";
const DUTCH = "nl";

// Read a float input value
function read_float(id){
    const value = document.getElementById(id).value
    return Number(value);
}

// Display the requested text
function display_output(text){
    document.getElementById("output").innerText = text;
}

// Check whether the input is valid, display an error if not
function check_input(english_name, dutch_name, input, language){
    if(input <= 0){
        if(language == ENGLISH){
            display_output(`Invalid value for ${english_name}`);
        }else{
            display_output(`Ongeldige waarde voor ${dutch_name}`)
        }
        return false;
    }
    return true;
}

// Hide the element with the requested id
function hide_element(id){
    document.getElementById(id).style.display = "none";
}

// Hide the elements with the requested ids
function hide_elements(ids){
    for(const id of ids){
        hide_element(id);
    }
}

// Show the element with the requested id
function show_element(id){
    document.getElementById(id).style.display = "inline";
}

// Calculate fat percentage and mass
function calculate_fat_percentage(language){
    // Read the height
    const height = read_float("height");
    if((!check_input("height", "lengte", height))){
        hide_elements(["weight_span", "waist_span", "neck_span", "gender_span", "hip_span"]);
        return;
    }

    // Read the weight
    show_element("weight_span");
    const weight = read_float("weight");
    if(!check_input("weight", "gewicht", weight)){
        hide_elements(["waist_span", "neck_span", "gender_span", "hip_span"]);
        return;
    }

    // Read the waist circumference
    show_element("waist_span");
    const waist = read_float("waist");
    if(!check_input("waist circumference", "middel omtrek", waist)){
        hide_elements(["neck_span", "gender_span", "hip_span"]);
        return;
    }

    // Make sure the waist circumference is valid
    if(waist < 45.0 || waist > height){
        if(language == ENGLISH){
            display_output("Waist circumference must be between 45 cm and your height");
        }else{
            display_output("Middel omtrek moet tussen de 45 cm en uw lengte zijn");
        }
        hide_elements(["neck_span", "gender_span", "hip_span"]);
        return;
    }

    // Read the neck circumference
    show_element("neck_span");
    const neck = read_float("neck");
    if(!check_input("neck circumference", "nek omtrek", neck)){
        hide_elements(["gender_span", "hip_span"]);
        return;
    }

    // Read the gender
    show_element("gender_span");
    const genders = document.getElementsByName("gender");
    let selected_gender = "";
    for(const gender of genders){
        if(gender.checked){
            selected_gender = gender.value;
        }
    }

    // Calculate the fat percentage
    const log_con = Math.log(10);
    let fat_percentage = 0;
    switch(selected_gender){
        case "":
            if(language == ENGLISH){
                display_output("No gender has been selected yet");
            }else{
                display_output("U heeft nog geen gender geselecteerd");
            }
            hide_element("hip_span");
            return;
        case "man":
            fat_percentage = 495.0
                    / (1.0324
                        - 0.19077 * (Math.log(waist - neck) / log_con)
                        + 0.15456 * (Math.log(height) / log_con))
                    - 450.0;
            hide_element("hip_span");
            break;
        case "woman":
            // Women have fat in their hips, so also use that to calculate the fat percentage
            show_element("hip_span");
            const hip = read_float("hip");
            if(!check_input("hip circumference", "heup omtrek", hip)){
                return;
            }
            fat_percentage = 495.0
                    / (1.29579
                        - 0.35004
                            * (Math.log(waist + hip - neck)
                                / log_con)
                        + 0.221 * (Math.log(height) / log_con))
                    - 450.0;
            break;
        default:
            // This code should never be executed
            if(language == ENGLISH){
                display_output("Invalid gender label found");
            }else{
                display_output("Ongeldig gender label gevonden")
            }
            hide_element("hip_span");
            return;
    }

    // Calculate the lean and fat mass
    const lean_mass = (weight * ((100 - fat_percentage) / 100)).toFixed(1);
    const fat_mass = (weight.toFixed(1) - lean_mass).toFixed(1);
    fat_percentage = fat_percentage.toFixed(1);

    // Display the results
    let output = "";
    if(language == ENGLISH){
        output += `Fat percentage: ${fat_percentage}%\n`;
        output += `Lean mass: ${lean_mass} kg\n`;
        output += `Fat mass: ${fat_mass} kg\n`;
    }else{
        output += `Vet percentage: ${fat_percentage}%\n`;
        output += `Vetvrije massa: ${lean_mass} kg\n`;
        output += `Vet massa: ${fat_mass} kg\n`;
    }
    display_output(output)
}