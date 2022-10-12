import { setTimeout } from "timers/promises";

let BYTES = 0; 
let CLIENTS = 0;
let ERRORS = 0;
let CLOSED = 0;

const client = async () => {
  try {
    CLIENTS++;
    const res = await fetch("http://stream.openstream.test/stream/1");
    for await (const data of res.body) {
      BYTES += data.length;
    }
  } catch (e) {
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
    await setTimeout(1);
    client();
  }
}

createClients(25_000);

setInterval(() => {
  console.log(`CLIENTS: ${CLIENTS} => BYTES ${BYTES} => CLOSED ${CLOSED} => ERRORS ${ERRORS}`)
}, 1000)