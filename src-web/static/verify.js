


function initiate_verification(){
    var doc = document.getElementById("verify_replacable");
    doc.innerHTML = code_html();
}

function code_html(){
    return `
    <h1>Enter your 6-digit email verification code</h1>
    <div>
        <input type="text" name="code" id="code" placeholder="6-digit code" required maxlength="6" minlength="6">
        <button onclick="submit_code()">Verify</button>
    </div>`;
}

function submit_code(){
    var code = document.getElementById("code").value;
    var data = {
        code: code,
    }
    fetch("/ve", {
        method: 'POST', 
        headers: {
            'Content-Type': 'application/json',
        },
        body: JSON.stringify(data),
    })
    .then(handle)
    .then(_ => {
        redirect("../");
    })
    .catch(notify);
}

