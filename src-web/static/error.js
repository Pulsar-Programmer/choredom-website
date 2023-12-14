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
    // alert_error(error);
    // console_error(error);
    alert(error);
    console.log(error);
}