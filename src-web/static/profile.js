window.addEventListener("load", function() {
    // let urlbase = window.location.href.substring(0, window.location.href.indexOf('users')).trim();
    var path = window.location.pathname;
    var pathParts = path.split('/');
    var newPath = pathParts[pathParts.indexOf('users') + 1];
    // let url = urlbase + "obtain_profile";
    
    fetch('/obtain_profile', {
        method: 'POST', 
        headers: {
            'Content-Type': 'application/json',
        },
        body: JSON.stringify(newPath),
    })
    .then(response => response.json())
    .then(data => {
        prefill_profile(data);
    })
    .catch((error) => {
        console.error('Error:', error);
    });
    // yuh cuh fella
});


function prefill_profile(data){
    //display user data:
    document.getElementById("profile_pic").href = data.pfp_url;
    document.getElementById("displayName").innerHTML = "Name: " + data.displayname;
    document.getElementById("username").innerHTML = "Username: " + data.username;
    document.getElementById("AvgRating").innerHTML = "Rating: " + (data.avg_rating === 0 ? "No Rating" : String(data.avg_rating));
    document.getElementById("CreationDate").innerHTML = "Joined: " + data.creation_date;
    document.getElementById("state").innerHTML = data.state;
    document.getElementById("bio").innerHTML = data.bio;

    //display rater data
    data.reviews.forEach(review => {
        displayRatingHTML(review.stars, review.rater, review.body);
    });
}


function displayRatingHTML(stars, rater, body){
    let html = `<div id="review">
        <h1 id="Rating">Rating: ${stars}</h1>
        <h2>Poster Username: ${rater}</h2>
        <p>${body}</p>
    </div>`;
    document.getElementById("reviews").appendChild(html);
}

function submitReviewForm(){

    let selectedRating = document.querySelector('input[name="rating"]:checked').value;
    let body = document.getElementById("body");

    let value = {
        stars: selectedRating,
        body: body,
    };
    
    const url = `${window.location.href}/rate` // this url might be incorrect
    fetch(url, {
        method: 'POST', 
        headers: {
            'Content-Type': 'application/json',
        },
        body: value,
    })
    .then(response => response.json())
    .then(data => { //send data back to self for quick display?
        // prefill_profile(data);
    })
    .catch((error) => {
        console.error('Error:', error);
    });

}


function delete_rating(){
    const url = `${window.location.href}/rate/delete` // this url might be incorrect
    fetch(url, {
        method: 'POST', 
        headers: {
            'Content-Type': 'application/json',
        },
        body: value,
    })
    .then(response => response.json())
    .then(data => {

    })
    .catch((error) => {
        console.error('Error:', error);
    });
}