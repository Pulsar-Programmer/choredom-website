window.addEventListener("load", function() {

    fetch('/nav-links', {
        method: 'POST', 
        headers: {
            'Content-Type': 'application/json',
        },
    })
    .then(response => response.json())
    .then(data => {
        data.forEach(part => add_to_html(part.room_id));
    })
    .catch((error) => {
        console.error('Error:', error);
    });

});

function add_to_html(yapper){
    let url = `/chats/${yapper}`;
    let html = `<div class="link"><a href=${url}>${yapper}</a></div>`;
    let links_div = document.getElementById("chatList");
    links_div.innerHTML += html;
}



// window.onload = function() {
//     prefill();
// }