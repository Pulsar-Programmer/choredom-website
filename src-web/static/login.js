
// document.querySelector('#vrbtn').addEventListener('click', async function() {
//     this.disabled = true;
//     console.log(this.disabled)
//     await login_request();
//     this.disabled = false;
//     console.log(this.disabled)
// });

function login_request(){

    let email = document.getElementById("email").value;
    let password = document.getElementById("password").value;

    let data = {
        email: email,
        password: password,
    }

    lock_btn();

    fetch("/signin", {
        method: 'POST', 
        headers: {
            'Content-Type': 'application/json',
        },
        body: JSON.stringify(data), 
    })
    .then(handle)
    .then(_ => {
        initiate_verification("/ve_log");
    }) //For some reason this causes an error upon HttpResponse::Ok().finish(). Why? I FIXED IT.
    .catch(unlock_notify);
}