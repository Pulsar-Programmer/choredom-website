<<<<<<< HEAD
const mail = require("mail");
const prompt = require("prompt");



async function createAccount(data) {
  const email = req.body.email;
  const password = req.body.password;
  const confirmPassword = req.body.confirmPassword;
  const town = req.body.town;
  const username = req.body.username;
  const displayName = req.body.displayName;

  if (password !== confirmPassword) {
    res.send({
      success: false,
      error: "Passwords do not match."
    });
    return;
  }

  // Send an email with a verification code.
  const verificationCode = Math.random().toString(36).substring(7);
  const mailOptions = {
    to: email,
    subject: "Email Verification",
    text: "Your verification code is: ${verificationCode}"
  };
  mail.sendMail(mailOptions);//

  // Ask the user to enter the verification code.
  const code = await prompt("Please enter the verification code you received in your email: ");

  // Verify the verification code.
  if (code === verificationCode) {
    // Create a new account.
    const account = await createAccount({
      email: email,
      password: password,
      town: town,
      username: username,
      displayName: displayName
    });
=======
const mail = require("mail");
const prompt = require("prompt");



async function createAccount(data) {
  const email = req.body.email;
  const password = req.body.password;
  const confirmPassword = req.body.confirmPassword;
  const town = req.body.town;
  const username = req.body.username;
  const displayName = req.body.displayName;

  if (password !== confirmPassword) {
    res.send({
      success: false,
      error: "Passwords do not match."
    });
    return;
  }

  // Send an email with a verification code.
  const verificationCode = Math.random().toString(36).substring(7);
  const mailOptions = {
    to: email,
    subject: "Email Verification",
    text: "Your verification code is: ${verificationCode}"
  };
  mail.sendMail(mailOptions);//

  // Ask the user to enter the verification code.
  const code = await prompt("Please enter the verification code you received in your email: ");

  // Verify the verification code.
  if (code === verificationCode) {
    // Create a new account.
    const account = await createAccount({
      email: email,
      password: password,
      town: town,
      username: username,
      displayName: displayName
    });
>>>>>>> 59cf81640aca21bcc8ebaad78e8a2f77f9d06450
}}