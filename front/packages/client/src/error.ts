import type { PublicErrorCode } from "./defs/error/PublicErrorCode";

export type ClientErrorCode = 
  | PublicErrorCode
  | "CLIENT_GATEWAY_FETCH" 
  | "CLIENT_GATEWAY_JSON"
  | "CLIENT_GATEWAY_MISSING_CODE";

export class ClientError extends Error {
  status: number
  code: ClientErrorCode

  constructor(status: number, code: ClientErrorCode, message: string) {
    super(message);
    this.status = status;
    this.code = code;
  }
}
