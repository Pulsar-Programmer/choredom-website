const cities = require('./us-cities.json');

const select = document.querySelector('select[name="Cities"]');

cities.forEach((city) => {
  const option = document.createElement('option');
  option.value = city.name;
  option.textContent = city.name;
  select.appendChild(option);
});
