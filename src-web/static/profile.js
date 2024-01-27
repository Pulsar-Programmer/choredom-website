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
    .then(handle)
    .then(data => {
        prefill_profile(data);
    })
    .catch(notify);
    // yuh cuh fella
});


function prefill_profile(data){
    //display user data:
    document.getElementById("profile_pic").src = data.pfp_url;
    document.getElementById("displayName").innerHTML = "Name: " + data.displayname;
    document.getElementById("username").innerHTML = "Username: " + data.username;
    document.getElementById("AvgRating").innerHTML = "Rating: " + (data.avg_rating === 0 ? "No Rating" : String(data.avg_rating));
    document.getElementById("CreationDate").innerHTML = "Joined: " + data.creation_date;
    document.getElementById("state").innerHTML = data.state;
    document.getElementById("bio").innerHTML = data.bio;
    const pics = document.getElementById("pics");
    data.bio_imgs.forEach(url => {
        if(url === null || url === undefined || url === ""){
            return;
        }
        pics.innerHTML += `<img src="${url}">`;
    });

    let reviews = Array.from(data.reviews);
    if(reviews.length == 0){
        document.getElementById("reviews").innerHTML = `<p id="initial"> This user has no reviews.</p>`;
    }
    //display rater data
    reviews.forEach(review => {
        displayRatingHTML(review.stars, review.rater, review.body);
    });
}


function displayRatingHTML(stars, rater, body){
    let html = `<div id="review">
        <h1 class="rating">Rating: ${stars}</h1>
        <h2 class="rater">Poster Username: ${rater}</h2>
        <p>${body}</p>
    </div>`;
    document.getElementById("reviews").innerHTML += html;
}

function submitReviewForm(){

    let selectedRating = document.querySelector('input[name="rating"]:checked');
    //get the number of stars the user has selected from the HTML.
    if(selectedRating.value === 0 || selectedRating.value === null){
        return;
    }
    let body = document.getElementById("body").value;

    let value = {
        stars: Number(selectedRating.value),
        body: body,
    };
    
    fetch(`${window.location.href}/rate`, {
        method: 'POST', 
        headers: {
            'Content-Type': 'application/json',
        },
        body: JSON.stringify(value),
    })
    .then(handle)
    .then(data => {
        document.getElementById("initial").outerHTML = ``; //this should work shatmaster
        displayRatingHTML(data.stars, data.rater, data.body);
    })
    .catch(notify);

}


function delete_rating(){
    const url = `${window.location.href}/rate/delete` // this url might be incorrect
    fetch(url, {
        method: 'POST', 
        headers: {
            'Content-Type': 'application/json',
        },
    })
    .then(handle)
    .then(data => {
        // let str = String(data);
        // console.log(data);
        document.getElementById("AvgRating").innerHTML = "Rating: " + (data.avg_rating === 0 ? "No Rating" : String(data.avg_rating));

        // let str = data.rater;
        let reviewsNode = document.getElementById('reviews');
        let reviews = Array.from(reviewsNode.children);
        reviews.forEach(review => {
            let posterUsername = review.querySelector('.rater').textContent;
            if(posterUsername === `Poster Username: ${data.rater}`) {
                review.parentNode.removeChild(review);
            }
        });
    })
    .catch(notify);
}