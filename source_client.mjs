import { once } from "events";
import { readFileSync } from "fs";
import net from "net";
import { setTimeout } from "timers/promises";

const file = readFileSync("./audio.aac");

const write = async (socket, data) => {
  socket.write(data);
  await once(socket, "drain");
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
  const socket = net.connect({ host: "192.168.1.102", port: 80 });
  //const socket = net.connect({ host: "source.openstream.test", port: 80 });

  socket.on("connect", async () => {
    
    console.log(`socket ${id} opened`);
    
    const method = "PUT";
    const url = `/${id}/source`;
    const version = "HTTP/1.0";
    const headers = {
      "content-type": "audio/aac",
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
      await write(socket, file);
    }
  })
}

const createClients = async (n) => {
  for(let id = 1; id <= n; id++) {
    client(id);
    await setTimeout(1000);
  }
}

createClients(1);