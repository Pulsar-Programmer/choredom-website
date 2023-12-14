window.addEventListener("load", function() {
    let url = window.location.href.substring(0, window.location.href.indexOf('chat')).trim() + "nav-links";
    fetch(url, {
        method: 'POST', 
        headers: {
            'Content-Type': 'application/json',
        },
    })
    .then(error)
    .then(data => {
        console.log(data);
        data.forEach(part => add_to_html(part.room_name));
    })
    .catch(console_alert_error);

});

function add_to_html(yapper){
    let url = `/chats/${yapper}`;
    let html = `<div class="link"><a href=${url}>${yapper}</a></div>`;
    let links_div = document.getElementById("chatList");
    links_div.innerHTML += html;
}
