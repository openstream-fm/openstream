import { PublicErrorCode } from "./defs/error/PublicErrorCode";

export type ErrorCodeMore = 
  "FRONT_INTERNAL" |
  "FRONT_BAD_REQUEST" |
  "FRONT_UNAUTHORIZED" |
  "FRONT_INVALID_PAYLOAD" |
  "FRONT_GATEWAY_FETCH" | 
  "FRONT_GATEWAY_JSON" |
  "FRONT_GATEWAY_MISSING_CODE" |
  "FRONT_SESSION_EXPIRED" |
  "FRONT_RESOURCE_NOT_FOUND";

export type ErrorCode = PublicErrorCode | ErrorCodeMore;