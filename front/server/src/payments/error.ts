import type { Request, Response, NextFunction } from "express";
import type { PaymentsErrorKind } from "../defs/payments/api/PaymentsErrorKind";
import { PaymentsErrorPayload } from "../defs/payments/api/PaymentsErrorPayload";
import { Logger } from "../logger";


export class PaymentsClientError extends Error {
  
  kind: PaymentsErrorKind;

  constructor(message: string, kind: PaymentsErrorKind, cause?: any) {
    super(message, { cause });
    this.kind = kind;
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
        ...this.kind
      }
    }
  }

  
}

export const operation_rethrow = (e: any): never => {
  throw new PaymentsClientError(String(e?.message), { kind: "provider" }, e);
}

export const validate_rethrow = (e: any): never => {
  throw new PaymentsClientError(String(e?.message), { kind: "payload" }, e);
} 

export const not_found_catch = () => {
  throw new PaymentsClientError("resource not found", { kind: "resource-not-found" });
}

export const catch_handler = ({ logger }: { logger: Logger }) => {
  return (src_error: any, req: Request, res: Response, next: NextFunction) => {
    const target_error = PaymentsClientError.from(src_error);
    if(target_error === src_error) {
      logger.warn(`error at endpoint ${req.path} => ${target_error.kind.kind} => ${target_error.message}${target_error.cause ? ` => cause: ${String(target_error.cause)}` : ""}`)
    } else {
      logger.error(`unhandled error at endpoint ${req.path}`);
      logger.error("source error");
      logger.error(src_error)
      logger.error("target error")
      logger.error(PaymentsClientError.from(src_error));
    }
  }
}