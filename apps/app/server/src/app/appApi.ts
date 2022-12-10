import { Config } from "../config";
import { Request, Response, NextFunction, Router, json as json_body } from "express";
import { ApiError, json_catch_handler } from "../error";
import { Logger } from "../logger";
import { json } from "../handler";
import { validate } from "../validate";
import { assertType } from "typescript-is";
import { Client } from "../client";
import { saveSession, session } from "../session";

export const api = ({
  config,
  logger: _logger,
}: {
  config: Config,
  logger: Logger,
}) => {

  const client = new Client(config.openstream.apiBaseURL);

  const logger = _logger.scoped("appApi");

  let api = Router();
  api.use(json_body())
  api.use(session(config));

  api.post("/login", json(async req => {
    const { email, password } = validate(() => assertType<import("../defs/api/login/POST/Payload").Payload>(req.body));
    const { token, user } = await client.login({ email, password });
    req.session.user = { _id: user._id, token };
    await saveSession(req);    
    return { user }
  }))

  api.post("/register", async req => {
    const payload = validate(() => assertType<import("../defs/api/register/POST/Payload").Payload>(req.body));
    const { account, token, user } = await client.register(config.openstream.token, payload);
    req.session.user = { _id: user._id, token };
    await saveSession(req);
    return { account, user }
  })
  
  const pages = Router();
  api.use("/pages", pages);

  pages.get("/session", json(async req => {
    const token = req.session.user?.token ?? null;
    const userId = req.session.user?._id ?? null;
    if(userId && token) {
      try {
        const user = await client.users.get(token, userId);
        const accounts = await client.accounts.list(token, { skip: 0, limit: 100 });
        return { user, accounts }
      } catch(e) {
        logger.warn(`error getting session: ${e}`)
      }
    }

    return { user: null }
  }));

  api.use(json_catch_handler(logger));

  return api;

}