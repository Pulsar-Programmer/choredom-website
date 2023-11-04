// Function to generate the HTML for each job
function generateJobHTML(job) {
    let verification_status = job.user.state === "Verified" ? "V" : "Unv";
    return `
        <div class="job">
        <h3>${job.data.title}</h3>
        <h4><a href="/users/${job.user.username}">${job.user.displayname}</a> (${job.user.username}) (${verification_status}erified User)</h4>
        <p>${job.data.body}</p>
        <p>Date of Task: ${job.data.time}</p>
        <p>Price: $${job.data.price / 100.0}</p>
        <a href="/${job.id}">Visit Job Post</a>
        </div>
    `;
}
//<button onclick="initiateChat('${job.user.usernam}', '${currentUserId}')">Apply</button>`


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

function filterJobs() {
    console.log('Filter Jobs function called');
    const titleBodyFilter = document.getElementById("titleBodyFilter").value.toLowerCase();
    const statusFilter = document.getElementById("statusFilter").value;
    const timeFilter = document.getElementById("timeFilter").value;
    const minPriceFilter = Number(document.getElementById("minPriceFilter").value);
    const maxPriceFilter = Number(document.getElementById("maxPriceFilter").value);
    console.log(`titleBodyFilter:${titleBodyFilter} statusFilter:${statusFilter} timeFilter:${timeFilter} minPriceFilter:${minPriceFilter} maxPriceFilter:${maxPriceFilter}`);
    
    // Get all job divs
    let jobDivsHTML = document.getElementsByClassName('job');
    let jobDivs = [...jobDivsHTML];

    // Filter job divs
    jobDivs.forEach((jobDiv) => {
        // Extract job data from the job div
        let jobTitle = jobDiv.getElementsByTagName('h3')[0].innerText.toLowerCase();
        let jobBody = jobDiv.getElementsByTagName('p')[0].innerText.toLowerCase();
        let jobStatus = jobDiv.getElementsByTagName('h4')[0].innerText.includes('V') ? 'Verified' : 'Non-verified';
        let jobTime = formatDate(new Date(jobDiv.getElementsByTagName('p')[1].innerText.replace('Date of Task: ', '')));
        let jobPrice = parseFloat(jobDiv.getElementsByTagName('p')[2].innerText.replace('Price: $', ''));

        console.log(`jobTitle:${jobTitle} jobBody:${jobBody} jobStatus:${jobStatus} jobTime:${jobTime} jobPrice:${jobPrice}`);

        // Check if the job matches the search criteria
        let matchesSearchCriteria = 
            (titleBodyFilter === "" || jobTitle.includes(titleBodyFilter) || jobBody.includes(titleBodyFilter)) &&
            (statusFilter === "" || jobStatus === statusFilter) &&
            (timeFilter === "" || jobTime === timeFilter) &&
            (minPriceFilter === 0 || jobPrice >= minPriceFilter) &&
            (maxPriceFilter === 0 || jobPrice <= maxPriceFilter);
        console.log("Matches:" + matchesSearchCriteria);
        jobDiv.style.display = matchesSearchCriteria ? 'block' : 'none';
    });
    console.log("Test my nigga");
    // I finished Mr. Shatmaster
}

function formatDate(date) {
    let month = date.getMonth() + 1; // getMonth() is zero-based
    let day = date.getDate();
    let year = date.getFullYear();
 
    month = month < 10 ? '0' + month : month;
    day = day < 10 ? '0' + day : day;
 
    return year + '-' + month + '-' + day;
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


function searchPosts() {
    const searchQuery = document.getElementById('searchInput').value.toLowerCase();

    fetch('/job-handling', {
        method: 'POST',
        headers: {
            'Content-Type': 'application/json',
        },
        body: JSON.stringify({ location: searchQuery }),
    })
    .then(response => response.json())
    .then(jobsData => {
        console.log('Jobs Success:', jobsData);

        const filteredJobs = jobsData.filter(job => 
            job.data.title.toLowerCase().includes(searchQuery) || 
            job.data.body.toLowerCase().includes(searchQuery)
        );

        displayJobs(filteredJobs);
    })
    .catch((error) => {
        console.error('Error:', error);
    });
}