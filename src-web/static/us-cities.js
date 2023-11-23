//make sure this script is embedded in defer.



// the html should looks like :
/*





*/

// const input = document.getElementById("us_location_dropdown");
const input = document.getElementById('filterInput');
const dropdown = document.getElementById('dropdownOptions');

input.addEventListener('input', function() {
    fetch("/src-web/assets/us_cities.json")
    .then(response => response.json())
    .then(data => {

        dropdown.children.clear();

        const transformedData = data.map(entry => {
            return `${entry.CITY}, ${entry.STATE_NAME}`;
        });

        const filteredTowns = transformedData.filter(town => town.includes(input.value.toLowerCase())).sort().slice(0, 10);

        let towns = filteredTowns;

        towns.forEach(town => populateDropdown(town))
    });
});

function populateDropdown(text) {
    var option = document.createElement("option");

    option.text = text;

    dropdown.appendChild(option);
}







