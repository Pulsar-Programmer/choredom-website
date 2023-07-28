// Attach a submit event listener to the form
document.querySelector('form').addEventListener('submit', async (event) => {
    // Prevent the form from submitting normally
    event.preventDefault();
  
    // Get the email and password from the form
    const email = document.querySelector('input[name="email"]').value;
    const password = document.querySelector('input[name="password"]').value;
  
    // Query the surrealDB for the user
    const user = await surrealDB.getUserByEmail(email);
  
    if (!user) {
      alert('No user found with this email');
      return;
    }
  
    // Compare the provided password with the stored password
    const isMatch = await surrealDB.comparePassword(password, user.password);
  
    if (!isMatch) {
      alert('Incorrect password');
      return;
    }
  
    // If the email and password match, proceed with sign-in
    // This could be redirecting to another page, showing a welcome message, etc.
    alert('Sign in successful');
  });