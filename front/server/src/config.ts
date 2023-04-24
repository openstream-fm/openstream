import { assertEquals } from "typia";
import toml from "toml"
import { readFileSync } from "fs";
import { color } from "./color"
import type { Logger } from "./logger"
import type { PartialDeep } from "type-fest";
import * as dot from "dot-prop";
import { clone } from "./util/collections";
import CommentJSON from "comment-json";

export type Config = {
  openstream: {
    api_base_url: string
    token: string
  }

  session: {
    secret: string
    // domain: string
    max_age_days: number
    cookie_name: string,
  }

  studio: {
    enabled: boolean
    port: number    
  }

  source_port: {
    local: number
    srv1: number
    srv2: number
    test: number
    default: number
  }
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
  str("openstream.token")

  str("session.secret");
  num("session.max_age_days");
  str("session.cookie_name")

  bool("studio.enabled");
  num("studio.port");

  num("source_port.local");
  num("source_port.test");
  num("source_port.srv1");
  num("source_port.srv2");
  num("source_port.default");

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
    logger.info("loading config only from env varianbles");
  } else {
    logger.info(`loading config from ${color.yellow(filename)}`);
    if(filename.endsWith(".json")) format = "json";
    // read formatted config string from file
    source = readFileSync(filename, "utf8");
  }

  return load_from_string(source, format, { env, logger });
}
