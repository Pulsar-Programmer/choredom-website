//Fix this to make this chat-compatible (not job)
//Speak w/ Aaron regarding method in which to preform this with HTML
//As in, how does he want the HTML structured?


// Function to generate the HTML for each job

function generateJobHTML(job) {
  return `
      <div class="job">
      <h3>${job.data.title}</h3>
      <h4><a href="/users/${job.user.username}">${job.user.displayname}</a> (${job.user.username})</h4>
      <p>${job.data.body}</p>
      <p>Date and Time: ${job.data.time}</p>
      <p>Price: $${job.data.price}</p>
      <a href="/${job.id}">Visit Job Post</a>
      </div>
  `;
}
//<button onclick="initiateChat('${job.user.usernam}', '${currentUserId}')">Apply</button>`

// function to start chat
function initiateChat(user1, user2) {
  // Send a request to the server to create a chat room
  fetch('/create-chat-room', {
      method: 'POST',
      headers: {
      'Content-Type': 'application/json',
      },
      body: JSON.stringify({
          user1: user1, 
          user2: user2
      }),
  })
  .then(response => response.json())
  .then(roomId => {
      // Redirect the user to the chat room using the generated room ID
      let url = `/chats/room=${roomId}`;
      window.location.href = url;
  })
  .catch((error) => {
      console.error('Error:', error);
  });
}


// Get the post container element
const jobContainer = document.getElementById("job-container");

// Function to display jobs on the frontend
function displayJobs(jobsData) {
  jobContainer.innerHTML = ``;
  jobsData.forEach((job) => {
      const jobHTML = generateJobHTML(job);
      jobContainer.innerHTML += jobHTML;
  });
}

function get_location_data(){
  const location = document.getElementById('filterInput').value;

  fetch('/job-handling', {
      method: 'POST',
      headers: {
          'Content-Type': 'application/json',
      },
      body: JSON.stringify(location),
  })
  .then(response => response.json())
  .then(jobsData => {
      console.log('Location Success:', jobsData);
      
      console.log('Jobs Success:', jobsData);
      displayJobs(jobsData);
  })
  .catch((error) => {
      console.error('Error:', error);
  });
}