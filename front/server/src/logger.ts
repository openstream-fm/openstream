import { color } from "./color";

export enum LogLevel {
  SILENT = 0,
  ERROR = 1,
  WARN = 2,
  INFO = 3,
  DEBUG = 4,
}

export interface Logger {
  error: (error: Error) => void,
  warn: (arg: any) => void,
  info: (arg: any) => void,
  debug: (arg: any) => void,
  enabled: (level: LogLevel) => boolean,
  scoped: (scope: string | null) => Logger 
}

export class ConsoleLogger implements Logger {
  
  #level: LogLevel;
  #scope: string | null;
  #s: string;

  constructor(level: LogLevel, scope: string | null = null) {
    
    this.#level = level;
    this.#scope = scope;

    if(this.#scope != null) {
      this.#s = ` ${color.bold(this.#scope)} >`
    } else {
      this.#s = "";
    }
  }

  get level() {
    return this.#level;
  }

  enabled(level: LogLevel) {
    return this.#level >= level;
  }

  scoped(scope: string | null): ConsoleLogger {
    return new ConsoleLogger(this.#level, scope);
  }

  debug(arg: any) {
    if(this.#level >= LogLevel.DEBUG) {
      console.log(`${color.blue(`[DEBUG]:`)}${this.#s} ${arg}`)
    }
  }

  info(arg: any) {
    if(this.#level >= LogLevel.INFO) {
      console.log(`${color.green(`[INFO]:`)}${this.#s} ${arg}`)
    }
  }

  warn(arg: any) {
    if(this.#level >= LogLevel.WARN) {
      console.log(`${color.yellow(`[WARN]:`)}${this.#s} ${arg}`)
    }
  }

  error(error: Error) {
    if(this.#level >= LogLevel.ERROR) {
      console.warn(`${color.red(`[ERROR]:`)}${this.#s} ${error?.stack}`)
    }
  }
}