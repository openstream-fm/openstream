import { LogLevel } from "./log-level"

const get_log_level = (): LogLevel => {
  let env = process.env.LOG_LEVEL?.toLowerCase();
  if(env === "silent") return LogLevel.SILENT;
  if(env === "error") return LogLevel.ERROR;
  if(env === "warn") return LogLevel.WARN;
  if(env === "info") return LogLevel.INFO;
  if(env === "debug") return LogLevel.DEBUG;
  return LogLevel.INFO;
}

const get_log_ts = (): boolean => {
  const env = process.env.LOG_TS;
  if(env === "true") return true;
  if(env === "1") return true;
  if(env === "false") return false;
  if(env === "0") return false;
  return true;
}

const bool = (key: string, def: boolean | null = null): boolean => {
  const v = process.env[key];
  if(v == null) {
    if(def != null) return def;
    else throw new Error(`env.${key} is required`)
  } else {
    if(v === "1" || v === "true") return true;
    else if(v === "0" || v === "false") return false;
    else throw new Error(`env.${key} must be a boolean ("1", "0", "true" or "false")`)
  }
}

export const env = {
  SVELTEKIT_APP_DEV: bool("SVELTEKIT_APP_DEV", false),
  SVELTEKIT_APP_PORT: Number(process.env.SVELTEKIT_APP_PORT) || 3100,
  SVELTEKIT_ADMIN_DEV: bool("SVELTEKIT_ADMIN_DEV", false),
  SVELTEKIT_ADMIN_PORT: Number(process.env.SVELTEKIT_ADMIN_PORT) || 5100,
  LOG_LEVEL: get_log_level(),
  LOG_TS: get_log_ts()
}