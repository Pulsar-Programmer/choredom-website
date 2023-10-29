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
    // const submitButton = form.querySelector('input[type="submit"]');
// submitButton.addEventListener("click", function(event) {

// const post_form = document.getElementById("delete-account-form");
// post_form.addEventListener("submit", function(event) {
    
//     // let username = document.getElementById("username");
//     // let displayname = document.getElementById("displayname");
//     // let location = document.getElementById("location");
//     // let email = document.getElementById("email");
//     // let bio = document.getElementById("bio");
//    // dont let it submit invalid fields(just like in signup or whatever)
//     // if (username === "" || displayname === "" || location === "" || email === "" || bio === ""){
//     //     alert("Please don't submit a blank field.")
//     //     event.preventDefault();
//     // } 

//     post_form.submit();

// });

const delete_form = document.getElementById("delete-account-form");
post_form.addEventListener("submit", function(event) {
    

    
    var passwordConfirmation = document.getElementById("password-confirmation").value;
    if (passwordConfirmation === "") {
        alert("Please enter your password to confirm deletion.");
        event.preventDefault();
    }
    let didConfirm = confirm('Are you sure you want to delete your account? This action cannot be undone.');
    if (didConfirm){
        post_form.submit();
    }

});
});