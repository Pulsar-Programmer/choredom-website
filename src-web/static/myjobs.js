
function add_job(job){
    let add_jobs_div = document.getElementById("add_jobs");
    add_jobs_div.innerHTML += generateJobHTML(job);
}



function generateJobHTML(job) {
    return `
        <div class="job">
            <h3>${job.data.title}</h3>
            <p>${job.data.body}</p>
            <p>Date of Task: ${job.data.time}</p>
            <p>Price: $${job.data.price / 100.0}</p>
            <p>Location: ${job.data.location}</p>
            <a id="job_id" href="/jobs/${job.id.id.String}">Visit Job Post</a>
            <button onclick="delete_job('${job.id.id.String}')">Delete Job</button>
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
            add_jobs_div.innerHTML = `<p> You have no Jobs! Click <a href="/post-job">here</a> to post a job and get started!`;
        }
        jobsData.forEach(job => {
            add_job(job);
        });
    })
    .catch(notify);
}



function delete_job(job_id) {
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

// function edit_job(){

    
// }