
function signup_request(){
    
    let email = document.getElementById("email").value;
    let password1 = document.getElementById("password").value;
    let password2 = document.getElementById("password2").value;
    let username = document.getElementById("username").value;
    let displayname = document.getElementById("displayname").value;
    let location = document.getElementById("city").value;

    if (password2 !== password1){
        alert('Passwords do not match. Please try again.');
    }

    let data = {
        email: email,
        password: password1,
        username: username,
        displayname: displayname,
        location: location,
    }

    fetch("/verify-email", {
        method: 'POST', 
        headers: {
            'Content-Type': 'application/json',
        },
        body: JSON.stringify(data), 
    })
    .then(handle)
    .then(_ => {
        initiate_verification();
    }) //For some reason this causes an error upon HttpResponse::Ok().finish(). Why? I FIXED IT.
    .catch(notify);
}

// window.addEventListener("load", function() {
//         // const submitButton = form.querySelector('input[type="submit"]');
//     // submitButton.addEventListener("click", function(event) {
//     const form = document.getElementById("signupForm");
//     form.addEventListener("submit", function(event) {
//         let password1Field = document.getElementById("password");
//         let password2Field = document.getElementById("password2");

//         // let inputFields = form.getElementsByTagName('input');
        
//         // // Loop through each input field
//         // for(let i = 0; i < inputFields.length; i++) {
//         //     // Check if the input field is empty
//         //     if (inputFields[i].type == "submit"){
//         //         continue;
//         //     }
//         //     if(inputFields[i].value == "" || inputFields[i].value == null) {
//         //         alert('Please fill all the fields');
//         //         event.preventDefault(); // prevents the form from submitting
//         //         return; // exit the loop
//         //     }
//         // }
        
//         if (password2Field.value !== password1Field.value){
//             alert('Passwords do not match. Please try again.');
//             event.preventDefault(); // prevents the form from submitting
//         } else {
//             password2Field.disabled = true; // disables the password2 field
//             form.submit(); // now submit the form as is
//         }
//     });
// });