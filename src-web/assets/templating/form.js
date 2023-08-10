



window.addEventListener("load", function() {


    const form = document.getElementById("signupForm");
    form.addEventListener("submit", function(event) {
        //event.preventDefault();
        
        let password1Field = document.getElementById("password");
        let password2Field = document.getElementById("password2");
        if (password2Field.value !== password1Field.value){
            alert('Passwords do not match. Please try again.');
            event.preventDefault();
        }
        password2Field.disabled = true; // disables the password2 field

        // now submit the form as is
        form.submit();
    });

    
});