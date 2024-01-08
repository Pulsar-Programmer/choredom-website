
function login_request(){
    
    let email = document.getElementById("email").value;
    let password = document.getElementById("password").value;

    let data = {
        email: email,
        password: password,
    }

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
    .catch(notify);
}