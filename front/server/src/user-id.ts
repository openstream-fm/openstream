import type { Request } from "express";
import { Unauthorized } from "./error";

export const user_id = (req: Request): string => {
  const userId = req.cookie_session.user?._id;
  if(!userId) throw new Unauthorized("Session has expired", "FRONT_SESSION_EXPIRED");
  return userId;
}