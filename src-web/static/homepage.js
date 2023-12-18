

document.getElementById("signout-button").addEventListener("click", function() {

    fetch('/signout', {
        method: 'POST',
        headers: {
            'Content-Type': 'application/json',
        },
    })
    .then(handle)
    // .then(data => data)
    .catch(notify);

});