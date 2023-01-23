import type { Request } from "express";
import { Unauthorized } from "./error";

export const mediakey = (req: Request): string => {
  const media_key = req.cookie_session.user?.media_key;
  if(!media_key) throw new Unauthorized("Session has expired", "FRONT_SESSION_EXPIRED");
  return media_key;
}