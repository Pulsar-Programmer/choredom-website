<<<<<<< HEAD
const express = require("express");

const app = express();

app.get("/signup", signup);

app.post("/signup", async (req, res) => {
  // The code that handles the form submission goes here.
});

=======
const express = require("express");

const app = express();

app.get("/signup", signup);

app.post("/signup", async (req, res) => {
  // The code that handles the form submission goes here.
});

>>>>>>> 59cf81640aca21bcc8ebaad78e8a2f77f9d06450
app.listen(3000, () => console.log("Server is running on port 3000"));