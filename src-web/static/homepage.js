

document.getElementById("signout-button").addEventListener("click", function() {

    fetch('/signout', {
        method: 'POST',
        headers: {
            'Content-Type': 'application/json',
        },
    })
    .then(response => response.json())
    // .then(data => data)
    .catch((error) => {
        console.error('Error:', error);
    });

});