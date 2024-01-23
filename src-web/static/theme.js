
document.addEventListener("DOMContentLoaded", function (event) {
    event.stopPropagation();
    // event.stopImmediatePropagation();
    // event.preventDefault();
    console.log("YUHH!")
    determine_theme();
});



async function determine_theme() {
    var value = await fetch("/get-theme", {
        method: 'POST',
        headers: {
            'Content-Type': 'application/json',
        },
    })
    .then(handle)
    .catch(notify);

    // console.log(value);

    let needs_logo = document.getElementById("needs_logo");
    let needs_style = document.getElementById("needs_style");
    let needs_favicon = document.getElementById("needs_favicon");

    let customizer = value === "Aero" ? "aero" : (value === "Dark" ? "dark" : (value === "Contrast" ? "" : ""));

    // console.log(customizer)

    needs_logo.outerHTML = `<img src="/src-web/assets/${customizer}logo.png" id="needs_logo" alt="Choredom Logo">`;
    needs_style.outerHTML = `<link id="needs_style" rel="stylesheet" href="/src-web/static/main${customizer === "" ? "light" : customizer}.css">`;
    needs_favicon.outerHTML = `<link id="needs_favicon" rel="icon" type="image/ico" href="/src-web/assets/${customizer}favicon.ico">`;
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
