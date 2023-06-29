import * as wasm from "hpke";

let pubKey = null;
// Fetch the public HPKE key.
(async () => {
  let resp = await fetch("http://localhost:8000/api/v1/hpke");
  let payload = await resp.json();
  pubKey = payload.pub_key;
})()

const query = document.getElementById("search-for");
query.addEventListener("input", async (e) => {
  const resultElm = document.getElementById("result");
  const interestsElm = document.getElementById("interests");
  const mail = wasm.seal(pubKey, JSON.stringify({
    interests: interestsElm.value,
    q: e.target.value,
  }));
  const [encapsulatedText, ciphertext] = mail.split("|");
  
  // const resp = await fetch("http://localhost:8000/api/v1/interests", {
  const resp = await fetch("http://localhost:8080/relay/interests", {
    method: "POST",
    headers: {
      "Accept": "application/json",
      "Content-Type": "application/json",
    },
    body: JSON.stringify({
      ciphertext,
      encapsulated: encapsulatedText,
    }),
  });
  const payload = await resp.json();

  resultElm.textContent = JSON.stringify(payload, null, 2);
});
