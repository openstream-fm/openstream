import StatusCode from "http-status-codes";
import { ClientError } from "./client";
import { Logger } from "./logger";
import type { Request, Response, NextFunction } from "express";

export class ApiError extends Error {
  
  status: number;
  code: string;

  constructor(status: number, code: string, message: string) {
    super(message);
    this.status = status;
    this.code = code;
  }

  toJSON() {
    return {
      error: {
        status: this.status,
        message: this.message,
        code: this.code,
      }
    }
  }

  static from(e: any): ApiError {
    if(e instanceof ApiError) {
      return e;
    } else if(e instanceof ClientError) {
      return new ApiError(e.status, e.code, e.message);
    } else {
      return new Internal("Internal server error");
    }
  }
}

const Err = (status: number, default_code: string) => {
  return class extends ApiError {
    
    static DEFAULT_CODE = default_code;
    static STATUS = status;

    constructor(message: string, code = default_code) {
      super(status, code, message);
    }
  }
}

export const Internal = Err(StatusCode.INTERNAL_SERVER_ERROR, "ERR_INTERNAL");
export const BadRequest = Err(StatusCode.BAD_REQUEST, "ERR_BAD_REQUEST");
export const Unauthorized = Err(StatusCode.UNAUTHORIZED, "ERR_UNAUTHORIZED");

export const json_catch_handler = (logger: Logger) => {
  return (e: Error, req: Request, res: Response, next: NextFunction) => {
    const error = ApiError.from(e);
    logger.warn(`API Error: ${req.method} ${req.path} => ${error.status} ${e}`)
    res.status(error.status).json(error);
  }
}