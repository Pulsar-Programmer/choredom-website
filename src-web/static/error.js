async function handle(response) {
    const answer = await response.json();
    if(response.status === 303){
        window.location.href = response.headers.get('Location');
        window.location.reload();
    }
    if (!response.ok) {
        throw {msg: answer.message, for_user: answer.for_user};
    }
    return answer;
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