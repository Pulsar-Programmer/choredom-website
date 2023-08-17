// const $status = document.querySelector('#status')
// const $connectButton = document.querySelector('#connect')
// const $log = document.querySelector('#log')
// const $form = document.querySelector('#chatform')
// const $input = document.querySelector('#text')

// /** @type {WebSocket | null} */
// var socket = null

// function log(msg, type = 'status') {
// $log.innerHTML += `<p class="msg msg--${type}">${msg}</p>`
// $log.scrollTop += 1000
// }

// function connect() {
// disconnect()

// const { location } = window

// const proto = location.protocol.startsWith('https') ? 'wss' : 'ws'
// const wsUri = `${proto}://${location.host}/chat/ws`

// log('Connecting...')
// socket = new WebSocket(wsUri)

// socket.onopen = () => {
//     log('Connected')
//     updateConnectionStatus()
// }

// socket.onmessage = (ev) => {
//     log('Received: ' + ev.data, 'message')
// }

// socket.onclose = () => {
//     log('Disconnected')
//     socket = null
//     updateConnectionStatus()
// }
// }

// function disconnect() {
// if (socket) {
//     log('Disconnecting...')
//     socket.close()
//     socket = null

//     updateConnectionStatus()
// }
// }

// function updateConnectionStatus() {
// if (socket) {
//     $status.style.backgroundColor = 'transparent'
//     $status.style.color = 'green'
//     $status.textContent = `connected`
//     $connectButton.innerHTML = 'Disconnect'
//     $input.focus()
// } else {
//     $status.style.backgroundColor = 'red'
//     $status.style.color = 'white'
//     $status.textContent = 'disconnected'
//     $connectButton.textContent = 'Connect'
// }
// }

// $connectButton.addEventListener('click', () => {
// if (socket) {
//     disconnect()
// } else {
//     connect()
// }

// updateConnectionStatus()
// })

// $form.addEventListener('submit', (ev) => {
//     ev.preventDefault()

//     const text = $input.value

//     log('Sending: ' + text)
//     socket.send(text)

//     $input.value = ''
//     $input.focus()
// })

// updateConnectionStatus()


// Create a WebSocket connection to the server
let socket = new WebSocket("ws://your-websocket-server.com");

// Get the HTML elements
let messageBox = document.getElementById('messageBox');
let messageInput = document.getElementById('messageInput');
let sendButton = document.getElementById('sendButton');

// When a message is received from the server, append it to the message box
socket.onmessage = function(event) {
    let newMessage = document.createElement('p');
    newMessage.textContent = event.data;
    messageBox.appendChild(newMessage);
};

// When the send button is clicked, send the current message to the server
sendButton.addEventListener('click', function() {
    let message = messageInput.value;
    socket.send(message);
    messageInput.value = '';
});

// Add an event listener for the WebSocket connection opening
socket.onopen = function(event) {
    console.log('Connection opened');
};

// Add an event listener for the WebSocket connection closing
socket.onclose = function(event) {
    console.log('Connection closed');
};

// Add an event listener for any errors that occur on the WebSocket
socket.onerror = function(error) {
    console.log('Error occurred:', error);
};
