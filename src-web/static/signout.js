function signout(){
    fetch('/signout', {
        method: 'POST',
        headers: {
            'Content-Type': 'application/json',
        },
    })
    .then(handle)
    .then(_ => redirect("/success"))
    .catch(notify);
}