import { setTimeout } from "timers/promises";

let BYTES = 0; 
let CLIENTS = 0;
let ERRORS = 0;
let CLOSED = 0;

const ID = "tmeqv925";

const client = async () => {
  try {
    CLIENTS++;
    const res = await fetch(`http://127.0.0.1:20200/stream/${ID}`);
    for await (const data of res.body) {
      BYTES += data.length;
    }
  } catch (e) {
    console.warn(`${e} => ${e.cause}`);
    CLIENTS--;
    ERRORS++;
    client();
  }
  CLIENTS--;
  CLOSED++;
  client();
}

const createClients = async (n) => {  
  for(let i = 0; i < n; i++) {
    await setTimeout(25);
    client();
  }
}

createClients(Number(process.env.C) || 100);

setInterval(() => {
  console.log(`CLIENTS: ${CLIENTS} => BYTES ${BYTES} => CLOSED ${CLOSED} => ERRORS ${ERRORS}`)
}, 1000)