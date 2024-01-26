import StatusCode from "http-status-codes";
import { ClientError } from "./client.server";
import type { Logger } from "./logger";
import type { Request, Response, NextFunction } from "express";
import type { ErrorCode } from "./types";
import type { PublicErrorCode } from "./defs/error/PublicErrorCode";

export class ApiError extends Error {
  
  status: number;
  code: ErrorCode;

  constructor(status: number, code: ErrorCode, message: string) {
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

  static from_error_payload(payload: any): ApiError {
    const status = payload?.status;
    const message = payload?.message;
    const code = payload?.code;
    
    if(typeof status === "number" && typeof message === "string" && typeof code === "string") {
      return new ApiError(status, code as PublicErrorCode, message);
    } else {
      return new ApiError(StatusCode.BAD_GATEWAY, "FRONT_GATEWAY_MISSING_CODE", "Gateway error (missing code)")
    }
  }
}

const Err = (status: number, default_code: ErrorCode) => {
  return class extends ApiError {
    
    static DEFAULT_CODE = default_code;
    static STATUS = status;

    constructor(message: string, code = default_code) {
      super(status, code, message);
    }
  }
}

export const Internal = Err(StatusCode.INTERNAL_SERVER_ERROR, "FRONT_INTERNAL");
export const BadRequest = Err(StatusCode.BAD_REQUEST, "FRONT_BAD_REQUEST");
export const Unauthorized = Err(StatusCode.UNAUTHORIZED, "FRONT_UNAUTHORIZED");

export const json_catch_handler = (logger: Logger) => {
  return (e: Error, req: Request, res: Response, _next: NextFunction) => {
    const error = ApiError.from(e);
    logger.warn(`API Error: ${req.originalUrl} ${req.path} => ${error.status} ${error.code} ${e?.message}`)
    if(!res.headersSent) {
      res.status(error.status).json(error);
    }
  }
}