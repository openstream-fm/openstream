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

export const env = {
  SVELTEKIT_APP_DEV: ["1", "true"].includes(process.env.SVELTEKIT_APP_DEV ?? ""),
  SVELTEKIT_APP_PORT: Number(process.env.SVELTEKIT_APP_PORT) || 3100,
  LOG_LEVEL: get_log_level()
}

