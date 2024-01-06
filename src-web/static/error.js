async function handle(response) {
    //The SeeOther functionality was scrapped to be specific for certain JS files.
    if (!response.ok) {
        const answer = await response.json();
        throw {msg: answer.message, for_user: answer.for_user};
    }
    return response.json();
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