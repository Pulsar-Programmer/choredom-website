




window.addEventListener("load", function() {
    const form = document.getElementById("signupForm");
    form.addEventListener("submit", function(event) {
        //event.preventDefault();
        
        let password1Field = document.getElementById("password");
        let password2Field = document.getElementById("password2");
        if (password2Field.value !== password1Field.value){
            alert('Passwords do not match. Please try again.');
            event.preventDefault();
        }
        password2Field.disabled = true; // disables the password2 field

        // now submit the form as is
        form.submit();
    });
});




/*
let allTowns = [];

    function populateDropdown(data) {
        allTowns = data;
    }

    function filterOptions() {
        const dropdown = document.getElementById('dropdownOptions');
        dropdown.innerHTML = '';

        let count = 0;
        const filterValue = this.value.toLowerCase();
        allTowns.forEach(town => {
            const optionText = `${town.CITY}, ${town.STATE_NAME}`.toLowerCase();
            if (optionText.includes(filterValue) && count < 10) {
                const option = document.createElement('div');
                option.setAttribute('value', town.ID);
                option.textContent = `${town.CITY}, ${town.STATE_NAME}`;
                dropdown.appendChild(option);
                count++;
            }
        });
        dropdown.style.display = (count > 0) ? 'block' : 'none';
    }

    let selectedOption = null;

    const dropdown = document.getElementById('dropdown');
    dropdown.addEventListener('click', function(e) {
        if (e.target.tagName === 'DIV' && e.target.parentNode.id === 'dropdownOptions') {
            selectedOption = e.target.textContent;
            document.getElementById('location').value = selectedOption;
        }
    });

    fetch('us_cities.json')
        .then(response => response.json())
        .then(data => populateDropdown(data));

    const input = document.getElementById('location');
    input.addEventListener('input', filterOptions);


*/