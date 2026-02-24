
function add_job(job){
    let add_jobs_div = document.getElementById("add_jobs");
    add_jobs_div.innerHTML += generateJobHTML(job);
}


// black outline
function generateJobHTML(job) {
    return `
    <div class = "stdbox">
        <div class="job" id="${job.id.key.String}">
            <h2 id="job_title">${job.data.title}</h3>
            <p id="job_body">${job.data.body}</p>
            <p id="job_time">Date of Task: ${job.data.time}</p>
            <p id="job_price">Price: $${job.data.price / 100.0}</p>
            <p id="job_location">Location: ${job.data.location}</p>
            <button class="btnb embedb" onclick="initiate_edit('${job.id.key.String}')">Edit Job Post</button>
            <a class="btna embedb" id="job_id" href="/jobs/${job.id.key.String}">Visit Job Post</a>
            <button class="dangerzonebtn btnb embedb" onclick="delete_job('${job.id.key.String}')">Delete Job</button>
        </div>
    </div>
    `;
}


window.onload = function() {
    fetch("/myjobs-get", {
        method: 'POST',
        headers: {
            'Content-Type': 'application/json',
        },
    })
    .then(handle)
    .then(jobsData => {
        console.log('Jobs Success:', jobsData);
        let jobs = Array.from(jobsData);
        if(jobs.length == 0){
            let add_jobs_div = document.getElementById("add_jobs");
            add_jobs_div.innerHTML = `<div class="hanger small_region">You have no Jobs! Click <a href="/post-job" class="green_link">here</a> to post a job and get started!</div>`;
        }
        jobsData.forEach(job => {
            add_job(job);
        });
    })
    .catch(notify);
}



function delete_job(job_id) {

    if(!confirm("Are you sure you want to delete this job post?")){
        return;
    }


    fetch("/delete-post", {
        method: 'POST',
        headers: {
            'Content-Type': 'application/json',
        },
        body: JSON.stringify(job_id)
    })
    .then(handle)
    .then(_ => {
        redirect("/success");
    })
    .catch(notify);
}

function label(expr, title){
    return `
        <div class="label_region">
            <label class="stdlabel">${title}</label>
            ${expr}
        </div>
    `;
}

function initiate_edit(job_id){
    //Make:
    //All the things (location, price, title, body, time) into inputs
    //Create a DONE button in place of the edit button : <button onclick="submit_edit('${job_id}')">Done</button>

    // Get the job element by ID or class
    var jobElement = document.getElementById(job_id); // Assuming each job has an ID like 'job_123'

    // Extract and replace title and body
    var title = jobElement.querySelector('#job_title');
    var body = jobElement.querySelector('#job_body');
    title.innerHTML = label(`<input type="text" value="${title.innerText}">`, "Title:");
    body.innerHTML = label(`<textarea class="stdtextarea">${body.innerText}</textarea>`, "Body:");

    // Extract and replace time, price, and location
    var price = jobElement.querySelector('#job_price');
    var time = jobElement.querySelector('#job_time');
    var location = jobElement.querySelector('#job_location');

    price.innerHTML = label(`<input type="number" value="${price.innerText.replace('Price: $', '')}">`, "Price:");

    let dateStr = time.innerText.replace('Date of Task: ', '').split('/');
    let formattedDate = `${dateStr[2]}-${dateStr[0]}-${dateStr[1]}`;
    time.innerHTML = label(`<input type="date" value="${formattedDate}">`, "Date:");
    
    location.outerHTML = label(`
    <div id="dropdown" class="dropdown">
        <link rel="stylesheet" href="https://cdnjs.cloudflare.com/ajax/libs/selectize.js/0.15.2/css/selectize.default.min.css">
        <select required name="location" id="city" style="width: 300px;">
            <option selected>${location.innerText.replace("Location: ", '')}</option>
        </select>
    </div>`, "Location:");

    $('#city').selectize({
        options: [],
        items: [],
        render: {
            option: function(data, escape) {
                return '<div>' + escape(data.text) + '</div>';
            },
            item: function(data, escape) {
                return '<div>' + escape(data.text) + '</div>';
            }
        },
        load: function(query, callback) {
            if (!query.length) return callback();
            fetch('./src-web/assets/us_cities.json')
            .then(response => response.json())
            .then(data => {
                callback(data.map(city => ({text: `${city.CITY}, ${city.STATE_NAME}`, value: `${city.CITY}, ${city.STATE_NAME}`})));
            });
        },
        // preload: true,
    });    

    // Replace the edit button with a done button
    var editButton = jobElement.querySelector('button[onclick^="initiate_edit"]');
    editButton.outerHTML = `
    <button class="btnb embedb" onclick="submit_edit('${job_id}')">Done</button>
    <button class="btnb embedb" onclick="cancel('${job_id}')">Cancel</button>`;
}


function cancel(job_id) {
    redirect("/my-jobs");
}


// function show_edit(job_id, edit){

// }


function submit_edit(job_id){
    //add the changes to the job post and put back all of the details
    
    // Get the job element by ID
    var jobElement = document.getElementById(job_id);

    // Collect the new values from input fields
    var title = jobElement.querySelector('#job_title input').value;
    var body = jobElement.querySelector('#job_body textarea').value;
    var time = jobElement.querySelector('#job_time input').value;
    var price = jobElement.querySelector('#job_price input').value;
    var location = jobElement.querySelector('#city').value;

    var confluence = {
        change: {
            title: title,
            body: body,
            time: time,
            price: price,
            location: location,
        },
        id: job_id,
    }

    fetch("/edit-post", {
        method: 'POST',
        headers: {
            'Content-Type': 'application/json',
        },
        body: JSON.stringify(confluence)
    })
    .then(handle)
    .then(_ => {
        // alert("Success!");
        // redirect("/success")
        redirect("/my-jobs");
    })
    .catch(notify);
}