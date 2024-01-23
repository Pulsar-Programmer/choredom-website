
document.addEventListener("DOMContentLoaded", function (event) {
    event.stopPropagation();
    // event.stopImmediatePropagation();
    // event.preventDefault();
    console.log("YUHH!")
    determine_theme();
});



function determine_theme() {
    var value = "Light";
    fetch("/get-theme", {
        method: 'POST',
        headers: {
            'Content-Type': 'application/json',
        },
    })
    .then(handle)
    .then(v => {
        value = v;
    })
    .catch(notify);

    let needs_logo = document.getElementById("needs_logo");
    let needs_style = document.getElementById("needs_style");
    let needs_favicon = document.getElementById("needs_favicon");

    switch (value) {
        case "Aero": {
            needs_logo.outerHTML = `<img src="/src-web/assets/aerologo.png" id="needs_logo" alt="Choredom Logo">`;
            needs_style.outerHTML = `<link id="needs_style" rel="stylesheet" href="/src-web/static/mainaero.css">`;
            needs_favicon.outerHTML = `<link id="needs_favicon" rel="icon" type="image/ico" href="/src-web/assets/aerofavicon.ico">`;
            break;
        }
        case "Dark": {
            needs_logo.outerHTML = `<img src="/src-web/assets/darklogo.png" id="needs_logo" alt="Choredom Logo">`;
            needs_style.outerHTML = `<link id="needs_style" rel="stylesheet" href="/src-web/static/maindark.css">`;
            needs_favicon.outerHTML = `<link id="needs_favicon" rel="icon" type="image/ico" href="/src-web/assets/darkfavicon.ico">`;
            // no dark backgrounsd, would have to attach css
            // no dark footer
            break;
        }
        // Are we doing this?
        // case "Contrast": {
        //     document.getElementById("/src-web/assets/contrastlogo.png");
        //     // no contrast BG
        //     // no contrast footer
        //     document.getElementById("/src-web/assets/contrastfavicon.ico");
        // needs_favicon.outerHTML = `<link id="needs_favicon" rel="icon" type="image/ico" href="/src-web/assets/contrastfavicon.ico">`;
        //     break;
        // }
        case "Light":
        default: {
            //Default is Light
            needs_logo.outerHTML = `<img src="/src-web/assets/logo.png" id="needs_logo" alt="Choredom Logo">`;
            needs_style.outerHTML = `<link id="needs_style" rel="stylesheet" href="/src-web/static/mainlight.css">`;
            needs_favicon.outerHTML = `<link id="needs_favicon" rel="icon" type="image/ico" href="/src-web/assets/favicon.ico">`;
            break;
        }
    }
}







// const btn = document.getElementById('themeToggleBtn');
// let currentThemeIndex = 0;
// const themes = ['light', 'dark', 'high-contrast']; // Add more themes here if needed

// btn.addEventListener('click', function () {
//     // Remove all theme classes from the body
//     document.body.className = '';

//     // Cycle to the next theme
//     currentThemeIndex = (currentThemeIndex + 1) % themes.length;

//     // Apply the new theme
//     if (themes[currentThemeIndex] !== 'default') {
//         document.body.classList.add(themes[currentThemeIndex]);
//     }

//     // Save the current theme
//     localStorage.setItem('theme', themes[currentThemeIndex]);
// });


// function askTheme() {
//     let username = sessionStorage.getItem('theme');

//     if (theme === null) {
//         theme = prompt("To make your experience on this website better, please choose your prefered theme.");
//         if (theme != null) {
//             sessionStorage.setItem('theme', theme);
//         }
//     }
// }

// askTheme();
