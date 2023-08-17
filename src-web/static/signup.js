
window.addEventListener("load", function() {
    const form = document.getElementById("signupForm");
    form.addEventListener("submit", function(event) {

        let password1Field = document.getElementById("password");
        let password2Field = document.getElementById("password2");
        if (password2Field.value !== password1Field.value){
            alert('Passwords do not match. Please try again.');
            // event.preventDefault();
            // return
        }
        password2Field.disabled = true; // disables the password2 field

        // now submit the form as is
        form.submit();
    });
});


// window.addEventListener("load", function() {
//     const form = document.getElementById("signupForm");
//     form.addEventListener("submit", function(event) {

//         let password1Field = document.getElementById("password");
//         let password2Field = document.getElementById("password2");
//         if (password2Field.value !== password1Field.value){
//             alert('Passwords do not match. Please try again.');
//             event.preventDefault(); // prevents the form from submitting
//         } else {
//             password2Field.disabled = true; // disables the password2 field

//             // now submit the form as is
//             form.submit();
//         }
//     });
// });


// Include the JavaScript code here...
fetch('/us_cities.json')
.then(response => response.json())
.then(data => populateDropdown(data));

function populateDropdown(data) {
const dropdown = document.getElementById('dropdownOptions');
dropdown.style.display = 'none'; // Hide the dropdown initially
    data.forEach(town => {
    const option = document.createElement('div');
    option.setAttribute('value', town.ID);
    option.textContent = `${town.CITY}, ${town.STATE_NAME}`; 
    option.addEventListener('click', selectOption); 
    dropdown.appendChild(option);
    });
}

function selectOption() {
    const input = document.getElementById('filterInput');
    input.value = this.textContent; 
    const dropdown = document.getElementById('dropdownOptions');
    dropdown.style.display = 'none'; 
    input.blur(); 
}

const input = document.getElementById('filterInput');
input.addEventListener('input', filterOptions);

function filterOptions() {
    const filterValue = this.value.toLowerCase();
    const dropdown = document.getElementById('dropdownOptions');
    const options = Array.from(dropdown.children);
    options.forEach(option => option.style.display = "none"); 

    if (!filterValue) { 
        return; // Do nothing when the input field isn't touched or is empty
    }

    const relevantOptions = options
    .filter(option => option.textContent.toLowerCase().includes(filterValue))
    .sort((option1, option2) => {
        return option1.textContent.toLowerCase().indexOf(filterValue) -
        option2.textContent.toLowerCase().indexOf(filterValue);
    })
    .slice(0, 10); 

    relevantOptions.forEach(option => option.style.display = ""); 

    if (relevantOptions.length > 0) {
        dropdown.style.display = 'block'; 
    }
}