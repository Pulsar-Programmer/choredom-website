function handle(response) {
    const answer = response.json();
    if(response.status === 303){
        window.location.href = response.headers.get('Location');
    }
    if (!response.ok) {
        throw new Error(`${answer.message}`);
    }
    return answer;
}

// function alert_error(error){
//     alert(error);
// }

// function console_error(error){
//     console.log(error);
// }

function notify(error){
    if(error.for_user){
        alert(error);
        console.log(error);
    } else {
        console.log(error);
    }
}