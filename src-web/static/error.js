async function handle(response) {
    const answer = await response.json();
    if(response.status === 303){
        //we are going to have to
        //check if the response is SeeOther somehow
        //and if it is then we just go to the place
        //in the JSON sent maybe instead of using headers
        //I mean we can idk whatever works. 
        window.location.href = response.headers.get('Location');
        window.location.reload();
    }
    if (!response.ok) {
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