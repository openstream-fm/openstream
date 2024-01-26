import type { RequestHandler } from "express";
import { ip } from "./ip";
import { REAL_IP_HEADER } from "./constants";

export const kit = (handler: RequestHandler): RequestHandler => {
  return (req, res, next) => {
    req.headers[REAL_IP_HEADER] = ip(req);
    return handler(req, res, next);
  }
}