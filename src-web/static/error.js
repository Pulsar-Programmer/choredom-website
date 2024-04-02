async function handle(response) {
    //The SeeOther functionality was scrapped to be specific for certain JS files.
    //They will be path-specific concepts.
    if (!response.ok) {
        const answer = await response.json();
        throw {msg: answer.message, for_user: answer.for_user};
    }
    let text = await response.text();
    try {
        return Promise.resolve(JSON.parse(text));
    } catch (_error) {
        return Promise.resolve("");
    }
}

function notify(error){
    if(error.msg != null){
        if(error.for_user){
            alert(error.msg);
        }
        console.log(error.msg);
    } else {
        console.log(error);
    }
}

function redirect(url){
    window.location.assign(url);
    // setTimeout(() => {
    //     window.location.reload();
    // }, 100);
}

function direct(url){
    window.location.replace(url);
    // window.location.reload();
}

// function alert_user(message) {
//     // Create a new div element
//     const errorDiv = document.createElement('div');
//     // Set the error message
//     errorDiv.textContent = message;
//     // Style the div
//     errorDiv.style.color = 'red';
//     errorDiv.style.fontSize = '14px';
//     errorDiv.style.marginTop = '5px';
//     // Append the div to the body or a specific container
//     document.body.appendChild(errorDiv);
// }
   