import { assertEquals } from "typia";
import toml from "toml"
import { readFileSync } from "fs";
import { color } from "./color"
import { Logger } from "./logger"

export type Config = {
  openstream: {
    apiBaseURL: string
    token: string
  }

  public: {
    streamPublicURL: string
    storagePublicURL: string
  }

  mongodb: {
    url: string
  }

  session: {
    secret: string
    domain: string
    maxAgeDays: number
    cookieName?: string,
  }

  admin?: {
    enabled: boolean,
    port: number
    publicBaseURL: string
  }

  app?: {
    enabled: boolean
    port: number    
    publicBaseURL: string
  }
}

export const load = (filename: string, { logger: _logger }: { logger: Logger }): Config => {
  const logger = _logger.scoped("config");

  logger.info(`loading config from ${color.yellow(filename)}`);
  
  try {
    const source = readFileSync(filename, "utf8");
    const config = assertEquals<Config>(toml.parse(source));
    return config;
  } catch(e: any) {
    logger.warn(`error loading config file: ${e}`);
    logger.error(e);
    process.exit(1);
  }
}