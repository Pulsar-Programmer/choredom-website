<<<<<<< HEAD
const express = require('express');
const mail = require('mail');
const prompt = require('prompt');

app.get('/', (req, res) => {
  res.send('Welcome to the home page!');
});

app.get('/signin', (req, res) => {
  res.send('Sign in page');
});

app.post('/signin', async (req, res) => {
  const email = req.body.email;
  const password = req.body.password;

  if (!email || !password) {
    res.send({
      success: false,
      error: "Please enter your email and password."
    });
    return;
  }

  const account = await getAccountByEmail(email);

  if (!account) {
    res.send({
      success: false,
      error: "Account does not exist."
    });
    return;
  }

  if (account.password !== password) {
    res.send({
      success: false,
      error: "Invalid password."
    });
    return;
  }

  res.send({
    success: true
  });
});

async function getAccountByEmail(email) {
  const account = await db.query('SELECT * FROM accounts WHERE email = ?', [email]);
  if (account.length === 0) {
    return null;
  }

  return account[0];
}

app.listen(3000, () => console.log('Server is running on localhost:3000'));
=======
const express = require('express');
const mail = require('mail');
const prompt = require('prompt');

app.get('/', (req, res) => {
  res.send('Welcome to the home page!');
});

app.get('/signin', (req, res) => {
  res.send('Sign in page');
});

app.post('/signin', async (req, res) => {
  const email = req.body.email;
  const password = req.body.password;

  if (!email || !password) {
    res.send({
      success: false,
      error: "Please enter your email and password."
    });
    return;
  }

  const account = await getAccountByEmail(email);

  if (!account) {
    res.send({
      success: false,
      error: "Account does not exist."
    });
    return;
  }

  if (account.password !== password) {
    res.send({
      success: false,
      error: "Invalid password."
    });
    return;
  }

  res.send({
    success: true
  });
});

async function getAccountByEmail(email) {
  const account = await db.query('SELECT * FROM accounts WHERE email = ?', [email]);
  if (account.length === 0) {
    return null;
  }

  return account[0];
}

app.listen(3000, () => console.log('Server is running on localhost:3000'));
>>>>>>> 59cf81640aca21bcc8ebaad78e8a2f77f9d06450
