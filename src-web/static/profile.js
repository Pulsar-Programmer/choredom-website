window.addEventListener("load", function() {
    let form = document.getElementById('ratingForm');
    form.addEventListener("submit", function(event) {
        event.preventDefault();

        form.action = `${window.location.href}/rate`;

        let selectedRating = document.querySelector('input[name="rating"]:checked').value;

        let stars = document.createElement('input');
        stars.setAttribute('type', 'text');
        stars.setAttribute('name', 'stars');
        stars.value = selectedRating;

        form.appendChild(stars);

        // Create new div for the review
        let review = document.createElement('div');
        review.className = 'review';

        // Create paragraph for the review text
        let reviewText = document.createElement('p');
        reviewText.textContent = form.elements['body'].value;
        review.appendChild(reviewText);

        // Create paragraph for the review rating
        let reviewRating = document.createElement('p');
        reviewRating.textContent = `Rating: ${selectedRating} stars`;
        review.appendChild(reviewRating);

        // Append the review to the document body
        document.body.appendChild(review);

        form.reset();
    });
});
