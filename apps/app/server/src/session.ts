import _session from "express-session";
import MongoDBSession from "connect-mongodb-session";
import { Config } from "./config";
import type { Request } from "express"

const SessionStore = MongoDBSession(_session);

const store = (config: Config) => new SessionStore({
  uri: config.mongodb.url,
  collection: "sessions",
})

const sessionConfig = (config: Config) => {
  return {
    secret: config.session.secret,
    cookie: { domain: config.session.domain, maxAge: 1000 * 60 * 60 * 24 * 30 },
    store: store(config),
    resave: true,
    saveUninitialized: false,
  }
}

export const session = (config: Config) => _session(sessionConfig(config));

export const saveSession = async (req: Request) => {
  return new Promise<void>((resolve, reject) => {
    req.session!.save(e => {
      if(e) reject(e);
      else resolve();
    })
  })
}