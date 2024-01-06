function settings_post(){
    let url = '/settings-post';

    let username = document.getElementById('username').value;
    // let password = document.getElementById('password-confirmation').value;
    let displayname = document.getElementById('displayname').value;
    let location = document.getElementById('city').value;
    let bio = document.getElementById('bio').value;
    // let pfpInput = document.getElementById('pfp');
    // let pfp = pfpInput.files.length > 0 ? URL.createObjectURL(pfpInput.files[0]) : '';

    // For the bio pictures, you need to read the file inputs and convert them to a format that can be sent via JSON
    // let bioPicsInput = document.getElementById('bioPics');
    // let bioPics = [];
    // for (let i = 0; i < bioPicsInput.files.length; i++) {
    //     bioPics.push(URL.createObjectURL(bioPicsInput.files[i]));
    // }
    //WUT ^^ how old is this?

    let data = {
        username: username, 
        // password: password,
        displayname: displayname,
        location: location,
        bio: bio,
        // pfp: pfp,
        // bioPics: bioPics
    };

    fetch(url, {
        method: 'POST', 
        headers: {
            'Content-Type': 'application/json',
        },
        body: JSON.stringify(data), 
    })
    .then(handle)
    .then(_ => {
        redirect("/settings");
    })
    .catch(notify);
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
    .then(handle)
    .then(data => {
        prefill(data.username, data.displayname, data.location, data.bio, data.pfplink)
    })
    .catch(notify);
});

function prefill(username, displayname, location, bio, pfplink){
    document.getElementById('pfp').src = pfplink;
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








function upload_pfp(){
    const fileInputElement = document.getElementById("file_upload_pfp");
    let formData = new FormData();
    formData.append('file', fileInputElement.files[0], 'filename.png');
    console.log(formData)
    fetch('/settings/pics-pfp', {
        method: 'POST',
        body: formData
    })
    .then(handle)
    .then(_ => {
        // let data = String(data);
        alert(`Successful upload!`); //Your url is https:://www.choredom.com${data}
    })
    .catch(notify);
}

function upload_bio(){
    const fileInputElement = document.getElementById("file_upload_bio");
    let formData = new FormData();
    for(f of fileInputElement.files){
        formData.append('file', f, 'filename.png');
    }
    fetch('/settings/pics-bio', {
        method: 'POST',
        body: formData,
    })
    .then(handle)
    .then(data => {
        let mylinks = String(data);
        alert(`Successful upload! Your links are the following: \n${mylinks} You can paste the links in and they will auto-resolve.`)
    })
    .catch(notify);
}