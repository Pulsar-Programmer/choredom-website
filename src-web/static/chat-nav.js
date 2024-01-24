window.addEventListener("load", function() {
    let url = window.location.href.substring(0, window.location.href.indexOf('chat')).trim() + "nav-links";
    fetch(url, {
        method: 'POST', 
        headers: {
            'Content-Type': 'application/json',
        },
    })
    .then(handle)
    .then(data => {
        // console.log(data);
        let dat = Array.from(data);
        if(dat.length == 0) {
            let links_div = document.getElementById("chatList");
            links_div.innerHTML = `<label>You have no chats as of now!</label>`;
        }
        dat.forEach(part => add_to_html(part.room_name));
    })
    .catch(notify);

});

function add_to_html(yapper){
    let url = `/chats/${yapper}`;
    let html = `<div class="link"><a href=${url}>${yapper}</a></div>`;
    let links_div = document.getElementById("chatList");
    links_div.innerHTML += html;
}