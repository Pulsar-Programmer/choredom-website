window.onload = function() {
    var path = window.location.pathname;
    var pathParts = path.split('/');
    var newPath = pathParts[pathParts.indexOf('jobs') + 1];
    console.log(newPath);

    
}
  