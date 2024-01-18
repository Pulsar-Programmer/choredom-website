//make sure this script is embedded in defer.



// the html should looks like :
/*

<div id="dropdown" class="dropdown">
    <input id="filterInput" type="text" placeholder="Filter towns...">
    <select id="dropdownOptions">
    </select>
  </div>



*/

$(function() {
    $('#city').selectize({
        options: [],
        items: [],
        render: {
            option: function(data, escape) {
                return '<div>' + escape(data.text) + '</div>';
            },
            item: function(data, escape) {
                return '<div>' + escape(data.text) + '</div>';
            }
        },
        load: function(query, callback) {
            if (!query.length) return callback();
            fetch('./src-web/assets/us_cities.json')
            .then(response => response.json())
            .then(data => {
                callback(data.map(city => ({text: `${city.CITY}, ${city.STATE_NAME}`, value: `${city.CITY}, ${city.STATE_NAME}`})));
            });
        },
        // preload: true,
    });
});
