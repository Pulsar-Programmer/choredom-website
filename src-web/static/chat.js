
var button = document.getElementById('send-message');
button.addEventListener('click', function(event) {
    // event.preventDefault();
    var input = document.getElementById('message');
    send_message(input.value)
    input.value = '';

    receive_messages();
});

// Function to generate the HTML for each chat
function generateMessageHTML(msg) {
    // Return the HTML string
    return `
        <div class="chat-message">
            <div class="message-info">
                <span class="sender">${msg.sender}</span>
                <span class="time">${msg.time}</span>
            </div>
            <div class="message-text">${msg.message}</div>
        </div>g
    `;
}

// Function to display jobs on the frontend
function displayMessages(msgData) {
    const chatContainer = document.getElementById("chat-box");

    chatContainer.innerHTML = ``;
    msgData.forEach((msg) => {
        const msgHTML = generateMessageHTML(msg);
        chatContainer.innerHTML += msgHTML;
    });
}

function send_message(msg){
    fetch('/send-message', {
        method: 'POST',
        headers: {
            'Content-Type': 'application/json',
        },
        body: JSON.stringify(msg),
    })
    .then(response => response.json())
    .then(ok => {
        console.log('Message Success:', ok);
    })
    .catch((error) => {
        console.error('Error:', error);
    });
}


function receive_messages(){
    fetch('/receive_messages', {
        method: 'GET',
        headers: {
            'Content-Type': 'application/json',
        },
    })
    .then(response => response.json())
    .then(msgs => {
        displayMessages(msgs);
    })
    .catch((error) => {
        console.error('Error:', error);
    });
}



//FOR EACH CHAT ROOM
//-> Message from User
//<- Vector of Messages









