
// Function to generate the HTML for each job
function generateJobHTML(job) {
    return `
        <div class="job">
        <h3>${job.data.title}</h3>
        <h4><a href="/profile/${job.user.userId}">${job.user.displayname}</a> (${job.user.username})</h4>
        <p>${job.data.body}</p>
        <p>Date and Time: ${job.data.time}</p>
        <p>Price: $${job.data.price}</p>
        <button onclick="initiateChat('${job.user.userId}', '${currentUserId}')">Apply</button>
        <a href="/${job.id}">Visit Job Post</a>
        </div>
    `;
}

// function to start chat
function initiateChat(user1, user2) {
    // Send a request to the server to create a chat room
    fetch('/create-chat-room', {
        method: 'POST',
        headers: {
        'Content-Type': 'application/json',
        },
        body: JSON.stringify({
            user1: user1, 
            user2: user2
        }),
    })
    .then(response => response.json())
    .then(roomId => {
        // Redirect the user to the chat room using the generated room ID
        let url = `/chats/room=${roomId}`;
        window.location.href = url;
    })
    .catch((error) => {
        console.error('Error:', error);
    });
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




// Dropdown menu code
fetch('/src-web/assets/us_cities.json')
.then(response => response.json())
.then(data => populateDropdown(data));

function populateDropdown(data) {
    const dropdown = document.getElementById('dropdownOptions');
    dropdown.style.display = 'none'; // Hide the dropdown initially
    data.forEach(town => {
    const option = document.createElement('div');
    option.setAttribute('value', town.ID);
    option.textContent = `${town.CITY}, ${town.STATE_NAME}`; 
    option.addEventListener('click', selectOption); 
    dropdown.appendChild(option);
    });
}

function selectOption() {
    const input = document.getElementById('filterInput');
    input.value = this.textContent; 
    const dropdown = document.getElementById('dropdownOptions');
    dropdown.style.display = 'none'; 
    input.blur(); 
}

const input = document.getElementById('filterInput');
input.addEventListener('input', filterOptions);

function filterOptions() {
    const filterValue = this.value.toLowerCase();
    const dropdown = document.getElementById('dropdownOptions');
    const options = Array.from(dropdown.children);
    options.forEach(option => option.style.display = "none"); 

    if (!filterValue) { 
    return; // Do nothing when the input field isn't touched or is empty
    }

    const relevantOptions = options
    .filter(option => option.textContent.toLowerCase().includes(filterValue))
    .sort((option1, option2) => {
        return option1.textContent.toLowerCase().indexOf(filterValue) -
        option2.textContent.toLowerCase().indexOf(filterValue);
    })
    .slice(0, 10); 

    relevantOptions.forEach(option => option.style.display = ""); 

    if (relevantOptions.length > 0) {
    dropdown.style.display = 'block'; 
    }
}
