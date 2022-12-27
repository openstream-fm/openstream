import type { Request } from "express";
import { Unauthorized } from "./error";
import "./auth";

export const userId = (req: Request): string => {
  const userId = req.session.user?._id;
  if(!userId) throw new Unauthorized("Session has expired", "FRONT_SESSION_EXPIRED");
  return userId;
}