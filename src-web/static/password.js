function submit_passcode(){

    let currentPassword = document.getElementById("currentPassword").value;
    let newPassword = document.getElementById("newPassword").value;
    let confirmPasswordField = document.getElementById("confirmPassword");

    // Check if new password and confirm password match
    if (newPassword !== confirmPasswordField.value) {
        alert("New password and confirm password do not match");
        return;
    }

    // Check if current password is not empty
    if (currentPassword === "") {
        alert("Please enter your current password");
        return;
    }

    if (newPassword === "") {
        alert("Please enter your current password");
        return;
    }

    if (confirmPasswordField.value === "") {
        alert("Please enter your current password");
        return;
    }

    let data = {
        p_old: currentPassword,
        p_new: newPassword,
    }

    fetch('/settings/password/form', {
        method: 'POST',
        headers: {
            'Content-Type': 'application/json',
        },
        body: JSON.stringify(data),
    })
    .then(handle)
    .then(_ => redirect("/settings"))
    .catch(notify);
}