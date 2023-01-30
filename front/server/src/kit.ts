import { RequestHandler } from "express";
import { ip } from "./ip";
import { X_REAL_IP } from "./constants";

export const kit = (handler: RequestHandler): RequestHandler => {
  return (req, res, next) => {
    req.headers[X_REAL_IP] = ip(req);
    return handler(req, res, next);
  }
}