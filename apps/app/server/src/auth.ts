import type { Request } from "express";

declare module "express-session" {
  export interface SessionData {
    user?: { _id: string, token: string } | null,
    admin?: { _id: string, token: string } | null,
  }
}

declare module "express" {
  export interface Request {

  }
}