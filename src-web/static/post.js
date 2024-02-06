function submit_post(){
    let title = document.getElementById("title").value;
    let body = document.getElementById("body").value;
    let location = document.getElementById("city").value;
    let time = document.getElementById("time").value;
    let price = document.getElementById("price").value;
    
    if(String(title).trim() === "") {
        alert("Please fill in the `title` field.");
        return;
    }

    let jobdata = {title: title, body: body, location: location, time: time, price: price};

    fetch("/post-job-2", {
        method: 'POST', 
        headers: {
            'Content-Type': 'application/json',
        },
        body: JSON.stringify(jobdata), 
    })
    .then(handle)
    .then(_ => {
        redirect("/success");
    })
    .catch(notify);
}





// const textarea = document.getElementById("description-area");
    

// function autoResize() {
//   this.style.height = "auto";
//   this.style.height = `${this.scrollHeight}px`;
// }

// textarea.addEventListener("input", autoResize);



// document.addEventListener('DOMContentLoaded', function() {
//     const textarea = document.getElementById("description-area");
  
//     if (textarea) {
//       function autoResize() {
//         this.style.height = "auto";
//         this.style.height = `${this.scrollHeight}px`;
//       }
  
//       // Event listener for input event
//       textarea.addEventListener("input", autoResize);
//     } else {
//       console.error('Element with ID "description-area" not found');
//     }
//   });
  



//   document.addEventListener('DOMContentLoaded', function() {
//     const textarea = document.getElementById("description-area");
  
//     if (textarea) {
//       function autoResize() {
//         this.style.height = "auto";
//         this.style.height = `${this.scrollHeight}px`;
//       }
  
//       // Event listener for input event
//       textarea.addEventListener("input", autoResize);
//     } else {
//       console.error('Element with ID "description-area" not found');
//     }
//   });
  