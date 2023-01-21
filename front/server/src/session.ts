import cookieParser from "cookie-parser";
import { NextFunction, Request, Response, Router } from "express";
import { is } from "typia";
import { Config } from "./config";
import crypto from "crypto";
import { Logger } from "./logger";

const COOKIE_NAME = "openstream-front.sid";

export type SessionData = {
  user: { _id: string, token: string } | null;
}

const ALGO = "aes-256-ctr"

export function encrypt(value: string, iv: Buffer, key: Buffer, logger: Logger): string {
  try {
    const cipher = crypto.createCipheriv(ALGO, key, iv)
    const start = cipher.update(value, "utf8", "base64")
    return start + cipher.final("base64")
  } catch(e) {
    logger.scoped("cookie-session").warn(`encrypt error: ${e}`)
    throw e;
  }
}

export function decrypt(base64: string, iv: Buffer, key: Buffer, logger: Logger): string {
  try {
    const decipher = crypto.createDecipheriv(ALGO, key, iv);
    const start = decipher.update(base64, "base64", "utf8");
    return start + decipher.final("utf8")
  } catch(e) {
    logger.scoped("cookie-session").warn(`decrypt error: ${e}`)
    throw e;
  }
}

const get_cookie_session = (req: Request, iv: Buffer, key: Buffer, logger: Logger): SessionData => {
  try {
    const v = req.cookies[COOKIE_NAME];
    if(typeof v !== "string") return { user: null };
    const json_string = decrypt(v, iv, key, logger);
    let data: any;
    try {
      data = JSON.parse(json_string);
    } catch (e) {
      logger.scoped("cookie-session").warn(`json parse error: JSON.parse('${json_string}'): ${e}`)
    }
    if(is<SessionData>(data)) {
      return data;
    } else {
      return { user: null };
    }
  } catch(e) {
    return { user: null }
  }
}

declare global {
  namespace Express {
    interface Request {
      cookie_session: SessionData;
    }

    interface Response {
      set_session: (data: SessionData) => void;
      clear_session: () => void;
    }
  }
}

export const session = (config: Config, logger: Logger) => {
  const iv = crypto.createHash("md5").update(config.session.secret).digest();
  const key = crypto.createHash("sha256").update(config.session.secret).digest();
  

  const router = Router();
  router.use(cookieParser());
  router.use((req: Request, res: Response, next: NextFunction) => {
    req.cookie_session = get_cookie_session(req, iv, key, logger);
    res.set_session = (data: SessionData) => {
      res.cookie(
        COOKIE_NAME,
        encrypt(JSON.stringify(data), iv, key, logger), { 
          maxAge: config.session.maxAgeDays + 1000 * 60 * 60 * 24,
          httpOnly: true,
          sameSite: "strict",
        })
    }
    res.clear_session = () => res.clearCookie(COOKIE_NAME, { httpOnly: true, signed: true });
    next();
  })

  return router;
}