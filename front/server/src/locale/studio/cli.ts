import { fileURLToPath } from "url"; 
import fs from "fs";
import openai, { ChatCompletionRequestMessage } from "openai";
import path from "path";
import util from "util";
import readline from "readline/promises";
import type { Readable } from "stream";

import { default_logger } from "../../logger";

const __dirname = path.dirname(fileURLToPath(import.meta.url));

const AI_KEY = process.env.AI_KEY;
if(!AI_KEY) {
  console.warn("No env.AI_KEY provided, aborting"),
  process.exit(1);
}

const rl = readline.createInterface({
  input: process.stdin,
  output: process.stdout
})

let iso: string;
while(true) {
  iso = (await rl.question("Language iso code? ")).trim();
  if(iso !== "") break;
}

let name: string;
while(true) {
  name = (await rl.question("Language name in English? ")).trim();
  if(name !== "") break;
}

while(true) {
  const n = (await rl.question(`Generate ${name} (${iso}) locale, continue? y/n `)).trim().toLowerCase();
  if(n === "y") break;
  if(n === "n") {
    console.log("Aborting");
    process.exit(1);
  }
}

const request_message = `\
Generate a localization file in typescript format for the language ${name} (ISO code: ${iso}), based on the \
file provided in Spanish.
Keep variables starting with "@" as is.
The context of the translation is a user interface for a radio broadcasting application.

/// file: countries.es.ts
${fs.readFileSync(`${__dirname}/studio.es.ts`, "utf8")}`;

const logger = default_logger.scoped("locale-gen");

logger.info(`ai query: \n${request_message}`)

logger.info("generating locale file");

const client = new openai.OpenAIApi(new openai.Configuration({
  apiKey: AI_KEY
}))

const createChatCompletion = async function * (params: {
  model: "gpt-4" | "gpt-3.5-turbo",
  messages: ChatCompletionRequestMessage[]
}) {
  const res = await client.createChatCompletion({
    ...params,
    frequency_penalty: 0,
    temperature: 0,
    presence_penalty: 0,
    stream: true,
  }, {
    responseType: "stream",
  })

  // @ts-ignore
  const stream = res.data as Readable & AsyncIterable<Buffer>;

  try {
    for await (const buffer of stream) {
      const lines = buffer.toString().split("\n");
      if(lines.length === 0) {
        continue;
      }

      for(const line of lines) {
        if(line.trim() === "") continue;
        const event = line.trim().replace(/^data\:/, "").trim();
        if(event === "[DONE]") {
          break;
        }
        const json = JSON.parse(event);
        const token = json?.choices?.[0]?.delta?.content;
        if(typeof token === "string") {
          yield token;
        }
      }
    } 
  } catch(e) {
    stream.destroy();
    throw e;
  }
}

const last_lines = (buf: string, n: number): string => {
  return buf.split("\n").slice(-n).join("\n");
}

let buf = "";
let request_i = 0;
request: while(true) {
  request_i++;
  logger.info(`sending request #${request_i}`);
  process.stdout.write(buf);
  
  const messages: ChatCompletionRequestMessage[] = [{
    role: "user",
    content: request_message,
  }];

  if(buf !== "") {
    const content = `[...]\n${last_lines(buf, 20)}`;
    
    messages.push({
      role: "assistant",
      content,
    })

    logger.info("last lines context");
    logger.info(content);
  }

  const stream = createChatCompletion({
    model: "gpt-4",
    messages
  })

  try {
    for await (const token of stream) {
      buf += token;
      process.stdout.write(token);
    }

    logger.info("stream ended");
    break;

  } catch(e: any) {
    logger.warn(`stream error: ${e}`)
    logger.error(e);
    while(true) {
      const r = (await rl.question("Continue? y/n")).trim();
      if(r === "n") break request;
      if(r === "y") continue request;      
    }
  }
}

const target = `${__dirname}/generated.${iso}.${Date.now()}.ts`;
logger.info(`writing file to ${target}`);

fs.writeFileSync(target, buf);

logger.info("OK");

process.exit(0);