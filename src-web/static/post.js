function submit_post(){
    let title = document.getElementById("title").value;
    let body = document.getElementById("body").value;
    let location = document.getElementById("city").value;
    let time = document.getElementById("time").value;
    let price = document.getElementById("price").value;
    // try {
    //     price = parseFloat(price);
    // } catch (error) {
    //     alert("The amount of money could not be resolved. Please try again.");
    //     return;
    // }
    // if(price === undefined || price === null){
        
    // }
    let jobdata = {title: title, body: body, location: location, time: time, price: price};

    fetch("/post-job-2", {
        method: 'POST', 
        headers: {
            'Content-Type': 'application/json',
        },
        body: JSON.stringify(jobdata), 
    })
    .then(handle)
    .then(_ => {
        redirect("/post-job");
    })
    .catch(notify);
}