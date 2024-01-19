


function initiate_verification(url){
    var doc = document.getElementById("verify_replacable");
    doc.innerHTML = code_html(url);
}

function code_html(url){
    return `
    <h1>Enter your 6-digit email verification code</h1>
    <div>
        <input type="text" name="code" id="code" placeholder="6-digit code" required maxlength="6" minlength="6">
        <button onclick="submit_code('${url}')">Verify</button>
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



//@Lockroach
function initiate_loading(){
    var div = document.getElementById("verify_replacable");
    // div.style.filter = "brightness(50%)";
    div.style.backgroundColor = "rgba(0, 0, 0, 0.5)";
    div.style.display = "flex";
    div.style.justifyContent = "center";
    div.style.alignItems = "center";
    var img = document.createElement('img');
    img.src = "/src-web/assets/loading.png";
    div.appendChild(img);
}

function revert_loading() {
    var div = document.getElementById("verify_replacable");
    div.style.filter = null; // Reverts to the original brightness
    div.style.display = null; // Reverts to the original display type
    div.style.justifyContent = null; // Reverts to the original alignment
    div.style.alignItems = null; // Reverts to the original alignment
 
    // Remove the loading logo
    var img = div.getElementsByTagName('img')[0];
    div.removeChild(img);
}
 