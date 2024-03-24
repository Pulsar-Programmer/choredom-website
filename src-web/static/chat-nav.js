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
            links_div.innerHTML += `<p>You have no chats as of now!</p>`;
        }
        dat.forEach(part => add_to_html(part.room_name));
    })
    .catch(notify);

});

function add_to_html(yapper){
    let url = `/chats/${yapper}`;
    let html = `<div class="chat_list_elem"><a href=${url}>${yapper}</a></div>`;
    let links_div = document.getElementById("chatList");
    links_div.innerHTML += html;
}