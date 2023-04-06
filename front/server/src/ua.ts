import type { Request } from "express";

export const ua = (req: Request): string | null => {
  return req.header("user-agent")?.trim() || null;
}