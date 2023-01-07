import type { ErrorCode } from "./types";

export class ClientError extends Error {
  status: number
  code: ErrorCode

  constructor(status: number, code: ErrorCode, message: string) {
    super(message);
    this.status = status;
    this.code = code;
  }
}
