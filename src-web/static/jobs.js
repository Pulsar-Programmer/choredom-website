window.onload = function() {
    var path = window.location.pathname;
    var pathParts = path.split('/');
    var newPath = pathParts[pathParts.indexOf('jobs') + 1];
    let url = window.location.href.substring(0, window.location.href.indexOf('jobs')).trim() + "jobs_attain";
    // console.log(url);
    // console.log(window.location.href)
    fetch(url, {
        method: 'POST',
        headers: {
            'Content-Type': 'application/json',
        },
        body: JSON.stringify(newPath),
    })
    .then(response => response.json())
    .then(jobsData => {
        console.log('Jobs Success:', jobsData);
        displayJob(jobsData);
    })
    .catch((error) => {
        console.error('Error:', error);
    });
}
  
function displayJob(job){
    let verification_status = job.user.state === "Verified" ? "V" : "Unv";

    document.getElementById("post-title").innerHTML = job.data.title;
    document.getElementById("post-location").innerHTML = job.data.location;
    document.getElementById("post-date").innerHTML = job.data.time;
    document.getElementById("post-price").innerHTML = job.data.price / 100.0;
    document.getElementById("post-displayname").innerHTML = `<a href="/users/${job.user.username}">${job.user.displayname}</a> (${job.user.username}) (${verification_status}erified User)`;
    document.getElementById("post-body").innerHTML = job.data.body;
}