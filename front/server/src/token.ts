import type { Request } from "express";
import { Unauthorized } from "./error";

export const user_token = (req: Request): string => {
  const token = req.cookie_session.user?.token;
  if(!token) throw new Unauthorized("Session has expired", "FRONT_SESSION_EXPIRED");
  return token;
}

export const admin_token = (req: Request): string => {
  const token = req.cookie_session.admin?.token;
  if(!token) throw new Unauthorized("Session has expired", "FRONT_SESSION_EXPIRED");
  return token;
}