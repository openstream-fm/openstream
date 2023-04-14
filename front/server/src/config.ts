import { assertEquals } from "typia";
import toml from "toml"
import { readFileSync } from "fs";
import { color } from "./color"
import type { Logger } from "./logger"
import type { PartialDeep } from "type-fest";
import * as dot from "dot-prop";
import { clone } from "./util/collections";

export type Config = {
  openstream: {
    api_base_url: string
    token: string
  }

  // public: {
  //   stream_public_url: string
  //   source_public_url: string
  //   storage_public_url: string
  // }

  // mongodb: {
  //   url: string
  // }

  session: {
    secret: string
    // domain: string
    max_age_days: number
    cookie_name: string,
  }

  studio: {
    enabled: boolean
    port: number    
    // public_base_url: string
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

  // str("public.stream_public_url")
  // str("public.source_public_url")
  // str("public.storage_public_url")

  // str("mongodb.url")

  str("session.secret");
  // str("session.domain");
  num("session.max_age_days");
  str("session.cookie_name")

  bool("studio.enabled");
  num("studio.port");
  // str("studio.public_base_url");

  return config;
}

export const load_from_string = (source: string, { logger, env = process.env }: { logger?: Logger, env?: typeof process.env } = {}): Config => {
  // check that there are no unknown keys or invalid types for present keys
  const partial_config = assertEquals<PartialDeep<Config>>(toml.parse(source));

  // override config with available env variables
  const partial_merged_config = merge_env(partial_config, { env, logger });

  // asserts that the final config object is of expected type
  const config = assertEquals<Config>(partial_merged_config);

  return config;
}

export const load = (filename: string | null, { logger: _logger, env = process.env }: { logger: Logger, env?: typeof process.env }): Config => {
  const logger = _logger.scoped("config");

  let source: string = "";
  
  if(filename == null) {
    logger.info("loading config only from env varianbles");
  } else {
    logger.info(`loading config from ${color.yellow(filename)}`);
    // read toml formatted config string from file
    source = readFileSync(filename, "utf8");
  }

  return load_from_string(source, { env, logger });
}
