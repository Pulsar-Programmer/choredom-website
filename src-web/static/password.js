window.addEventListener("load", function() {
    const form = document.getElementById("passwordForm");
    form.addEventListener("submit", function(event) {


        let currentPassword = document.getElementById("currentPassword").value;
        let newPassword = document.getElementById("newPassword").value;
        let confirmPasswordField = document.getElementById("confirmPassword");

        // Check if new password and confirm password match
        if (newPassword !== confirmPasswordField.value) {
            alert("New password and confirm password do not match");
            event.preventDefault();
        }

        // Check if current password is not empty
        if (currentPassword === "") {
            alert("Please enter your current password");
            event.preventDefault();
        }

        if (newPassword === "") {
            alert("Please enter your current password");
            event.preventDefault();
        }

        if (confirmPasswordField.value === "") {
            alert("Please enter your current password");
            event.preventDefault();
        }
        confirmPasswordField.disabled = true; // disables the password2 field

        // now submit the form as is
        form.submit();
    });
});