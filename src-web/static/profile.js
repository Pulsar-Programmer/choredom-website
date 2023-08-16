window.addEventListener("load", function() {
    let form = document.getElementById('ratingForm');
    form.addEventListener("submit", function(event) {

        form.action = `${window.location.href}/rate`;

        let selectedRating = document.querySelector('input[name="rating"]:checked').value;

        let stars = document.createElement('input');
        stars.setAttribute('type', 'text');
        stars.setAttribute('name', 'stars');
        stars.value = selectedRating;

        form.appendChild(stars);
        
        form.submit();
    });
});


