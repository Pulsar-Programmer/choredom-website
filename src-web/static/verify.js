
function unlock_notify(args){
    notify(args);
    unlock_btn();
}

function initiate_verification(url){
    var doc = document.getElementById("verify_replacable");
    doc.innerHTML = code_html(url);
}

function code_html(url){
    return `
    <div>
        <label class="stdlabel" for="code">Enter Your 6-Digit Email Verification Code</label>
        <input type="text" name="code" id="code" placeholder="6-digit code" required maxlength="6" minlength="6">

        <button class="btnb embedb" onclick="submit_code('${url}')">Verify</button>
    </div>`;
}

function submit_code(url){
    var code = document.getElementById("code").value;
    var data = {
        code: code,
    }
    // initiate_loading();
    fetch(url, {
        method: 'POST', 
        headers: {
            'Content-Type': 'application/json',
        },
        body: JSON.stringify(data),
    })
    .then(handle)
    .then(_ => {
        // revert_loading();
        redirect("/");
    })
    .catch(notify);
}

function lock_btn(){
    let btn = document.getElementById("vrbtn");
    btn.disabled = true;
    btn.style.backgroundColor = "#268255";
}

function unlock_btn(){
    let btn = document.getElementById("vrbtn");
    btn.disabled = false;
    btn.style.backgroundColor = "";
}