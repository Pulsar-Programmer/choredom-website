// import axios from 'axios';

let clicks = 0;
let imclicks = 0;
let reverseb = false;
let parallelb = false;
// let 

get_user_score()

function update_leaderboard(){
  fetch('http://127.0.0.1:8080/leaderboard', {
    mode: "same-origin",
  }).then((r) => r.json()).then((d) => {
    document.getElementById("table").innerHTML = "<tr><th>Leaderboard</th></tr><tr><td>#</td><td>Name</td><td>Score</td></tr>";
    console.log(d)
    d.forEach((element, i) => {
      let tr = document.createElement("tr");
      console.log(i, element);
      tr.insertCell(0).innerHTML = `${i + 1}`;
      tr.insertCell(1).innerHTML = `${element.name}`;
      let rscore = Number(element.rscore);
      let iscore = Number(element.iscore);
      if (imclicks == 0) {
        tr.insertCell(2).innerHTML = rscore;
      }
      else if (imclicks < 0) {
        tr.insertCell(2).innerHTML = "" + rscore + " - " + (-iscore) + "i";
      }
      else {
        tr.insertCell(2).innerHTML = "" + rscore + " + " + iscore + "i";
      }
      document.getElementById("table").appendChild(tr)
    });
  })
}
function update_score(){
  const params = new URLSearchParams(window.location.search)
  let UserScore = {
    userid: Number(params.get("pk")),
    name: "",
    rscore: clicks,
    iscore: imclicks,}
  fetch('http://127.0.0.1:8080/update_leaderboard', {
    method: 'POST',
    mode: "same-origin",
    body: JSON.stringify(UserScore),
    headers: {
      'Content-Type': 'application/json',
    }
  })
}
function get_user_score(){
  const params = new URLSearchParams(window.location.search)
  let pk = Number(params.get("pk"));
  fetch(`http://127.0.0.1:8080/getuserscore/${pk}`, {
    method: 'GET',
    headers: {
      'Content-Type': 'application/json'
    }
  }).then((r) => r.json()).then((data) => {
    console.log(data)
    update_leaderboard();
    setInterval(() => {
      update_score();
      setTimeout(() => {
        update_leaderboard();
      }, 500)
      // console.log("update")
    }, 1000);
    imclicks = data.iscore;
    clicks = data.rscore;
    // console.log(clicks + " " + imclicks);
    update_counter();
    // document.getElementById("counter").innerText = 
  })
}









function click_cookie() {
  if (parallelb) {
    if (reverseb) {
      imclicks--;
    }
    else {
      imclicks++;
    }
  }
  else {
    if (reverseb) {
      clicks--;
    }
    else {
      clicks++;
    }
  }

  update_counter();
}
function reset() {
  if (reverseb) {
    if (parallelb) {
      imclicks = 1000;
    }
    else {
      clicks = 1000;
    }
  }
  else {
    if (parallelb) {
      imclicks = 0;
    }
    else {
      clicks = 0;
    }
  }
  update_counter();
}
function update_counter() {
  let counter = document.querySelector('#counter');
  if (imclicks == 0) {
    counter.innerHTML = clicks;
  }
  else if (imclicks < 0) {
    counter.innerHTML = "" + clicks + " - " + Math.abs(imclicks) + "i";
  }
  else {
    counter.innerHTML = "" + clicks + " + " + imclicks + "i";
  }
  update_screen();
}
function update_screen() {
  let p1 = document.getElementById("p1");
  let p2 = document.getElementById("p2");
  let p3 = document.getElementById("p3");
  if (reverseb) {
    p1.innerHTML = "Reverse: On"
  }
  else {
    p1.innerHTML = "Reverse: Off"
  }
  if (parallelb) {
    p2.innerHTML = "Parallel: On"
  }
  else {
    p2.innerHTML = "Parallel: Off"
  }
  let s = detect();
  //manipulate s to correct direction
  p3.innerHTML = "Direction: " + s;
  //p3 is Direction, North, South, Northeast, Northwest, Southeast, Southwest, East, West, None
}
function reverse() {
  reverseb = !reverseb;
  update_screen();
}
function parallel() {
  parallelb = !parallelb;
  update_screen();
}
function detect() {
  if (clicks == 0) {
    if (imclicks == 0) {
      return "None"
    }
    else if (imclicks > 0) {
      return "North"
    }
    else {
      return "South"
    }
  }
  else if (clicks > 0) {
    if (imclicks == 0) {
      return "East"
    }
    else if (imclicks > 0) {
      return "Northeast"
    }
    else {
      return "Southeast"
    }
  }
  else {
    if (imclicks == 0) {
      return "West"
    }
    else if (imclicks > 0) {
      return "Northwest"
    }
    else {
      return "Southwest"
    }
  }
}
// function expand(){
//   let c = document.querySelector('#cookie');
//   c.style.width *= dialation;
//   c.style.height *= dialation;
// }
// function shrink(){
//   let c = document.querySelector('#cookie');
//   c.style.width /= dialation;
//   c.style.height /= dialation;
// }
// const dialation = 1.25;