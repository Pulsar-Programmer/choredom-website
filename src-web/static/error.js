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
    window.location.reload();
}

function direct(url){
    window.location.replace(url);
    // window.location.reload();
}