import { LogLevel } from "./logger"

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

export const env = {
  SVELTEKIT_APP_DEV: ["1", "true"].includes(process.env.SVELTEKIT_APP_DEV ?? ""),
  SVELTEKIT_APP_PORT: Number(process.env.SVELTEKIT_APP_PORT) || 3100,
  LOG_LEVEL: get_log_level(),
  LOG_TS: get_log_ts()
}
