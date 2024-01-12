


function expandImages(msg){
    var urlRegex = /(\b(https?|ftp|file):\/\/[-A-Z0-9+&@#\/%?=~_|!:,.;]*[-A-Z0-9+&@#\/%=~_|])/ig;
    return msg.replace(urlRegex, function(url) {
        return `<img src="${url}">`;
    });
}