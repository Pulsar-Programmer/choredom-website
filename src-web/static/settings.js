function settings_post(){
    let url = 'http://localhost:8080/settings-post';
    let data = {
        username: 'username', 
        password: 'password',
        displayname: 'displayname',
        location: 'location',
        bio: 'bio',
        pfp: 'pfp'
    };
    fetch(url, {
        method: 'POST', 
        headers: {
            'Content-Type': 'application/json',
        },
        body: JSON.stringify(data), 
    })
    .then(response => response.json())
    .then(data => console.log(data))
    .catch((error) => {
        console.error('Error:', error);
    });
}

window.addEventListener("load", function() {

    const delete_form = document.getElementById("delete-account-form");
    delete_form.addEventListener("submit", function(event) {
        var passwordConfirmation = document.getElementById("password-confirmation").value;
        if (passwordConfirmation === "") {
            alert("Please enter your password to confirm deletion.");
            event.preventDefault();
        }
        let didConfirm = confirm('Are you sure you want to delete your account? This action cannot be undone.');
        if (didConfirm){
            delete_form.submit();
        }
    });

    fetch('/settings/present_data', {
        method: 'POST', 
        headers: {
            'Content-Type': 'application/json',
        },
    })
    .then(response => response.json())
    .then(data => {
        prefill(data.username, data.displayname, data.location, data.bio)
    })
    .catch((error) => {
        console.error('Error:', error);
    });

});

function prefill(username, displayname, location, bio){
    document.getElementById('displayname').value = displayname;
    document.getElementById('username').value = username;
    document.getElementById('location').value = location;
    document.getElementById('bio').value = bio;
}

// window.onload = function() {
//     prefill();
// }