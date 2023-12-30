//Fix this to make this chat-compatible (not job)
//Speak w/ Aaron regarding method in which to preform this with HTML
//As in, how does he want the HTML structured?


let urlbase = window.location.href.substring(0, window.location.href.indexOf('chats')).trim();

window.onload = function() {
    var path = window.location.pathname;
    var pathParts = path.split('/');
    var newPath = pathParts[pathParts.indexOf('chats') + 1];
    let url = urlbase + "chats_obtain";
    // console.log(url);
    // console.log(newPath);
    document.getElementById("room-title").innerHTML = newPath;
    fetch(url, {
        method: 'POST',
        headers: {
            'Content-Type': 'application/json',
        },
        body: JSON.stringify(newPath),
    })
    .then(handle)
    .then(chatsData => {
        console.log('Chats Success:', chatsData);
        displayChats(chatsData);
    })
    .catch(notify);
}



/// Function to generate the HTML for each chat
function generateChatHTML(chat, pfpurl) {
    return `<div class="message">
    <img src=${pfpurl} width="500" height="500">
    <h4><a href="/users/${chat.sender}">${chat.sender}</a></h4>
    <p>${chat.timestamp}</p>
    <p>${expandImages(chat.msg)}</p>
    </div>`;
}


let chatContainer = document.getElementById('chat_box');

// Function to display jobs on the frontend
function displayChats(chatsData) {
    console.log(chatsData);
    // chatsData = Array.from(chatsData);
    // console.log(chatsData);
  chatsData.data.forEach((chat) => {
      const chatHTML = generateChatHTML(chat, chatsData.pfpurl);
      console.log(chatHTML);
      chatContainer.innerHTML += chatHTML;
  });
}

///This function must send a chat to the DB but NOT make it appear on the screen.
/// We will be receiving our own chat messages and then adding it to the DOM separately.
/// ^ RETHINK THIS MAYBE. On iMessages, can't you see an unsent message? Maybe it is better to show it sent and then maybe we can do something like how they do it.
function send_chat(){
    const msg = document.getElementById('message-input').value;

    const room_title = document.getElementById('room-title').innerHTML;
    console.log("Chat sending..")
    fetch('/chat/send', {
        method: 'POST',
        headers: {
            'Content-Type': 'application/json',
        },
        body: JSON.stringify({room_title, msg}),
    })
    .then(handle)
    .then(chat => {
        console.log('Chats Bounceback Success:', chat);
        const chatHTML = generateChatHTML(chat);
        console.log(chatHTML);
        chatContainer.innerHTML += chatHTML;
    })
    .catch(notify);
    // chat = {msg: msg, username: room_title, timestamp: }
    // generateChatHTML()

}

///Done. This function works 
function receive_chat(){
    const roomSplitter = window.location.href.split('/').filter((val) => {
        return ! (val == "" || val == null);
    });
    const roomTitle = roomSplitter[roomSplitter.length - 1];
    // let url = urlbase + "chat/receive";
    fetch("/chat/receive", {
        method: 'POST',
        headers: {
            'Content-Type': 'application/json',
        },
        body: JSON.stringify(roomTitle),
    })
    .then(handle)
    .then(chatsData => {
        console.log('Chats Success:', chatsData);
        displayChats(chatsData);
    })
    .catch(notify);
}
//Erase this when doing long polling or the SSEs.
// setInterval(receive_chat, 10_000);

var eventSource = new EventSource("/chat-updates");

eventSource.onmessage = function(event) {
    if(event.data === "UPDATE"){
        receive_chat();
    } 
};


window.addEventListener('beforeunload', function (e) {
    eventSource.close();
}); 









function upload_chats(){
    const fileInputElement = document.getElementById("file_upload_chats");
    let formData = new FormData();
    for(f of fileInputElement.files){
        formData.append('file', f, 'filename.jpg');
    }
    var path = window.location.pathname;
    var pathParts = path.split('/');
    var opposite_chatter = pathParts[pathParts.indexOf('chats') + 1];
    formData.append('opposite_chatter', JSON.stringify(opposite_chatter));
    fetch('/pics-chats', {
        method: 'POST',
        body: formData,
    })
    .then(handle)
    .then(data => {
        
    })
    .catch(notify);
}