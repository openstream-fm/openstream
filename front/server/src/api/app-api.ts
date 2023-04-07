import { Config } from "../config";
import { Router, json as json_body_parser } from "express";
import { ApiError, BadRequest, json_catch_handler } from "../error";
import { Logger } from "../logger";
import { json } from "../handler";
import { Client } from "../client";
import { session } from "../session";
import { ip } from "../ip";
import { token } from "../token";
import { user_id } from "../user-id";
import { ACCESS_TOKEN_HEADER, FORWARD_IP_HEADER } from "../constants";
import { StatusCodes } from "http-status-codes";
import { pipeline } from "stream/promises";
import crypto from "crypto";
import { mediakey } from "../media_key";
import { ua } from "../ua";

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
  api.use(session(config, logger));

  api.get("/online", (req, res) => {
    res.json({ ok: true })
  })

  api.get("/config", json(async () => {
    return config.public;
  }))

  api.post("/login", json(async (req, res) => {
    if(req.cookie_session.user != null) {
      await client.auth.user.logout(ip(req), ua(req), token(req)).catch(() => {});
    }

    {
      const r = await client.auth.user.login(ip(req), ua(req), req.body);
      const data = req.cookie_session;
      res.set_session({ ...data, user: { _id: r.user._id, token: r.token, media_key: r.media_key  } });
      return { user: r.user, media_key: r.media_key }
    }
  }))

  api.post("/logout", json(async (req, res) => {
    const r = await client.auth.user.logout(ip(req), ua(req), token(req)).catch(() => {});
    const data = req.cookie_session;
    res.set_session({ ...data, user: null });
    return r;
  }))

  api.post("/register", json(async (req, res) => {
    // invalidate previous token
    if(req.cookie_session.user != null) {
      await client.auth.user.logout(ip(req), ua(req), token(req)).catch(() => {});
    }

    {
      const { account, user, token, media_key } = await client.auth.user.register(ip(req), ua(req), config.openstream.token, req.body);
      const data = req.cookie_session;
      res.set_session({ ...data, user: { _id: user._id, token, media_key }});
      return { user, account }
    }
  }))

  api.get("/users/me", json(async req => {
    const { user } = await client.users.get(ip(req), ua(req), token(req), user_id(req))
    return { user,  media_key: mediakey(req) };
  }))

  api.get("/users/:user", json(async req => {
    return await client.users.get(ip(req), ua(req), token(req), req.params.user);
  }))

  api.route("/accounts")
    .get(json(async req => {
      return await client.accounts.list(ip(req), ua(req), token(req), req.query);
    }))
    .post(json(async req => {
      return await client.accounts.post(ip(req), ua(req), token(req), req.body);
    }))

  api.route("/accounts/:account")
    .get(json(async req => {
      return await client.accounts.get(ip(req), ua(req), token(req), req.params.id)
    }))
    .patch(json(async req => {
      return await client.accounts.patch(ip(req), ua(req), token(req), req.params.id, req.body);
    }))


  api.route("/stations")
    .get(json(async req => {
      return await client.stations.list(ip(req), ua(req), token(req), req.query);
    }))
    .post(json(async req => {
      return await client.stations.post(ip(req), ua(req), token(req), req.body);
    }))

  api.route("/stations/:station")
    .get(json(async req => {
      return await client.stations.get(ip(req), ua(req), token(req), req.params.station);
    }))
    .patch(json(async req => {
      return await client.stations.patch(ip(req), ua(req), token(req), req.params.station, req.body);
    }))

  api.post("/stations/:station/restart-playlist", json(async req => {
    return await client.stations.restart_playlist(ip(req), ua(req), token(req), req.params.station);
  }))

  api.get("/stations/:station/limits", json(async req => {
    const { station: { limits } } = await client.stations.get(ip(req), ua(req), token(req), req.params.station);
    return limits;
  }))

  api.route("/stations/:station/files")
    .get(json(async req => {
      return await client.stations.files.list(ip(req), ua(req), token(req), req.params.station, req.query)
    }))

    .post(json(async req => {
      const content_type = req.header("content-type") ?? "application/octet-stream";
      const content_length = Number(req.header("content-length"));
      if(!content_length) {
        throw new BadRequest("Content length must be specified (front)", "CONTENT_LENGTH_REQUIRED");
      }
      return await client.stations.files.post(ip(req), ua(req), token(req), req.params.station, content_type, content_length, req.query as any, req);
    }))

  api.route("/stations/:station/files/shuffle")
    .post(json(async req => {
      return await client.stations.files.shuffle(ip(req), ua(req), token(req), req.params.station);
    }));

  api.route("/stations/:station/files/unshuffle")
    .post(json(async req => {
      return await client.stations.files.unshuffle(ip(req), ua(req), token(req), req.params.station);
    }));

  api.route("/stations/:station/files/:file")
    .get(json(async req => {
      return await client.stations.files.get(ip(req), ua(req), token(req), req.params.station, req.params.file);
    }))
    .delete(json(async req => {
      return await client.stations.files.delete(ip(req), ua(req), token(req), req.params.station, req.params.file);
    }))

  api.route("/stations/:station/files/:file/order/swap")
    .post(json(async req => {
      return await client.stations.files.swap_order(ip(req), ua(req), token(req), req.params.station, req.params.file, req.body);
    }))

  api.route("/stations/:station/files/:file/order/move-before")
    .post(json(async req => {
      return await client.stations.files.move_before(ip(req), ua(req), token(req), req.params.station, req.params.file, req.body);
    }))
  
  api.route("/stations/:station/files/:file/order/move-after")
    .post(json(async req => {
      return await client.stations.files.move_after(ip(req), ua(req), token(req), req.params.station, req.params.file, req.body);
    }))

  api.route("/stations/:station/files/:file/order/move-to-first")
    .post(json(async req => {
      return await client.stations.files.move_to_first(ip(req), ua(req), token(req), req.params.station, req.params.file);
    }))

  api.route("/stations/:station/files/:file/order/move-to-last")
    .post(json(async req => {
      return await client.stations.files.move_to_last(ip(req), ua(req), token(req), req.params.station, req.params.file);
    }))


    
  api.route("/stations/:station/files/:file/metadata")
    .put(json(async req => {
      return await client.stations.files.put_metadata(ip(req), ua(req), token(req), req.params.station, req.params.file, req.body);
    }))

  api.route("/stations/:station/now-playing")
    .get(json(async req => {
      return await client.stations.get_now_playing(ip(req), ua(req), token(req), req.params.station);
    }))

  api.route("/stations/:station/dashboard-stats")
    .get(json(async req => {
      return await client.stations.get_dashboard_stats(ip(req), ua(req), token(req), req.params.station);
    }))

  // TODO: deprecate this endpoint (go directly to storage rs backend)
  api
    .route("/stations/:station/files/:file/stream")
    .get(async (req, res, next) => {
  
      try {
      
        const { station, file } = req.params;

        const headers: Record<string, string> = Object.create(null);
        for(const key of [ "user-agent", "if-none-match", "accept", "accept-language", "range" ]) {
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

  api.route("/station-pictures")
    .post(json(async req => {
      return await client.stations.pictures.post(ip(req), ua(req), token(req), req.query as any, req)
    }));

  api.use(json(() => {
    throw new ApiError(StatusCodes.NOT_FOUND, "FRONT_RESOURCE_NOT_FOUND", "Resource not found");
  }))

  api.use(json_catch_handler(logger));

  return api;

}