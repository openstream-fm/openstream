import type { Request } from "express";
import { Unauthorized } from "./error";

export const user_media_key = (req: Request): string => {
  const media_key = req.cookie_session.user?.media_key;
  if(!media_key) throw new Unauthorized("Session has expired", "FRONT_SESSION_EXPIRED");
  return media_key;
}

export const admin_media_key = (req: Request): string => {
  const media_key = req.cookie_session.admin?.media_key;
  if(!media_key) throw new Unauthorized("Session has expired", "FRONT_SESSION_EXPIRED");
  return media_key;
}