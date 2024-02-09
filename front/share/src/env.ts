// import { env as kit_env } from "$env/dynamic/public";
import { browser, dev } from "$app/environment";
import type { LogLevel } from "./logger.js";

const get_log_level = (): LogLevel => {
  if(browser) {
    let hash = window.location.hash.toLowerCase();
    // fix for circular imports (numbers directly)
    if(hash.includes("log=silent")) return 0;
    if(hash.includes("log=error")) return 1;
    if(hash.includes("log=warn")) return 2;
    if(hash.includes("log=info")) return 3;
    if(hash.includes("log=debug")) return 4;
    if(dev) return 4;
    return 3;
  } else {
    return 3;
  }
}

const get_log_ts = (): boolean => {
  if(browser) {
    if(window.location.hash.toLowerCase().includes("!log-ts")) return false;
    return true;
  } else {
    return true
  }
}

export const env = {
  LOG_LEVEL: get_log_level(),
  LOG_TS: get_log_ts(),
}