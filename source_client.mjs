import { once } from "events";
import { readFileSync } from "fs";
import net from "net";

const file = readFileSync("./audio.mp3");

const sleep = (ms) => new Promise(resolve => setTimeout(resolve, ms));

const write = async (socket, data) => {
  new Promise((resolve, reject) => {
    socket.write(data, err => {
      err ? reject(err) : resolve();
    })
  })
}

const chunks = function * (buffer, size) {
  let start = 0;
  while(true) {
    if (start >= buffer.length) return;
    let chunk = buffer.subarray(start, start + size);
    start = start + size;
    yield chunk;
  }
}

/*
const pipe = async (readable, writable, peeker) => {
  for await (const buf of readable) {
    peeker(data);
    await write(writable, buf);
  }
}
*/

const client = async (id) => {
  //const socket = net.connect({ host: "192.168.1.102", port: 20500 });
  const socket = net.connect({ host: "source.openstream.test", port: 80 });

  socket.on("connect", async () => {
    
    console.log(`socket ${id} opened`);
    
    const method = "SOURCE";
    const url = `/${id}/source`;
    const version = "HTTP/1.0";
    const headers = {
      "content-type": "audio/mpeg",
      "expect": "100-Continue",
      "connection": "close",
    }

    console.log(`> ${method} ${url} ${version}`);
    socket.write(`${method} ${url} ${version}\r\n`);

    for(const [name, value] of Object.entries(headers)) {
      console.log(`> ${name}: ${value}`);
      socket.write(`${name}: ${value}\r\n`);
    }

    console.log(">");
    socket.write(`\r\n`);

    let first = true;
    socket.on("data", buf => {
      if(first) {
        first = false;
        process.stdout.write("< ");
      }

      process.stdout.write(buf.toString("utf-8").replaceAll("\n", "\n< "));
    })

    socket.on("close", () => {
      console.log("\n| socket closed, exiting");
      process.exit();
    })

    while(true) {
      console.log(`> @FILE => ${id}`);
      const start = Date.now();
      const byterate = 16 * 1024;
      let written = 0;
      for (const chunk of chunks(file, 1024)) {
        //console.log("chunk", chunk.length);
        await write(socket, chunk);
        //console.log("write", chunk.length);
        written += chunk.length;
        const until = start + ((written / byterate) * 1000);
        const ms = until - Date.now();
        //console.log(ms);
        await sleep(ms);
      }
    }
  })
}

const createClients = async (n) => {
  for(let id = 1; id <= n; id++) {
    client(id);
    await sleep(1000);
  }
}

createClients(1);