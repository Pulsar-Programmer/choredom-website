$(function() {
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
            fetch('/src-web/assets/us_cities.json')
            .then(response => response.json())
            .then(data => {
                callback(data.map(city => ({text: `${city.CITY}, ${city.STATE_NAME}`, value: `${city.CITY}, ${city.STATE_NAME}`})));
            });
        },
        onChange: function(){
            get_location_data();
        },
    });
});

document.addEventListener("DOMContentLoaded", function (event) {
    event.stopPropagation();
    // event.stopImmediatePropagation();
    // event.preventDefault();
    // console.log("YUHH!")
    fetch('/settings/present_data', {
        method: 'POST', 
        headers: {
            'Content-Type': 'application/json',
        },
    })
    .then(handle)
    .then(data => {
        prefill_tia(data.location)
    })
    .catch(notify);
    // prefill_tia(get_settings_location())
});

function prefill_tia(location){
    var selectElement = document.getElementById('city');
    var selectize = selectElement.selectize;
    selectize.addOption({
        text: location,
        value: location
    });
    selectize.setValue(location, true);
    get_location_data();
    // console.log(location, selectize, selectElement)
}



// Function to generate the HTML for each job
function generateJobHTML(job) {
    // console.log(job.id.id.String);
    let verification_status = job.user.state === "Verified" ? "V" : "Unv";
    return `
        <div class="stdbox job">
            <div class="pfp">
                <img src="${job.user.page.pfp_url}">
                <span><a href="/users/${job.user.username}">${job.user.displayname}</a> (${job.user.username}) <span class="jobStatus">(${verification_status}erified User)</span></span>
            </div>
            <h3 class="jobTitle">${job.data.title}</h3>
            <p class="jobBody">${job.data.body}</p>
            <p class="jobTime">Date of Task: ${job.data.time}</p>
            <p class="jobPrice">Price: $${job.data.price / 100.0}</p>
            <a class="btna embedb" href="/jobs/${job.id.key.String}">Visit Job Post</a>
            <a class="btna embedb" href="/chats/${job.user.username}">Open Chat</a>
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
        let jobTitle = jobDiv.getElementsByClassName("jobTitle")[0].innerText.toLowerCase();
        let jobBody = jobDiv.getElementsByClassName("jobBody")[0].innerText.toLowerCase();
        let jobStatus = jobDiv.getElementsByClassName("jobStatus")[0].innerText.includes('V') ? 'Verified' : 'Non-verified';
        let jobTime = formatDate(new Date(jobDiv.getElementsByClassName("jobTime")[0].innerText.replace('Date of Task: ', '')));
        let jobPrice = parseFloat(jobDiv.getElementsByClassName("jobPrice")[0].innerText.replace('Price: $', ''));

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
    const location = document.getElementById('city').value;

    fetch('/job-handling', {
        method: 'POST',
        headers: {
            'Content-Type': 'application/json',
        },
        body: JSON.stringify(location),
    })
    .then(handle)
    .then(jobsData => {
        console.log('Location Success:', jobsData);

        console.log('Jobs Success:', jobsData);
        displayJobs(jobsData);
    })
    .catch(notify);
}










// Function to toggle dark mode
// document.getElementById('toggle-button').addEventListener('click', function() {
//     document.body.classList.toggle('dark-mode');
// });

function searchPosts() {
    const searchQuery = document.getElementById('searchInput').value.toLowerCase();

    fetch('/job-handling', {
        method: 'POST',
        headers: {
            'Content-Type': 'application/json',
        },
        body: JSON.stringify({ searchQuery }),
    })
    .then(handle)
    .then(jobsData => {
        console.log('Jobs Success:', jobsData);

        const filteredJobs = jobsData.filter(job => 
            job.data.title.toLowerCase().includes(searchQuery) || 
            job.data.body.toLowerCase().includes(searchQuery)
        );

        displayJobs(filteredJobs);
    })
    .catch(notify);
}