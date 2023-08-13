
// Function to generate the HTML for each job
function generateJobHTML(job) {
    return `
        <div class="job">
        <h3>${job.data.title}</h3>
        <h4>${job.user.displayname} (${job.user.username})</h4>
        <p>${job.data.body}</p>
        <p>Date and Time: ${job.data.time}</p>
        <p>Price: $${job.data.price}</p>
        <button onclick="applyForJob('${job.user.username}')">Apply</button>
        <a href="/${job.id}">Visit Job Post</a>
        </div>
    `;
}

// Get the post container element
const jobContainer = document.getElementById("job-container");

// Function to display jobs on the frontend
function displayJobs(jobsData) {
    jobContainer.innerHTML = ``;
    jobsData.forEach((job) => {
        const jobHTML = generateJobHTML(job);
        jobContainer.innerHTML += jobHTML;
    });
}

// Function to start a chat with the poster
function startChat() {
    // Open a new chat or space with the poster's username
}

// Function to apply for a job
function applyForJob() {
    
}

function get_location_data(){
    const location = document.getElementById('filterInput').value;

    fetch('/job-handling', {
        method: 'POST',
        headers: {
            'Content-Type': 'application/json',
        },
        body: JSON.stringify(location),
    })
    .then(response => response.json())
    .then(jobsData => {
        console.log('Location Success:', jobsData);
        
        console.log('Jobs Success:', jobsData);
        displayJobs(jobsData);
    })
    .catch((error) => {
        console.error('Error:', error);
    });
}








// Function to toggle dark mode
// document.getElementById('toggle-button').addEventListener('click', function() {
//     document.body.classList.toggle('dark-mode');
// });