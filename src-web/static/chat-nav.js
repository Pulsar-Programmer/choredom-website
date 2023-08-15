const chatList = document.querySelector('#chatList');
  
    // Fetch chat data from server
    fetch('/path/to/your/server/endpoint')
      .then(response => response.json())
      .then(chatData => {
        chatData.forEach(chat => {
          const chatBox = document.createElement('div');
          chatBox.classList.add('chat-box');
          chatBox.id = chat.id;
  
          const chatTitle = document.createElement('h3');
          chatTitle.textContent = chat.displayName;
  
          const chatLastMessage = document.createElement('p');
          chatLastMessage.textContent = chat.lastMessage;
  
          const chatButton = document.createElement('button');
          chatButton.classList.add('send-money');
          chatButton.textContent = 'Send Money';
  
          const openChatButton = document.createElement('button');
          openChatButton.classList.add('open-chat');
          openChatButton.textContent = 'Open Chat';
  
          chatBox.appendChild(chatTitle);
          chatBox.appendChild(chatLastMessage);
          chatBox.appendChild(chatButton);
          chatBox.appendChild(openChatButton);
  
          chatList.appendChild(chatBox);
  
          // WebSocket code starts here
          const roomId = chat.id;
          var socket = null;
  
          function connect() {
            disconnect();
  
            const { location } = window;
            const proto = location.protocol.startsWith('https') ? 'wss' : 'ws';
            const wsUri = `${proto}://${location.host}/chat/ws/${roomId}`;
  
            socket = new WebSocket(wsUri);
  
            socket.onopen = () => {
              console.log('Connected to ' + roomId);
            };
  
            socket.onmessage = (ev) => {
              console.log('Received: ' + ev.data);
              chatLastMessage.textContent = ev.data; // Update the last message in the chat box
            };
  
            socket.onclose = () => {
              console.log('Disconnected from ' + roomId);
              socket = null;
            };
          }
  
          function disconnect() {
            if (socket) {
              socket.close();
              socket = null;
            }
          }
  
          openChatButton.addEventListener('click', connect);
          // WebSocket code ends here
        });
      })
      .catch(error => {
        console.error('Error:', error);
      });