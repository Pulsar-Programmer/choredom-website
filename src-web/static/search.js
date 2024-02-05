// Define the function
function handleSearchKeyDown(event) {
  if (event.key === 'Enter') {
    event.preventDefault();
    var searchValue = document.getElementById('search').value;
    // Perform the search and redirect to the appropriate page
    // For example, if the search value is "Home", redirect to the homepage
    if (searchValue === 'Home') {
      window.location.href = '/';
    }
  }
}

// Attach the event listener to the input element
document.getElementById('search').addEventListener('keydown', handleSearchKeyDown);
