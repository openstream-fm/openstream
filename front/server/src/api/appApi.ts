import { Config } from "../config";
import { Router, json as json_body } from "express";
import { ApiError, BadRequest, json_catch_handler } from "../error";
import { Logger } from "../logger";
import { json } from "../handler";
import { Client } from "../client";
import { saveSession, session } from "../session";
import { ip } from "../ip";
import { token } from "../token";
import "../auth";
import { userId } from "../userId";
import { ACCESS_TOKEN_HEADER, FORWARD_IP_HEADER } from "../contants";
import { StatusCodes } from "http-status-codes";
import { pipeline } from "stream/promises";

export const appApi = ({
  config,
  logger: _logger,
}: {
  config: Config,
  logger: Logger,
}) => {

  const client = new Client(config.openstream.apiBaseURL, { logger: _logger });

  const logger = _logger.scoped("app-api");

  let api = Router();
  api.use(json_body())
  api.use(session(config));

  api.post("/login", json(async req => {
    //const { email, password } = validate(() => assertType<import("../defs/api/login/POST/Payload").Payload>(req.body));
    const { token, user } = await client.auth.user.login(ip(req), req.body);
    req.session.user = { token, _id: user._id };
    await saveSession(req);
    return { user }
  }))

  api.post("/logout", json(async req => {
    const r = await client.auth.user.logout(ip(req), token(req));
    req.session.user = null;
    await saveSession(req);
    return r;
  }))

  api.post("/register", json(async req => {
    //const payload = validate(() => assertType<import("../defs/api/register/POST/Payload").Payload>(req.body));
    const { account, token, user } = await client.auth.user.register(ip(req), config.openstream.token, req.body);
    req.session.user = { token, _id: user._id };
    await saveSession(req);
    return { account, user }
  }))

  api.get("/users/me", json(async req => {
    return await client.users.get(ip(req), token(req), userId(req))
  }))

  api.get("/users/:user", json(async req => {
    return await client.users.get(ip(req), token(req), req.params.user);
  }))

  api.get("/accounts", json(async req => {
    return await client.accounts.list(ip(req), token(req), req.query);
  }))

  api.get("/accounts/:account", json(async req => {
    return await client.accounts.get(ip(req), token(req), req.params.id);
  }))

  api.route("/accounts/:account/files")
    .get(json(async req => {
      return await client.accounts.files.list(ip(req), token(req), req.params.account, req.query)
    }))

    .post(json(async req => {
      const contentType = req.header("content-type") ?? "application/octet-stream";
      const contentLength = Number(req.header("content-length"));
      if(!contentLength) {
        throw new BadRequest("Content length must be specified (front)", "CONTENT_LENGTH_REQUIRED");
      }
      return await client.accounts.files.post(ip(req), token(req), req.params.account, contentType, contentLength, req.query as any, req);
    }))

  api.route("/accounts/:account/files/:file")
    .get(json(async req => {
      return await client.accounts.files.get(ip(req), token(req), req.params.account, req.params.file);
    }))
    .delete(json(async req => {
      return await client.accounts.files.delete(ip(req), token(req), req.params.account, req.params.file);
    }))

  api.get("/accounts/:account/files/:file/stream", async (req, res, next) => {
  
    try {
    
      const { account, file } = req.params;

      const headers: Record<string, string> = Object.create(null);
      for(const key of [ "if-none-match", "accept", "accept-language", "range" ]) {
        const value = req.header(key);
        if(value) headers[key] = value;
      }

      headers[FORWARD_IP_HEADER] = ip(req);
      headers[ACCESS_TOKEN_HEADER] = token(req);

      const back = await client.fetch(`/accounts/${account}/files/${file}/stream`, {
        method: "GET",
        headers,
      })

      res.status(back.status);

      for(const key of ["etag", "content-type", "content-length", "content-language", "accept-ranges", "content-range"]) {
        const value = back.headers.get(key);
        if(value != null) {
          res.header(key, value);
        }
      }

      res.header("vary", "range");

      if(back.body) {
        await pipeline(back.body, res);
      } else {
        res.end();
      }
    } catch(e) {
      next(e)
    }
  })

  api.use(json(() => {
    throw new ApiError(StatusCodes.NOT_FOUND, "FRONT_RESOURCE_NOT_FOUND", "Resource not found");
  }))

  api.use(json_catch_handler(logger));

  return api;

}