import type { Request, Response, NextFunction } from "express";
import type { PaymentsErrorKind } from "../defs/payments/api/PaymentsErrorKind.js";
import type { PaymentsErrorPayload } from "../defs/payments/api/PaymentsErrorPayload.js";
import type { Logger } from "../logger.js";
import { StatusCodes } from "http-status-codes";

export const ERROR_STATUS_CODE = StatusCodes.IM_A_TEAPOT;
export class PaymentsClientError extends Error {
  
  detail: PaymentsErrorKind;

  constructor(message: string, detail: PaymentsErrorKind, cause?: any) {
    super(message, { cause });
    this.detail = detail;
  }

  static from(e: any): PaymentsClientError {
    if(e instanceof PaymentsClientError) return e;
    const message = String(e?.message);
    return new this(message, { kind: "unknown" });
  }

  to_payload = (): PaymentsErrorPayload => {
    return {
      error: {
        message: this.message,
        ...this.detail
      }
    }
  }
}

export const operation_rethrow = (e: any): never => {
  throw new PaymentsClientError(String(e?.message), { kind: "provider", provider_error_type: String(e?.type || "") || null }, e);
}

export const validate_rethrow = <T>(fn: () => T): T => {
  try {
    return fn()
  } catch(e: any) {
    throw new PaymentsClientError(String(e?.message), { kind: "payload" }, e);
  }
}


export const not_found_handler = () => {
  throw new PaymentsClientError("resource not found", { kind: "resource-not-found" });
}

export const access_token_error = (kind: "access-token-not-present" | "access-token-mismatch"): PaymentsClientError => {
  const message = kind === "access-token-not-present" ? "Access token not present" : "Access token mismatch";
  return new PaymentsClientError(message, { kind })
}

export const catch_handler = ({ logger }: { logger: Logger }) => {
  return (src_error: any, req: Request, res: Response, next: NextFunction) => {
    const target_error = PaymentsClientError.from(src_error);
    if(target_error === src_error) {
      logger.warn(`error at endpoint ${req.path} => ${target_error.detail.kind} => ${target_error.message}${target_error.cause ? ` => cause: ${String(target_error.cause)}` : ""}`)
    } else {
      logger.error(`unhandled error at endpoint ${req.path}`);
      logger.error("source error");
      logger.error(src_error)
      logger.error("target error")
      logger.error(PaymentsClientError.from(src_error));
    }

    if(!res.headersSent) {
      try {
        res.status(ERROR_STATUS_CODE).json(target_error.to_payload());
      } catch(e) {
        logger.warn(`error sending error json payload: ${String(e)}`)
      }
    }
  }
}