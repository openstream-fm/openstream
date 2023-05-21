import { fileURLToPath } from "url"; 
import fs from "fs";
import openai from "openai";
import path from "path";
import util from "util";
import readline from "readline/promises";

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

const message = `\
Generate a localization file in typescript format for the language ${name} (ISO code: ${iso}), based on the \
file provided in Spanish.
Keep variables starting with "@" as is.
The context of the translation is a user interface for a radio broadcasting application.

/// file: studio.es.ts
${fs.readFileSync(`${__dirname}/studio.es.ts`, "utf8")}`;

const logger = default_logger.scoped("locale-gen");

logger.info(`ai query: \n${message}`)

logger.info("generating locale file");

const client = new openai.OpenAIApi(new openai.Configuration({
  apiKey: AI_KEY
}))

const response = await client.createChatCompletion({
  model: "gpt-4",
  messages: [
    {
      role: "user",
      content: message,
    }
  ],
  frequency_penalty: 0,
  temperature: 0,
  presence_penalty: 0,
}).catch(e => {
  logger.warn(util.inspect(e?.response?.data, { depth: 100 }))
  logger.error(e);
  process.exit(1);
})

logger.info("response obtained");

logger.info(util.inspect(response.data, { depth: 100 }));

const target = `${__dirname}/generated.${iso}.${Date.now()}.ts`;
logger.info(`writing file to ${target}`);

fs.writeFileSync(target, response.data.choices[0].message!.content);

logger.info("OK");

process.exit(0);