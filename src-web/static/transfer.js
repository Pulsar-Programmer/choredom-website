function send_request(){
    let to_username = document.getElementById("recipientUsername").value;

    let self_password = document.getElementById("password").value;

    let amount = document.getElementById("amount").value;
    // let n = Number(amount);
    
    // if (n != amount || !isFinite(n) || isNaN(n) || n % 1 !== 0) {
    //     alert("Enter a valid number!")
    //     return;
    // }

    let data = {
        credits: amount,
        to_username: to_username,
        self_password: self_password,
    }

    fetch("/settings/funds/transfer/form", {
        method: 'POST',
        headers: {
            'Content-Type': 'application/json',
        },
        body: JSON.stringify(data),
    })
    .then(handle)
    .then(_ => redirect("/success"))
    .catch(notify);
}