import { fileURLToPath } from "url";
import fs from "fs";
import openai, { ChatCompletionRequestMessage } from "openai";
import path from "path";
import util from "util";
import readline from "readline/promises";
import type { Readable } from "stream";

import { default_logger } from "../logger";
const logger = default_logger.scoped("locale-gen");

const __dirname = path.dirname(fileURLToPath(import.meta.url));

const AI_KEY = process.env.AI_KEY;
if (!AI_KEY) {
  console.warn("No env.AI_KEY provided, aborting"),
    process.exit(1);
}

const rl = readline.createInterface({
  input: process.stdin,
  output: process.stdout
})

const language_names = ({
  "en": "English",
  "es": "Spanish",
  "es-AR": "Argentinian Spanish",
  "it": "Italian",
  "fr": "French",
  "de": "German",
  "pt": "Portuguese",
  "ar": "Arabic",
  "zh": "Simplified Chinese"
}) satisfies Partial<Record<string, string>>;

let isos: Array<keyof typeof language_names>;
while (true) {
  const isos_str = (await rl.question("Language iso codes? ")).trim();
  // @ts-ignore
  isos = isos_str.trim().split(/[\s,]+/g)
  for (const iso of isos) {
    const name: string = (language_names as any)[iso];
    if (name == null) {
      console.log(`Couldn't find name for iso ${iso}`)
      continue;
    }
  }

  break;
}

const kinds = ["studio", "admin", "misc", "wip", "countries", "stats-map", "validate", "type-of-content", "analytics", "payments"];
let selected_kinds: string[];

while (true) {
  const s = (await rl.question(`select a kind (${kinds.join(", ")}) `)).trim();
  if (s === "share") {
    selected_kinds = ["countries", "stats-map", "validate", "type-of-content", "analytics", "payments"];
    break;
  } else if (kinds.includes(s)) {
    selected_kinds = [s];
    break;
  }
}

let base: string;
while (true) {
  const s = (await rl.question(`select a base en/es `)).trim();
  if (s === "es" || s === "en") {
    base = s;
    break;
  }
}

for (const iso of isos) {
  logger.info(`generating files for iso ${iso} (${language_names[iso]})`);

  for (const kind of selected_kinds) {
    logger.info(`generating kind ${kind} - lang ${iso}`);

    const dir = kind === "studio" ? "studio" :
                kind === "admin" ? "admin" :
                kind === "wip" ? "wip" :
                kind === "misc" ? "misc" :
                `share/${kind}`;

    const src = `${__dirname}/${dir}/${kind}.${base}.ts`;
    const target = `${__dirname}/${dir}/${kind}.${iso}.ts`;

    while (true) {
      const n = (await rl.question(`\
  Generate ${language_names[iso]} (${iso}) ${kind} locale from base ${base}
  source = ${src}
  target = ${target}
  continue? y/n `)).trim().toLowerCase();
      if (n === "y") break;
      if (n === "n") {
        console.log("Aborting");
        process.exit(1);
      }
    }

    const request_message = `\
  Generate a localization file in typescript format for the language ${language_names[iso]} (ISO code: ${iso}), based on the \
  file provided in ${base === "es" ? "Spanish" : "English"}.
  Keep variables starting with "@" as is.
  The context of the translation is a user interface for a radio broadcasting application.

  /// file: ${kind}.${base}.ts
  ${fs.readFileSync(src, "utf8")}`;

    const client = new openai.OpenAIApi(new openai.Configuration({
      apiKey: AI_KEY
    }))

    const createChatCompletion = async function* (params: {
      model: "gpt-4" | "gpt-3.5-turbo" | "gpt-3.5-turbo-16k",
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
          if (lines.length === 0) {
            continue;
          }

          for (const line of lines) {
            if (line.trim() === "") continue;
            const event = line.trim().replace(/^data\:/, "").trim();
            if (event === "[DONE]") {
              break;
            }
            const json = JSON.parse(event);
            const token = json?.choices?.[0]?.delta?.content;
            if (typeof token === "string") {
              yield token;
            }
          }
        }
      } catch (e) {
        stream.destroy();
        throw e;
      }
    }

    const last_lines = (buf: string, n: number): string => {
      return buf.split("\n").slice(-n).join("\n");
    }

    let buf = "";
    let request_i = 0;
    request: while (true) {
      request_i++;
      buf = buf.split("\n").slice(0, -1).join("\n");
      logger.info(`sending request #${request_i}`);
      process.stdout.write(buf);

      const messages: ChatCompletionRequestMessage[] = [{
        role: "user",
        content: request_message,
      }];

      if (buf !== "") {
        const content = `[...]\n${last_lines(buf, 20)}`;

        messages.push({
          role: "assistant",
          content,
        })

        messages.push({
          role: "user",
          content: "Continue from the previous message",
        })
      }

      const stream = createChatCompletion({
        model: "gpt-3.5-turbo-16k",
        messages
      })

      try {
        for await (const token of stream) {
          buf += token;
          process.stdout.write(token);
        }

        logger.info("stream ended");
        break;

      } catch (e: any) {
        logger.warn(`stream error: ${e}`)
        logger.error(e);
        while (true) {
          const r = (await rl.question("Continue? y/n ")).trim();
          if (r === "n") break request;
          if (r === "y") continue request;
        }
      }
    }

    logger.info(`writing file to ${target}`);

    fs.writeFileSync(target, buf);
  }
}

logger.info("OK");

process.exit(0);