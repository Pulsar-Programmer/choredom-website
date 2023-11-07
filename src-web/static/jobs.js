window.onload = function() {
    var path = window.location.pathname;
    var pathParts = path.split('/');
    var newPath = pathParts[pathParts.indexOf('jobs') + 1];

    fetch('/job-handling', {
        method: 'POST',
        headers: {
            'Content-Type': 'application/json',
        },
        body: JSON.stringify(newPath),
    })
    .then(response => response.json())
    .then(jobsData => {
        console.log('Location Success:', jobsData);
        
        console.log('Jobs Success:', jobsData);
        displayJob(jobsData);
    })
    .catch((error) => {
        console.error('Error:', error);
    });
}
  
function displayJob(job){
//jobsData.user.username, jobsData.user.state, jobData.user.rating ..
//jobsData.data <== anything with to do with the actual job

let verification_status = job.user.state === "Verified" ? "V" : "Unv";
`
    <div class="job">
    <h3>${job.data.title}</h3>
    <h4><a href="/users/${job.user.username}">${job.user.displayname}</a> (${job.user.username}) (${verification_status}erified User)</h4>
    <p>${job.data.body}</p>
    <p>Date of Task: ${job.data.time}</p>
    <p>Price: $${job.data.price / 100.0}</p>
    <a href="/${job.id}">Visit Job Post</a>
    </div>
`;
//Shatmaster work on this
}