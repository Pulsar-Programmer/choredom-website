function handle(response) {
    const answer = response.json();
    if(response.status === 303){
        window.location.href = response.headers.get('Location');
    }
    if (!response.ok) {
        throw {msg: answer.message, for_user: answer.for_user};
    }
    return answer;
}

function notify(error){
    var error = error.json();
    if(error.for_user){
        alert(answer.msg);
    }
    console.log(answer.msg);
}