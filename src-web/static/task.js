// document.addEventListener("DOMContentLoaded", function(){
//     document.getElementById('city').addEventListener('change', function(event){
//         // event.stopPropagation();
//         // event.stopImmediatePropagation();
//         // event.preventDefault();
//         console.log("Hello!?");
//         get_location_data();
//     });
// });
// function setupEventListener() {
//     document.getElementById('city').addEventListener('change', get_location_data);
//     console.log("Hello!??");
// }
// setupEventListener();

// if (document.readyState === "loading") {
//     // Document is still loading, add the event listener when it loads
//     document.addEventListener("DOMContentLoaded", setupEventListener);
// } else {
//     // Document has already loaded, set up the event listener immediately
//     setupEventListener();
// }
   
// $('#city').selectize({
//     onChange: function(value) {
//         this.$input[0].dispatchEvent(new Event("change"));
//     }
// });

// document.getElementById('city').selectize.on('change', function() {
//     get_location_data();
// });

// document.addEventListener("DOMContentLoaded", function(event){
//     event.stopPropagation();
//     // event.stopImmediatePropagation();
//     // event.preventDefault();
//     console.log("YUHH!")
//     document.getElementById('city').selectize.on('change', function() {
//         get_location_data();
//     });
// });

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
        // onchange: get_location_data,
        onChange: function(){
            get_location_data();
        },
        // preload: true,
    });
});



// Function to generate the HTML for each job
function generateJobHTML(job) {
    // console.log(job.id.id.String);
    let verification_status = job.user.state === "Verified" ? "V" : "Unv";
    return `
        <div class="job">
        <h3>${job.data.title}</h3>
        <img src="${job.user.page.pfp_url}" height="500" width="500">
        <h4><a href="/users/${job.user.username}">${job.user.displayname}</a> (${job.user.username}) (${verification_status}erified User)</h4>
        <p>${job.data.body}</p>
        <p>Date of Task: ${job.data.time}</p>
        <p>Price: $${job.data.price / 100.0}</p>
        <a href="/jobs/${job.id.id.String}">Visit Job Post</a>
        <a href="/chats/${job.user.username}">Open Chat</a>
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
        body: JSON.stringify({ location: searchQuery }),
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