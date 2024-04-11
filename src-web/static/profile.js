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
});


function prefill_profile(data){
    //display user data:
    document.getElementById("profile_pic").src = data.pfp_url;
    document.getElementById("displayname").innerHTML = "Name: " + data.displayname;
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
        pics.innerHTML += `<img class="limitedpic" src="${url}">`;
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
        <h1>Rating: ${stars}</h1>
        <h2>Poster Username: ${rater}</h2>
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
        // redirect("/success")
        document.getElementById("initial").outerHTML = ``;
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
    .then(_data => {
        redirect("/success")
        // let str = String(data);
        // console.log(data);
        // document.getElementById("AvgRating").innerHTML = "Rating: " + (data.avg_rating === 0 ? "No Rating" : String(data.avg_rating));

        // // let str = data.rater;
        // let reviewsNode = document.getElementById('reviews');
        // let reviews = Array.from(reviewsNode.children);
        // reviews.forEach(review => {
        //     let posterUsername = review.querySelector('.rater').textContent;
        //     if(posterUsername === `Poster Username: ${data.rater}`) {
        //         review.parentNode.removeChild(review);
        //     }
        // });
    })
    .catch(notify);
}


function report_user(){
    if (!confirm("Are you sure you want to report this user?")){
        return;
    }
    let btn = document.getElementById("rpbutton");
    btn.innerHTML = "Submit Report";
    btn.onclick = submit_report;
    var textbox = document.createElement('input');
    textbox.type = "text";
    textbox.id = "reportbox";
    textbox.placeholder = "Enter Report Body...";
    btn.insertAdjacentElement('afterend', textbox);
}

function submit_report(){
    let path = window.location.pathname;
    let pathParts = path.split('/');
    let name = pathParts[pathParts.indexOf('users') + 1];
    let reportbox = document.getElementById("reportbox");
    let msg = reportbox.value;
    //soft check 
    // if(String(msg).trim() === ""){
    //     return alert("Please enter ")
    // }
    let data = {
        name: name,
        msg: msg,
    };
    fetch('/report', {
        method: 'POST', 
        headers: {
            'Content-Type': 'application/json',
        },
        body: JSON.stringify(data),
    })
    .then(handle)
    .then(_ => {
        // reportbox.outerHTML = "";
        // document.getElementById("rpbutton").outerHTML = "";
        // alert("Report successful!");
        redirect("/success")
    })
    .catch(notify);

}