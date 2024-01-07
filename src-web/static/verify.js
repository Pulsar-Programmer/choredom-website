


function initiate_verification(){
    var doc = document.getElementById("verify_replacable");
    doc.innerHTML = code_html();
}

function code_html(){
    return `
    <h1>Enter your 6-digit email verification code</h1>
    <form action="/ve" method="POST">
        <input type="text" name="code" id="code" placeholder="6-digit code" required maxlength="6" minlength="6">
        <input type="submit" value="Verify">
    </form>`;
}

function submit_code(){
    //...
}

