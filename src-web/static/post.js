// import { invoke } from '@tauri-apps/api/tauri';
// const invoke = window.__TAURI__.invoke;



function saveJob() {
  const jobData = {
    title: document.getElementById("title").value,
    body: document.getElementById("body").value,
    location: document.getElementById("location").value,
    time: document.getElementById("time").value,
    price: document.getElementById("price").value,
  };

  // invoke('register_job', jobData)

}



  





