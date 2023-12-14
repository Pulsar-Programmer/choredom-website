

document.getElementById("signout-button").addEventListener("click", function() {

    fetch('/signout', {
        method: 'POST',
        headers: {
            'Content-Type': 'application/json',
        },
    })
    .then(error)
    // .then(data => data)
    .catch(console_alert);

});