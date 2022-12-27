import type { Request } from "express";
import { FORWARD_IP_HEADER, LOCALHOST, X_REAL_IP } from "./contants";

export const ip = (req: Request): string => {
  let ip = req.ip;

  if(LOCALHOST.includes(ip)){
    const v = req.header(X_REAL_IP); 
    if(v) ip = v;
  }

  if(LOCALHOST.includes(ip)){
    const v = req.header(FORWARD_IP_HEADER);
    if(v) ip = v;
  }

  return ip
}