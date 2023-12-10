function settings_post(){
    let url = 'http://localhost:8080/settings-post';

    let username = document.getElementById('username').value;
    let password = document.getElementById('password-confirmation').value;
    let displayname = document.getElementById('displayname').value;
    let location = document.getElementById('city').value;
    let bio = document.getElementById('bio').value;
    let pfpInput = document.getElementById('pfp');
    let pfp = pfpInput.files.length > 0 ? URL.createObjectURL(pfpInput.files[0]) : '';

    // For the bio pictures, you need to read the file inputs and convert them to a format that can be sent via JSON
    let bioPicsInput = document.getElementById('bioPics');
    let bioPics = [];
    for (let i = 0; i < bioPicsInput.files.length; i++) {
        bioPics.push(URL.createObjectURL(bioPicsInput.files[i]));
    }

    let data = {
        username: username, 
        password: password,
        displayname: displayname,
        location: location,
        bio: bio,
        pfp: pfp,
        bioPics: bioPics
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
    .then(response => error(response))
    .then(data => {
        prefill(data.username, data.displayname, data.location, data.bio)
    })
    .catch(console_alert_error);

});

function prefill(username, displayname, location, bio){
    //does the profile pic show up during settings-present?
    document.getElementById('displayname').value = displayname;
    document.getElementById('username').value = username;
    document.getElementById('bio').value = bio;
    document.getElementById('gritty').href = `/users/${username}`;
    //location:

    // var $select = $('#city').selectize();
    // var selectize = $select[0].selectize;
    // console.log(location);
    // console.log(selectize);
    // selectize.setValue(location, true);
    // This is your second script  
    var selectElement = document.getElementById('city');
    var selectize = selectElement.selectize;
    selectize.addOption({
        text: location,
        value: location
    });
    selectize.setValue(location);
    console.log(location, selectize, selectElement)
}