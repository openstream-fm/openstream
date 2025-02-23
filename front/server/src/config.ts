import typia, { assertEquals } from "typia";
import toml from "toml"
import { readFileSync } from "fs";
import { color } from "./color.js"
import type { Logger } from "./logger.js"
import type { PartialDeep } from "type-fest";
import * as dot from "dot-prop";
import { clone } from "./util/collections.js";
import CommentJSON from "comment-json";
import { findUp } from "find-up";
import path from "path";

export type Config = {
  openstream: {
    api_base_url: string
    // token: string
  }

  session: {
    secret: string
    max_age_days: number
    cookie_name: string,
  }

  studio?: {
    enabled: boolean
    port: number    
  }

  admin?: {
    enabled: boolean
    port: number    
  }

  embed?: {
    enabled: boolean,
    port: number,
  }

  payments?: {
    enabled: boolean
    port: number
    access_token: string
    credentials: {
      environment: "sandbox" | "production"
      merchant_id: string
      public_key: string
      private_key: string
    }
  }

  hosts: Record<string, HostConfig | void> & { default: HostConfig }
}

export type HostConfig = {
  cookie_domain: string,
  site: { host: string }
  studio: { host: string }
  admin: { host: string }
  api: { host: string }
  storage: { host: string }
  stream: { host: string }
  source: { host: string, port: number }
}

/**
 * merges enviromental variables with config readed from config file
 * enviromental variables are prefixex with OPENSTREAM_FRONT
 * and config properties are mapped such as public.api_base_url transforms to OPENSTREAM_FRONT_PUBLIC_API_BASE_URL 
 * @param env: available for test porposes
 */

export const merge_env = (partial: PartialDeep<Config>, { logger, env = process.env }: { logger?: Logger, env?: typeof process.env }): PartialDeep<Config> => {
  const config = clone(partial);

  /**
   *  maps config deep props to env keys, properties are mapped such as
   *  public.api_base_url transforms to OPENSTREAM_FRONT_PUBLIC_API_BASE_URL 
   */
  const map_prop = (property_path: string): string => {
    return `OPENSTREAM_FRONT_${property_path.replaceAll(".", "_").toUpperCase()}`;
  }

    /**
   * override config option with enviromental variable if set
   * property is expected to be a number
   * */
    const hosts = (property_path: string) => {
      const key = map_prop(property_path);
      const s = env[key]?.trim();
      if(s != null) {
        logger?.info(`using env ${color.yellow(key)} as ${color.yellow(`$config.${property_path}`)}`);
        const raw_value = JSON.parse(s);
        const v = typia.assertEquals<Config["hosts"]>(raw_value);
        dot.setProperty(config, property_path, v);
      }
    }

  /**
   * override config option with enviromental variable if set
   * property is expected to be a number
   * */
  const num = (property_path: string) => {
    const key = map_prop(property_path);
    const s = env[key]?.trim();
    if(s != null) {
      const n = Number(s);
      if(Number.isNaN(n)) throw new Error(`env.${key} should be a number`);
      logger?.info(`using env ${color.yellow(key)} as ${color.yellow(`$config.${property_path}`)}`);
      dot.setProperty(config, property_path, n);
    }
  }

  /**
   * override config option with enviromental variable if set
   * property is expected to be a string
   */
  const str = (property_path: string) => {
    const key = map_prop(property_path);
    const s = env[key]?.trim();
    if(s != null) {
      logger?.info(`using env ${color.yellow(key)} as ${color.yellow(`$config.${property_path}`)}`);
      dot.setProperty(config, property_path, s.trim());
    }
  }

  /**
   * override config option with enviromental variable if set
   * property is expected to be a string
   * accepted env values are "1" "true" "0" "false"
   */
  const bool = (property_path: string) => {
    const key = map_prop(property_path);
    const s = env[key]?.trim();
    if(s != null) {
      switch(s) {
        case "1":
        case "true":
          logger?.info(`using env ${color.yellow(key)} as ${color.yellow(`$config.${property_path}`)}`);
          dot.setProperty(config, property_path, true);
          break;
        case "0":
        case "false":
          logger?.info(`using env ${color.yellow(key)} as ${color.yellow(`$config.${property_path}`)}`);
          dot.setProperty(config, property_path, false);
          break;
        default:
          throw new Error(`env ${key} should be a boolean, accepted values are "1", "true", "0", "false", received ${s}`)
      }
    }
  }

  str("openstream.api_base_url")
  // str("openstream.token")

  str("session.secret");
  num("session.max_age_days");
  str("session.cookie_name")

  bool("studio.enabled");
  num("studio.port");

  bool("admin.enabled");
  num("admin.port");
  
  bool("embed.enabled");
  num("embed.port");

  bool("payments.enabled");
  num("payments.port");
  str("payments.access_token");
  str("payments.credentials.environment");
  str("payments.credentials.merchant_id");
  str("payments.credentials.public_key");
  str("payments.credentials.private_key");
  
  hosts("hosts");

  return config;
}

// if source is null means load only from env variables
export const load_from_string = (source: string | null, format: "toml" | "json", { logger, env = process.env }: { logger?: Logger, env?: typeof process.env } = {}): Config => {
  
  let partial_config: PartialDeep<Config> = {};

  if(source != null) {
    const object = format === "json" ? CommentJSON.parse(source) : toml.parse(source);
    // check that there are no unknown keys or invalid types for present keys
    partial_config = assertEquals<PartialDeep<Config>>(object);   
  } 

  // override config with available env variables
  const partial_merged_config = merge_env(partial_config, { env, logger });

  // asserts that the final config object is of expected type
  const config = assertEquals<Config>(partial_merged_config);

  return config;
}

export const load = (filename: string | null, { logger: _logger, env = process.env }: { logger: Logger, env?: typeof process.env }): Config => {
  const logger = _logger.scoped("config");

  let source: string | null = null;
  let format: "toml" | "json" = "toml";

  if(filename == null) {
    logger.info("loading config only from env variables");
  } else {
    logger.info(`loading config from ${color.yellow(filename)}`);
    if(filename.endsWith(".json") || filename.endsWith(".jsonc")) format = "json";
    // read formatted config string from file
    source = readFileSync(filename, "utf8");
  }

  return load_from_string(source, format, { env, logger });
}

export const resolve = async (name: "__UP__" | string | null): Promise<string | null> => {
  if(name == null) return null;
  else if(name === "__UP__") {
    const up = await findUp("openstream-front.toml");
    if(up == null) {
      throw new Error("Couldn't find openstream-front.toml file in working diretory or parent directories");
    } else {
      return up;
    }
  } else {
    return path.resolve(process.cwd(), name);
  }
}