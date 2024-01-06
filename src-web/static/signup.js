
// function signup_request(){


//     let password1 = document.getElementById("password").value;
//     let password2 = document.getElementById("password2").value;

//     if (password2 !== password1){
//         alert('Passwords do not match. Please try again.');
//     }

//     let data = {
//         password: password1,
//     }

// }

window.addEventListener("load", function() {
        // const submitButton = form.querySelector('input[type="submit"]');
    // submitButton.addEventListener("click", function(event) {
    const form = document.getElementById("signupForm");
    form.addEventListener("submit", function(event) {
        let password1Field = document.getElementById("password");
        let password2Field = document.getElementById("password2");

        // let inputFields = form.getElementsByTagName('input');
        
        // // Loop through each input field
        // for(let i = 0; i < inputFields.length; i++) {
        //     // Check if the input field is empty
        //     if (inputFields[i].type == "submit"){
        //         continue;
        //     }
        //     if(inputFields[i].value == "" || inputFields[i].value == null) {
        //         alert('Please fill all the fields');
        //         event.preventDefault(); // prevents the form from submitting
        //         return; // exit the loop
        //     }
        // }
        
        if (password2Field.value !== password1Field.value){
            alert('Passwords do not match. Please try again.');
            event.preventDefault(); // prevents the form from submitting
        } else {
            password2Field.disabled = true; // disables the password2 field
            form.submit(); // now submit the form as is
        }
    });
});