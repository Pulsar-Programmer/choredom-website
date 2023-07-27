window.onload = function() {
  const select = document.querySelector('select[name="Cities"]');
  const filterInput = document.querySelector('#city-filter');

  fetch('./us_cities.json')
    .then(response => response.json())
    .then(cities => {
      cities.forEach((city) => {
        const option = document.createElement('option');
        option.value = city.CITY;
        option.textContent = city.CITY;
        select.appendChild(option);
      });
    });

  filterInput.addEventListener('input', (event) => {
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