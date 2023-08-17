// Array of WebSocket URLs for the chats
let chatUrls = ["ws://chat1-websocket-server.com", "ws://chat2-websocket-server.com", "ws://chat3-websocket-server.com"];

// Get the chat list HTML element
let chatList = document.getElementById('chatList');

// Create a WebSocket connection and a chat box for each chat
for(let url of chatUrls) {
    // Create a WebSocket connection
    let socket = new WebSocket(url);

    // Create a chat box
    let chatBox = document.createElement('div');
    chatBox.textContent = 'Connecting...';
    chatBox.style.border = '1px solid black';
    chatBox.style.margin = '10px';
    chatList.appendChild(chatBox);

    // When a message is received, update the chat box
    socket.onmessage = function(event) {
        chatBox.textContent = event.data;
    };
}
