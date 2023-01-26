import type { Request } from "express";
import { Unauthorized } from "./error";

export const token = (req: Request): string => {
  const token = req.cookie_session.user?.token;
  if(!token) throw new Unauthorized("Session has expired", "FRONT_SESSION_EXPIRED");
  return token;
}