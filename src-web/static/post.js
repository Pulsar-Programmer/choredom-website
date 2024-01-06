function submit_post(){
    let title = document.getElementById("title").value;
    let body = document.getElementById("body").value;
    let location = document.getElementById("city").value;
    let time = document.getElementById("time").value;
    let price = document.getElementById("price").value;
    let jobdata = {title: title, body: body, location: location, time: time, price: parseFloat(price)};

    fetch("/post-job-2", {
        method: 'POST', 
        headers: {
            'Content-Type': 'application/json',
        },
        body: JSON.stringify(jobdata), 
    })
    .then(handle)
    .then(_ => {
        window.location.href = "/post-job";
        window.location.reload();
    })
    .catch(notify);
}