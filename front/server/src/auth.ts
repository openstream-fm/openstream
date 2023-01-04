import type { Request } from "express";

declare module "express-session" {
  export interface SessionData {
    user?: { token: string, _id: string } | null
  }
}

declare module "express" {
  export interface Request {

  }
}