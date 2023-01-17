import { Config } from "../config";
import { Router, json as json_body_parser } from "express";
import { ApiError, BadRequest, json_catch_handler } from "../error";
import { Logger } from "../logger";
import { json } from "../handler";
import { Client } from "../client";
import { save_session, session } from "../session";
import { ip } from "../ip";
import { token } from "../token";
import { user_id } from "../user-id";
import { ACCESS_TOKEN_HEADER, FORWARD_IP_HEADER } from "../contants";
import { StatusCodes } from "http-status-codes";
import { pipeline } from "stream/promises";
import "../auth";

export const app_api = ({
  config,
  logger: _logger,
}: {
  config: Config,
  logger: Logger,
}) => {

  const client = new Client(config.openstream.apiBaseURL, { logger: _logger });

  const logger = _logger.scoped("app-api");

  let api = Router();
  api.use(json_body_parser())
  api.use(session(config));

  api.post("/login", json(async req => {
    //const { email, password } = validate(() => assertType<import("../defs/api/login/POST/Payload").Payload>(req.body));
    const { token, user } = await client.auth.user.login(ip(req), req.body);
    req.session.user = { token, _id: user._id };
    await save_session(req);
    return { user }
  }))

  api.post("/logout", json(async req => {
    const r = await client.auth.user.logout(ip(req), token(req));
    req.session.user = null;
    await save_session(req);
    return r;
  }))

  api.post("/register", json(async req => {
    //const payload = validate(() => assertType<import("../defs/api/register/POST/Payload").Payload>(req.body));
    const { station, token, user } = await client.auth.user.register(ip(req), config.openstream.token, req.body);
    req.session.user = { token, _id: user._id };
    await save_session(req);
    return { station, user }
  }))

  api.get("/users/me", json(async req => {
    return await client.users.get(ip(req), token(req), user_id(req))
  }))

  api.get("/users/:user", json(async req => {
    return await client.users.get(ip(req), token(req), req.params.user);
  }))

  api.get("/stations", json(async req => {
    return await client.stations.list(ip(req), token(req), req.query);
  }))

  api.get("/stations/:station", json(async req => {
    return await client.stations.get(ip(req), token(req), req.params.station);
  }))

  api.get("/stations/:station/limits", json(async req => {
    const { station: { limits } } = await client.stations.get(ip(req), token(req), req.params.station);
    return limits;
  }))

  api.route("/stations/:station/files")
    .get(json(async req => {
      return await client.stations.files.list(ip(req), token(req), req.params.station, req.query)
    }))

    .post(json(async req => {
      const content_type = req.header("content-type") ?? "application/octet-stream";
      const content_length = Number(req.header("content-length"));
      if(!content_length) {
        throw new BadRequest("Content length must be specified (front)", "CONTENT_LENGTH_REQUIRED");
      }
      return await client.stations.files.post(ip(req), token(req), req.params.station, content_type, content_length, req.query as any, req);
    }))

  api.route("/stations/:station/files/:file")
    .get(json(async req => {
      return await client.stations.files.get(ip(req), token(req), req.params.station, req.params.file);
    }))
    .delete(json(async req => {
      return await client.stations.files.delete(ip(req), token(req), req.params.station, req.params.file);
    }))

  api.route("/stations/:station/files/:file/metadata")
    .put(json(async req => {
      return await client.stations.files.put_metadata(ip(req), token(req), req.params.station, req.params.file, req.body);
    }))

  api.route("/stations/:station/now-playing")
    .get(json(async req => {
      return await client.stations.get_now_playing(ip(req), token(req), req.params.station);
    }))

  api.route("/stations/:station/dashboard-stats")
    .get(json(async req => {
      return await client.stations.get_dashboard_stats(ip(req), token(req), req.params.station);
    }))

  api
    .route("/stations/:station/files/:file/stream")
    .get(async (req, res, next) => {
  
      try {
      
        const { station, file } = req.params;

        const headers: Record<string, string> = Object.create(null);
        for(const key of [ "if-none-match", "accept", "accept-language", "range" ]) {
          const value = req.header(key);
          if(value) headers[key] = value;
        }

        headers[FORWARD_IP_HEADER] = ip(req);
        headers[ACCESS_TOKEN_HEADER] = token(req);

        const back = await client.fetch(`/stations/${station}/files/${file}/stream`, {
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