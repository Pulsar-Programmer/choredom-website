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
    .then(response => response.json())
    .then(chatsData => {
        console.log('Chats Success:', chatsData);
        displayChats(chatsData);
    })
    .catch((error) => {
        console.error('Error:', error);
    });
}





function expandImage(msg){
    return "";
    //todo!()
    // returns "" if nothing else "<image> .. "
}



/// Function to generate the HTML for each chat
function generateChatHTML(chat) {
    return `<div class="message">
    <h4>${chat.sender}</h4>
    <p>${chat.msg}</p>
    <p>${chat.timestamp}</p>
    ${expandImage(msg)}
    </div>`;
}


const chatContainer = document.getElementById("chat_box");
console.log(chatContainer);

// Function to display jobs on the frontend
function displayChats(chatsData) {
    console.log(chatsData);
  chatsData.forEach((chat) => {
      const chatHTML = generateChatHTML(chat);
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
    .catch((error) => {
        console.error('Error:', error);
    });
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
    .then(response => response.json())
    .then(chatsData => {
        console.log('Chats Success:', chatsData);
        displayChats(chatsData);
    })
    .catch((error) => {
        console.error('Error:', error);
    });
}
//Erase this when doing long polling or the SSEs.
setInterval(receive_chat, 10_000);