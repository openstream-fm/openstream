import type { Request } from "express";
import { Unauthorized } from "./error";

export const userToken = (req: Request): string | null => {
  const token = req.session?.user?.token;
  if(token == null) {
    throw new Unauthorized("Session has expired", "ERR_SESSION_EXPIRED");
  }
  return token;
}