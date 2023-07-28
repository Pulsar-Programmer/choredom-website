let url = 'http://localhost:8080/settings-post';
let data = {
    username: 'username', 
    password: 'password',
    displayname: 'displayname',
    location: 'location',
    bio: 'bio',
    pfp: 'pfp'
};
fetch(url, {
    method: 'POST', 
    headers: {
        'Content-Type': 'application/json',
    },
    body: JSON.stringify(data), 
})
.then(response => response.json())
.then(data => console.log(data))
.catch((error) => {
  console.error('Error:', error);
});