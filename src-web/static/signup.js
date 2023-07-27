const cities = require('./us-cities.json');

const select = document.querySelector('select[name="Cities"]');

cities.forEach((city) => {
  const option = document.createElement('option');
  option.value = city.name;
  option.textContent = city.name;
  select.appendChild(option);
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
