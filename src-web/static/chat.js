//Fix this to make this chat-compatible (not job)
//Speak w/ Aaron regarding method in which to preform this with HTML
//As in, how does he want the HTML structured?


/// Function to generate the HTML for each chat
function generateChatHTML(chat) {
    return `
        <div class="message">
          <h4>${chat.sender}</h4>
          <p>${chat.msg}</p>
          <p>${chat.timestamp}</p>
        </div>
    `;
}

// Get the post container element
const chatContainer = document.getElementById("chat-box");

// Function to display jobs on the frontend
function displayChats(chatsData) {
//   jobContainer.innerHTML = ``; // we don't need to do this anymore because the chat/receive only gives one
  chatsData.forEach((chat) => {
      const chatHTML = generateChatHTML(chat);
      chatContainer.innerHTML += chatHTML;
  });
}




function yield_chat_history(){
    ///This will be done automatically on Rust's side - make sure to delete this function 
    ///but also understand that it is necessary for the Rust's side
}

///This function must send a chat to the DB but NOT make it appear on the screen.
/// We will be receiving our own chat messages and then adding it to the DOM separately.
function send_chat(){
    const msg = document.getElementById('message-input').value;

    const roomTitle = document.getElementById('room-title').value;

  fetch('/chat/send', {
      method: 'POST',
      headers: {
          'Content-Type': 'application/json',
      },
      body: JSON.stringify({roomTitle, msg}),
  })
  .catch((error) => {
      console.error('Error:', error);
  });
}

///Done. This function works 
function receive_chat(){

    const roomTitle = document.getElementById('room-title').value;

  fetch('/chat/receive', {
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