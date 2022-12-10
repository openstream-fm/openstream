import type { Request } from "express";
import { Unauthorized } from "./error.js";

export const userToken = (req: Request): string | null => {
  const token = req.session?.user?.token;
  if(token == null) {
    throw new Unauthorized("Session has expired", "FRONT_SESSION_EXPIRED");
  }
  return token;
}