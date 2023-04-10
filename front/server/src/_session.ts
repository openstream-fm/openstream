import _session from "express-session";
import MongoDBSession from "connect-mongodb-session";
import { Config } from "./config";
import type { Request } from "express"

const SessionStore = MongoDBSession(_session);

const store = (config: Config) => new SessionStore({
  uri: config.mongodb.url,
  collection: "sessions",
})

const session_config = (config: Config): Parameters<typeof _session>[0] => {
  return {
    name: "openstream-front.sid",
    secret: config.session.secret,
    cookie: {
      // domain: config.session.domain,
      maxAge: config.session.max_age_days * 24 * 60 * 60 * 1000
    },
    store: store(config),
    rolling: true,
    resave: false,
    saveUninitialized: false,
  }
}

export const session = (config: Config) => _session(session_config(config));

export const save_session = async (req: Request) => {
  return new Promise<void>((resolve, reject) => {
    req.session!.save(e => {
      if(e) reject(e);
      else resolve();
    })
  })
}