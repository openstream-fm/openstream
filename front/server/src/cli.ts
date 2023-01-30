import { program as cmd } from "commander";
import { color } from "./color";
import fs from "fs";
import path from "path";
//import { promises } from "fs";

import * as config from "./config";
import * as app from "./app"
import { ConsoleLogger } from "./logger";
import { LogLevel } from "./log-level";

//import { fileURLToPath } from  "url";
import { env } from "./env";

//const __dirname =  path.dirname(fileURLToPath(import.meta.url));
//const { mkdir } = promises;

const originalEmit = process.emit;
// @ts-ignore
process.emit = function (name, data, ...args) {
  if (
    name === `warning` &&
    // @ts-ignore
    data?.name === `ExperimentalWarning`
  ) {
    return false;
  }
  // @ts-ignore
  return originalEmit.call(process, name, data, ...args);
};

const VERSION = "0.0.1"

const createConfig = (opts: { output: string }) => {
  const logger = new ConsoleLogger(LogLevel.INFO);
  logger.info("> Creating config file in " + color.yellow(opts.output));

  const sample = path.resolve(__dirname, "../config.sample.toml");
  const dest = path.resolve(process.cwd(), opts.output);
  if(fs.existsSync(dest)) {
    logger.warn(color.red(`> Aborting: file ${dest} already exists`))
    return process.exit(1);
  }

  fs.copyFileSync(sample, dest);
  logger.info("> Config file created in " + color.yellow(dest));
  logger.info("> Before start edit the settings as needed")
  logger.info("> Then run " + color.yellow("openstream-front start") + " in the config directory")
  process.exit(0);
}

const start = async (opts: { config: string }) => {
  
  const logger = new ConsoleLogger(env.LOG_LEVEL);
  const conf = config.load(path.resolve(process.cwd(), opts.config), { logger });
  
  app.start({ config: conf, logger });
}

cmd.version(VERSION);

cmd.command("start")
  .description("starts the webmail server")
  .option("-c --config <path>", "path to the config file", "./config.toml")
  .action(start);

cmd.command("create-config")
  .description("create the default config.toml file")
  .option("-o --output <path>", "path to ouput file", "./config.toml")
  .action(createConfig)

cmd.parse(process.argv);