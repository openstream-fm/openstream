import { program as cmd } from "commander";
import { color } from "./color.js";
import fs from "fs";
import path from "path";

import { TypeGuardError } from "typia";

import * as config from "./config.js";
import * as app from "./server.js"
import { ConsoleLogger } from "./logger.js";
import { LogLevel } from "./log-level.js";

import { fileURLToPath } from  "url";
import { env } from "./env.js";

const __dirname =  path.dirname(fileURLToPath(import.meta.url));

const VERSION = "0.0.1"

const createConfig = (opts: { output: string }) => {
  const logger = new ConsoleLogger(LogLevel.INFO);
  logger.info("> Creating config file in " + color.yellow(opts.output));

  let ext = opts.output.endsWith(".json") ? "jsonc" : "toml";
  let sample = path.resolve(__dirname, `../openstream-front.sample.${ext}`)

  const dest = path.resolve(process.cwd(), opts.output);
  
  if(fs.existsSync(dest)) {
    logger.warn(color.red(`> Aborting: file ${dest} already exists`))
    return process.exit(1);
  }

  fs.copyFileSync(sample, dest);
  logger.info("> Config file created in " + color.yellow(dest));
  logger.info("- every config option has a env variable counterpart") 
  logger.info("- env variables will override config options if present")
  logger.info("- you can skip the config file entirely providing --config=null as an argument to the start function")
  logger.info("- in that case you must fill all the configuration from env variables")
  logger.info("> Before start edit the settings as needed")
  logger.info("> Then run " + color.yellow("openstream-front start") + " in the config directory")
  process.exit(0);
}

const start = async (opts: { config: string }) => {
  
  const logger = new ConsoleLogger(env.LOG_LEVEL).scoped("start");
  
  let conf: config.Config;

  try {
    const filename = await config.resolve(opts.config === "null" ? null : opts.config);
    conf = config.load(filename, { logger });
  } catch(e: any) {
    logger.error(`error loading config: ${e}`);
    if(e instanceof TypeGuardError) {
      let message = [
        `generated config object is invalid at path ${color.yellow(e.path?.replace("$input.", "$config.") || "")}`,
        `expected: ${color.yellow(String(e.expected))}`,
        `generated: ${color.yellow(JSON.stringify(e.value) || "undefined")}`,
      ].join("\n")
      logger.error(message);
    } else {
      // report the error to the user and exit with error status code
      logger.error(e);
    }

    process.exit(1);
  }

  await app.start({ config: conf, logger });

  if(process.send) process.send("ready");
}

cmd.version(VERSION);

cmd.command("start")
  .description("start the webmail server")
  .option("-c --config <path>", "path to the config file", "__UP__")
  .action(start);

cmd.command("create-config")
  .description("create the default config file, it can be in TOML format (default) or in JSON format (guessed from the output file extension)")
  .option("-o --output <path>", "path to ouput file", "./openstream-front.toml")
  .action(createConfig)

cmd.parse(process.argv);