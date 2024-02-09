import type { Request } from "express";
import { Unauthorized } from "./error.js";

export const admin_id = (req: Request): string => {
  const adminId = req.cookie_session.admin?._id;
  if(!adminId) throw new Unauthorized("Session has expired", "FRONT_SESSION_EXPIRED");
  return adminId;
}