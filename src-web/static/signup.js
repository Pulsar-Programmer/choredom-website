window.onload = function() {
  // Load the JSON file
  const cities = require('./us_cities.json');

  // Get the select element
  const select = document.querySelector('select[name="Cities"]');

  // Populate the select options
  cities.forEach((city) => {
    const option = document.createElement('option');
    option.value = city.CITY;
    option.textContent = city.CITY;
    select.appendChild(option);
  });

  // Filter the options as the user types
  select.addEventListener('input', (event) => {
    const filter = event.target.value.toLowerCase();
    const options = select.querySelectorAll('option');

    for (const option of options) {
      const text = option.textContent.toLowerCase();

      if (text.indexOf(filter) === -1) {
        option.style.display = 'none';
      } else {
        option.style.display = 'block';
      }
    }
  });

  // Ensure that all inputs are filled before moving on
  const form = document.querySelector('form');

  form.addEventListener('submit', (event) => {
    const inputs = form.querySelectorAll('input');
    let allInputsFilled = true;

    for (const input of inputs) {
      if (input.value === '') {
        allInputsFilled = false;
        break;
      }
    }

    if (!allInputsFilled) {
      event.preventDefault();
      alert('Please fill in all of the inputs before submitting.');
    }
  });
};