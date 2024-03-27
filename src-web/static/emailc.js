
function emailc_request(){
    
    let email1 = document.getElementById("currentemail").value;
    let email2 = document.getElementById("newEmail").value;
    let password = document.getElementById("password").value;

    let data = {
        e_old: email1,
        e_new: email2,
        password: password,
    }

    lock_btn();

    fetch("/settings/email/form", {
        method: 'POST', 
        headers: {
            'Content-Type': 'application/json',
        },
        body: JSON.stringify(data), 
    })
    .then(handle)
    .then(_ => {
        initiate_verification("/ve_set");
    }) //For some reason this causes an error upon HttpResponse::Ok().finish(). Why? I FIXED IT.
    .catch(unlock_notify);
}