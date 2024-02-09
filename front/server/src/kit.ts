import type { RequestHandler } from "express";
import { ip } from "./ip.js";
import { REAL_IP_HEADER } from "./constants.js";

export const kit = (handler: RequestHandler): RequestHandler => {
  return (req, res, next) => {
    req.headers[REAL_IP_HEADER] = ip(req);
    return handler(req, res, next);
  }
}