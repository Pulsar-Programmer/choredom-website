let currentURL = window.location.href.slice(7);
get_profile_data();


// Get the post container element
const userContainer = document.getElementById("profile");

// Function to display jobs on the frontend
function displayProfile(userData) {
    let displayName = document.getElementById("displayName");
    displayName.innerHTML = userData.displayname;

    let username = document.getElementById("username");
    username.innerHTML = userData.username;

    let AvgRating = document.getElementById("AvgRating");
    AvgRating.innerHTML = userData.average_rating;

    let CreationDate = document.getElementById("CreationDate");
    CreationDate.innerHTML = userData.time;

    let State = document.getElementById("State");
    State.innerHTML = userData.state;

    let bio = document.getElementById("bio");
    bio.innerHTML = userData.bio;

    userContainer.innerHTML = userHTML;
}

function get_profile_data(){
    fetch(`/users/${currentURL}`, {
        method: 'POST',
        headers: {
            'Content-Type': 'application/json',
        },
    })
    .then(response => response.json())
    .then(userData => {
        displayProfile(userData);
    })
    .catch((error) => {
        console.error('Error:', error);
    });
}







//Reviews shall be handled later.


//rating



// Handle review form submission
document.getElementById('reviewForm').addEventListener('submit', function(event) {
    event.preventDefault();
    // Get the rating and review from the form
    var rating = document.getElementById('rating').value;
    var review = document.getElementById('review').value;

    // Prepare the data to send to the backend
    var data = {
        rating: rating,
        review: review
    };

    // Make an AJAX request to the backend
    var xhr = new XMLHttpRequest();
    xhr.open('POST', '/your-backend-endpoint', true);
    xhr.setRequestHeader('Content-Type', 'application/json');
    xhr.onreadystatechange = function() {
        if (xhr.readyState === XMLHttpRequest.DONE && xhr.status === 200) {
            alert('Your review has been submitted!');
            // Refresh the reviews after submitting the form
            fetchReviews();
        }
    };
    xhr.send(JSON.stringify(data));
});

// Function to fetch and display the reviews
function fetchReviews() {
    // Simulating fetching reviews from the backend
    var reviews = [
        { rating: 5, review: "This user is really helpful!" },
        { rating: 4, review: "Good user, but could be more active." },
        { rating: 3, review: "Average user." }
    ];
    // Get the reviews container element
    var reviewsContainer = document.getElementById('reviewsContainer');

    // Clear the existing reviews
    reviewsContainer.innerHTML = '';

    // Loop through the reviews and create review elements
    for (var i = 0; i < reviews.length; i++) {
        var review = reviews[i];

        // Create a div for the review
        var reviewDiv = document.createElement('div');
        reviewDiv.classList.add('review');

        // Create a heading for the review
        var heading = document.createElement('h3');
        heading.textContent = 'Rating: ' + review.rating;
        reviewDiv.appendChild(heading);

        // Create a paragraph for the review text
        var reviewText = document.createElement('p');
        reviewText.classList.add('review-text');
        reviewText.textContent = review.review;
        reviewDiv.appendChild(reviewText);

        // Append the review div to the reviews container
        reviewsContainer.appendChild(reviewDiv);
    }
}

// Call the fetchReviews function to initially display the reviews
fetchReviews();




// // Get the button
// var button = document.getElementById("toggleButton");
        
// // When the user clicks on the button, toggle between light and dark modes
// button.onclick = function() {
//     var body = document.body;
//     body.classList.toggle("dark-mode");
// }