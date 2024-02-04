function submit_post(){
    let title = document.getElementById("title").value;
    let body = document.getElementById("body").value;
    let location = document.getElementById("city").value;
    let time = document.getElementById("time").value;
    let price = document.getElementById("price").value;
    
    if(String(title).trim() === "") {
        alert("Please fill in the `title` field.");
        return;
    }

    if(String(location).trim() === "") {
        alert("Please fill in the `location` field.");
        return;
    }

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
        redirect("/success");
    })
    .catch(notify);
}

document.addEventListener("DOMContentLoaded", function (event) {
    event.stopPropagation();
    // event.stopImmediatePropagation();
    // event.preventDefault();
    console.log("YUHH!")
    fetch('/settings/present_data', {
        method: 'POST', 
        headers: {
            'Content-Type': 'application/json',
        },
    })
    .then(handle)
    .then(data => {
        prefill_jp(data.location)
    })
    .catch(notify);
    // prefill_tia(get_settings_location())
});

function prefill_jp(location){
    var selectElement = document.getElementById('city');
    var selectize = selectElement.selectize;
    selectize.addOption({
        text: location,
        value: location
    });
    selectize.setValue(location);
    // console.log(location, selectize, selectElement)
}