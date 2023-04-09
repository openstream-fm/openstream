import cookieParser from "cookie-parser";
import { NextFunction, Request, Response, Router } from "express";
import { is } from "typia";
import { Config } from "./config";
import crypto from "crypto";
import { Logger } from "./logger";

const COOKIE_NAME = "openstream-front.sid";

export type SessionData = {
  user: { _id: string, token: string, media_key: string } | null;
}

const ALGO = "aes-192-cbc"

export function encrypt(value: string, key: Buffer, logger: Logger): string {
  try {
    const iv = crypto.randomBytes(16);
    const cipher = crypto.createCipheriv(ALGO, key, iv);
    const hash = cipher.update(value, "utf8", "base64") + cipher.final("base64");
    const encrypted = `${iv.toString("hex")}.${hash}`;
    // logger.info(`ecrypted: ${encrypted}`)
    return encrypted;
  } catch(e) {
    logger.warn(`encrypt error: ${e}`)
    throw e;
  }
}

export function decrypt(hash: string, key: Buffer, logger: Logger): string {
  try {
    const [ivhex, base64] = hash.split(".");
    if(!ivhex || !base64) throw new Error("malformed hash: iv or hash missing");
    const iv = Buffer.from(ivhex, "hex");
    const decipher = crypto.createDecipheriv(ALGO, key, iv);
    const value = decipher.update(base64, "base64", "utf8") + decipher.final("utf8");
    // logger.info(`decrypted: ${value}`);
    return value;
  } catch(e) {
    logger.warn(`decrypt error: ${e}`)
    throw e;
  }
}

const get_cookie_session = (req: Request, name: string, key: Buffer, logger: Logger): SessionData => {
  try {
    const v = req.cookies[name];
    logger.debug(`v: ${v}`)
    if(typeof v !== "string") {
      logger.debug(`not string, ${typeof v}`)
      return { user: null };
    }
    const json_string = decrypt(v, key, logger);
    let data: any;
    try {
      data = JSON.parse(json_string);
    } catch (e) {
      logger.warn(`json parse error: JSON.parse('${json_string}'): ${e}`)
    }

    if(is<SessionData>(data)) {
      return data;
    } else {
      logger.warn(`not is<SessionData>, ${JSON.stringify(data)}`)
      return { user: null };
    }
  } catch(e) {
    logger.warn(`error: ${e}`)
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

export const session = (config: Config, _logger: Logger) => {
  const logger = _logger.scoped("cookie-session");
  const key = crypto.scryptSync(config.session.secret, "salt", 24);

  const router = Router();
  router.use(cookieParser());
  router.use((req: Request, res: Response, next: NextFunction) => {
    req.cookie_session = get_cookie_session(req, config.session.cookieName, key, logger);
    res.set_session = (data: SessionData) => {
      const encoded = encrypt(JSON.stringify(data), key, logger);
      res.cookie(config.session.cookieName, encoded, {
        domain: config.session.domain,
        httpOnly: true,
        maxAge: config.session.maxAgeDays * 1000 * 60 * 60 * 24,
        sameSite: "strict",
        signed: false,
      });
    }
    res.clear_session = () => res.clearCookie(config.session.cookieName, {
      domain: config.session.domain,
      httpOnly: true,
      sameSite: "strict",
      signed: false,
    });
    next();
  })

  return router;
}