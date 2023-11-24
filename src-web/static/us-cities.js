//make sure this script is embedded in defer.



// the html should looks like :
/*

<div id="dropdown" class="dropdown">
    <input id="filterInput" type="text" placeholder="Filter towns...">
    <select id="dropdownOptions">
    </select>
  </div>



*/




// // const input = document.getElementById("us_location_dropdown");
// const input = document.getElementById('filterInput');
// const dropdown = document.getElementById('dropdownOptions');

// input.addEventListener('input', function() {
//     fetch("/src-web/assets/us_cities.json")
//     .then(response => response.json())
//     .then(data => {
//         // dropdown.innerHTML = "";
//         dropdown.children.clear();

//         const transformedData = data.map(entry => {
//             return `${entry.CITY}, ${entry.STATE_NAME}`;
//         });

//         const filteredTowns = transformedData.filter(town => town.includes(input.value.toLowerCase())).sort().slice(0, 10);

//         let towns = filteredTowns;

//         towns.forEach(town => populateDropdown(town))
//     });
// });

// function populateDropdown(text) {
//     var option = document.createElement("option");

//     option.text = text;

//     dropdown.appendChild(option);
// }




window.onload = function() {
    fetch('./src-web/assets/us_cities.json')
    .then(response => response.json())
    .then(data => {
        when_data(data);
    });


    function when_data(data){
        console.log(data);
        const selectElement = document.getElementById('city');
        data.forEach(city => {
            const optionElement = document.createElement('option');
            // optionElement.value = city.ID; //maybe one day make it coded? not very efficiency anyway tho
            optionElement.value = `${city.CITY}, ${city.STATE_NAME}`;
            optionElement.text = `${city.CITY}, ${city.STATE_NAME}`;
            selectElement.appendChild(optionElement);
        });
    }
};   

