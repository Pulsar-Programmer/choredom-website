/*

NOT DONE YET
put HTML data here

*/


// document.getElementById('upload-form').addEventListener('submit', async (event) => {
//     event.preventDefault();

//     const fileInput = document.getElementById('file-input');
//     const file = fileInput.files[0];

//     const formData = new FormData();
//     formData.append('file', file);

//     try {
//         const response = await fetch('/upload', {
//         method: 'POST',
//         body: formData
//         });

//         if (!response.ok) {
//             throw new Error(`Server responded with status ${response.status}`);
//         }

//         const result = await response.json();
//         console.log(result);
//     } catch (error) {
//         console.error('An error occurred:', error);
//     }
// });
   
function upload(){
    const fileInputElement = document.getElementById("file_upload");
    let formData = new FormData();
    for(f of fileInputElement.files){
        formData.append('file', f, 'filename.png');
    }
    fetch('/settings/upload/form', {
        method: 'POST',
        body: formData
    })
    .then(handle)
    .then(_ => {
        alert(`Successful upload!`);
        redirect("/settings")
    })
    .catch(notify);
}