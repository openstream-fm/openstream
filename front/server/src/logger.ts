import { color } from "./color.js";
import { env } from "./env.js";
import { LogLevel } from "./log-level.js";

export interface Logger {
  error: (error: string | Error) => void,
  warn: (arg: any) => void,
  info: (arg: any) => void,
  debug: (arg: any) => void,
  enabled: (level: LogLevel) => boolean,
  scoped: (scope: string | null) => Logger 
}

const p = (v: string | number, n = 2, fill = "0"): string => {
  return String(v).padStart(n, fill);
}

const timestamp = (date: Date = new Date()): string => {
  const y = date.getFullYear();
  const m = date.getMonth() + 1;
  const d = date.getDate();
  const h = date.getHours();
  const min = date.getMinutes();
  const sec = date.getSeconds();
  const ms = date.getMilliseconds();
  const offset = date.getTimezoneOffset();

  const tz_h = Math.floor(Math.abs(offset) / 60);
  const tz_min = Math.abs(offset) % 60; 
  const tz_sign = offset >= 0 ? "+" : "-"

  return `${y}-${p(m)}-${p(d)} ${p(h)}:${p(min)}:${p(sec)}.${p(ms, 3)} ${tz_sign}${p(tz_h)}:${p(tz_min)}`
}

export type Options = {
  ts: boolean
}

export class ConsoleLogger implements Logger {
  
  #options: Options;
  #level: LogLevel;
  #scope: string | null;
  #s: string;

  constructor(level: LogLevel, scope: string | null = null, options: Partial<Options> = {}) {
    
    this.#level = level;
    this.#scope = scope;
    this.#options = { ts: options.ts ?? true };

    if(this.#scope != null) {
      this.#s = ` ${color.bold(this.#scope)} >`
    } else {
      this.#s = "";
    }
  }

  get level() {
    return this.#level;
  }

  get options (): Readonly<Options> {
    return { ...this.#options };
  }

  
  #ts(): string {
    return this.#options.ts ? `${timestamp()} ` : "";
  }

  enabled(level: LogLevel): boolean {
    return this.#level >= level;
  }

  scoped(scope: string | null): ConsoleLogger {
    return new ConsoleLogger(this.#level, scope, this.#options);
  }

  with_level(level: LogLevel): ConsoleLogger {
    return new ConsoleLogger(level, this.#scope, this.#options);
  }

  with_options (options: Partial<Options>): ConsoleLogger {
    return new ConsoleLogger(this.#level, this.#scope, { ...this.#options, ...options })
  }

  debug(arg: any) {
    if(this.#level >= LogLevel.DEBUG) {
      console.log(`${this.#ts()}${color.blue(`DEBUG`)}${this.#s} ${arg}`)
    }
  }

  info(arg: any) {
    if(this.#level >= LogLevel.INFO) {
      console.log(`${this.#ts()}${color.green(`INFO`)} ${this.#s} ${arg}`)
    }
  }

  warn(arg: any) {
    if(this.#level >= LogLevel.WARN) {
      console.warn(`${this.#ts()}${color.yellow(`WARN`)} ${this.#s} ${arg}`)
    }
  }

  error(error: string | Error) {
    if(this.#level >= LogLevel.ERROR) {
      if(typeof error === "string") {
        console.warn(`${this.#ts()}${color.red(`ERROR`)}${this.#s} ${error}`);
      } else {
        const stack = error?.stack;
        if(stack) {
          const [head, _skip, ...rest] = stack.toString().split("\n");
          const ts = this.#ts();
          console.warn(`${ts}${color.red(`ERROR`)}${this.#s} ${head}`)
          for(const line of rest) {
            console.warn(`${ts} ${color.red("---")} ${this.#s} ${line.trim()}`)
          }
        } else {
          console.warn(`${this.#ts()}${color.red(`ERROR`)}${this.#s} ${error}`);
        }
      }
    }
  }
}

export const default_logger = new ConsoleLogger(env.LOG_LEVEL, null, { ts: env.LOG_TS });