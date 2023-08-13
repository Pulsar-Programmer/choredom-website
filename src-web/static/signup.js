let allTowns = [];
function populateDropdown(data) {
    allTowns = data;
}
function filterOptions() {
    const dropdown = document.getElementById('dropdownOptions');
    dropdown.innerHTML = '';
    let count = 0;
    const filterValue = document.getElementById('filterInput').value.toLowerCase();
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
fetch('us_cities.json')
    .then(response => response.json())
    .then(data => populateDropdown(data));
const input = document.getElementById('filterInput');
input.addEventListener('input', filterOptions);
input.addEventListener('focus', function() {
    document.getElementById('dropdownOptions').style.display = 'block';
});
input.addEventListener('blur', function() {
    setTimeout(function() {
        document.getElementById('dropdownOptions').style.display = 'none';
    }, 200);
});

window.addEventListener("load", function() {
    const form = document.getElementById("signupForm");
    form.addEventListener("submit", function(event) {
        let password1Field = document.getElementById("password");
        let password2Field = document.getElementById("password2");
        if (password2Field.value !== password1Field.value){
            alert('Passwords do not match. Please try again.');
            event.preventDefault();
        }
        password2Field.disabled = true; // disables the password2 field
    });
});