import http from "http";
import fs from "fs";
import { fileURLToPath } from "url";
import path from "path";

const __dirname = path.dirname(fileURLToPath(import.meta.url));

const file = fs.readFileSync(`${__dirname}/audio-5s.mp3`);

http.createServer((req, res) => {
  res.writeHead(200, {
    "content-type": "audio/mpeg",
  });

  res.write(file);
}).listen(3000, () => {
  console.log("server listening on port 3000");
})