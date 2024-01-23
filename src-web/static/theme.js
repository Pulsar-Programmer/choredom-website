
window.onload = function theme(){
    // fetches theme from backend
    //backend returns "Dark", "Light", "Aero", "Contrast" as enum from Rust
    // return "Aero";
    var value = "Aero";
    
    switch(value){
        case "Aero": {
            let needs_logo = document.getElementById("needs_logo");
            needs_logo.outerHTML = `<img src="/src-web/assets/aerologo.png" id="needs_logo" alt="Choredom Logo">`;
            // let needs_footer = document.getElementById("needs_footer");
            // document.getElementById("/src-web/assets/aerobackground.png");
            // document.getElementById("/src-web/assets/aerofooter.png");
            // document.getElementById("/src-web/assets/aerofavicon.ico");
            break;
        }
// //         case "Dark" : {
// //             document.getElementById("/src-web/assets/darklogo.png");
// no dark background, would have to attach css
// no dark footer
// //             document.getElementById("/src-web/assets/darkfavicon.ico");
// //             break;
// //         }
            // Are we doing this?
// //         case "Contrast": {
// //             document.getElementById("/src-web/assets/contrastlogo.png");
// no contrast BG
// no contrast footer
// //             document.getElementById("/src-web/assets/contrastfavicon.ico");
// //             break;
// //         }
// //         case "Light" : {
// //             document.getElementById("/src-web/assets/lightlogo.png");
// no light bg
// no light footer
// //             document.getElementById("/src-web/assets/favicon.ico");
// //             break;
// //         }
// //         default: {
// //             "Light"
// //         }
    }
}



