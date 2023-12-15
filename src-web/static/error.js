function error(response) {
    const answer = response.json();
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

function console_alert(error){
    if(error.for_user){
        alert(error);
        console.log(error);
    } else {
        console.log(error);
    }
}