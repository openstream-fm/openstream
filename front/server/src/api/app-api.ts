import type { Config } from "../config";
import { Router, json as json_body_parser } from "express";
import { ApiError, BadRequest, json_catch_handler } from "../error";
import type { Logger } from "../logger";
import { json } from "../handler";
import { Client } from "../client";
import { session } from "../session";
import { ip } from "../ip";
import { token } from "../token";
import { user_id } from "../user-id";
import { ACCESS_TOKEN_HEADER, FORWARD_IP_HEADER } from "../constants";
import { StatusCodes } from "http-status-codes";
import { pipeline } from "stream/promises";
import { mediakey } from "../media_key";
import { ua } from "../ua";

export type PublicConfig = {
  storage_public_url: string
  stream_public_url: string
  source_public_host: string
  source_public_port: number
}

export const public_config = (host: string, source_port_map: Config["source_port"]): PublicConfig => {
  let port: number;
  if(host === "studio.openstream.fm") {
    port = source_port_map.default;
  } else if (host === "studio.test.openstream.fm") {
    port = source_port_map.test;
  } else if (host === "studio.srv1.openstream.fm") {
    port = source_port_map.srv1;
  } else if (host === "studio.srv2.openstream.fm") {
    port = source_port_map.srv2;
  } else if (host === "studio.local.openstream.fm") {
    port = source_port_map.local;
  } else {
    port = source_port_map.default;
  }

  const config: PublicConfig = {
    storage_public_url: `https://${host.replace("studio.", "storage.")}`,
    stream_public_url: `https://${host.replace("studio.", "stream.")}`,
    source_public_host: `${host.replace("studio.", "source.")}`,
    source_public_port: port,
  }

  return config;
}

export const app_api = ({
  config,
  logger: _logger,
}: {
  config: Config,
  logger: Logger,
}) => {

  const client = new Client(config.openstream.api_base_url, { logger: _logger });

  const logger = _logger.scoped("app-api");

  let api = Router();

  // api.use((req, res, next) => {
  //   console.log("===============================");
  //   console.log({ url: req.originalUrl, headers: req.headers });
  //   next();
  // })

  api.use(json_body_parser())
  api.use(session(config, logger));

  api.get("/status", (req, res) => {
    res.json({ ok: true })
  })

  api.get("/config", json(async (req) => {
    return public_config(req.hostname || "studio.openstream.fm", config.source_port);
  }))

  api.post("/login", json(async (req, res) => {
    const sess = req.cookie_session;
    const r = await client.auth.user.login(ip(req), ua(req), { ...req.body, device_id: sess.device_id });
    const data = req.cookie_session;
    res.set_session({ ...data, user: { _id: r.user._id, token: r.token, media_key: r.media_key  } });
    return { user: r.user, media_key: r.media_key }
  }))

  api.post("/logout", json(async (req, res) => {
    const r = await client.auth.user.logout(ip(req), ua(req), token(req)).catch(() => {});
    const data = req.cookie_session;
    res.set_session({ ...data, user: null });
    return r;
  }))

  api.post("/register", json(async (req, res) => {
    const sess = req.cookie_session;
    const { account, user, token, media_key } = await client.auth.user.register(ip(req), ua(req), config.openstream.token, { ...req.body, device_id: sess.device_id });
    res.set_session({ ...sess, user: { _id: user._id, token, media_key }});
    return { user, account }
  }))

  api.route("/users/me")
    .get(json(async req => {
      const { user } = await client.users.get(ip(req), ua(req), token(req), user_id(req))
      return { user,  media_key: mediakey(req) };
    }))

  api.route("/users/:user")
    .get(json(async req => {
      return await client.users.get(ip(req), ua(req), token(req), req.params.user);
    }))
    .patch(json(async req => {
      return await client.users.patch(ip(req), ua(req), token(req), req.params.user, req.body);
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
      return await client.accounts.get(ip(req), ua(req), token(req), req.params.account)
    }))
    .patch(json(async req => {
      return await client.accounts.patch(ip(req), ua(req), token(req), req.params.account, req.body);
    }))

  api.route("/accounts/:account/stream-stats")
    .get(json(async req => {
      return await client.accounts.get_stream_stats(ip(req), ua(req), token(req), req.params.account);
    }))

  api.route("/accounts/:account/stream-stats/now")
    .get(json(async req => {
      return await client.accounts.get_stream_stats_item_now(ip(req), ua(req), token(req), req.params.account);
    }))

  api.route("/accounts/:account/stream-stats/now/count")
    .get(json(async req => {
      return await client.accounts.get_stream_stats_item_now_count(ip(req), ua(req), token(req), req.params.account);
    }))

  api.route("/accounts/:account/stream-stats/last-:num([0-9]+):unit(ms|s|min|h|d|w)")
    .get(json(async req => {
      return await client.accounts.get_stream_stats_item_since(ip(req), ua(req), token(req), req.params.account, req.params.num, req.params.unit);
    }))

  api.route("/accounts/:account/stream-stats/last-:num([0-9]+):unit(ms|s|min|h|d|w)/count")
    .get(json(async req => {
      return await client.accounts.get_stream_stats_item_since_count(ip(req), ua(req), token(req), req.params.account, req.params.num, req.params.unit);
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

  api.route("/stations/:station/stream-stats")
    .get(json(async req => {
      return await client.stations.get_stream_stats(ip(req), ua(req), token(req), req.params.station);
    }))

  api.route("/stations/:station/stream-stats/now")
    .get(json(async req => {
      return await client.stations.get_stream_stats_item_now(ip(req), ua(req), token(req), req.params.station);
    }))

  api.route("/stations/:station/stream-stats/now/count")
    .get(json(async req => {
      return await client.stations.get_stream_stats_item_now_count(ip(req), ua(req), token(req), req.params.station);
    }))

  api.route("/stations/:station/stream-stats/last-:num([0-9]+):unit(ms|s|min|h|d|w)")
    .get(json(async req => {
      return await client.stations.get_stream_stats_item_since(ip(req), ua(req), token(req), req.params.station, req.params.num, req.params.unit);
    }))

  api.route("/stations/:station/stream-stats/last-:num([0-9]+):unit(ms|s|min|h|d|w)/count")
    .get(json(async req => {
      return await client.stations.get_stream_stats_item_since_count(ip(req), ua(req), token(req), req.params.station, req.params.num, req.params.unit);
    }))

  api.route("/stations/:station/restart-playlist")
    .post(json(async req => {
      return await client.stations.restart_playlist(ip(req), ua(req), token(req), req.params.station);
    }))

  api.route("/stations/:station/reset-source-password")
    .post(json(async req => {
      return await client.stations.reset_source_password(ip(req), ua(req), token(req), req.params.station);
    }))

  api.route("/stations/:station/limits")
    .get(json(async req => {
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
      return await client.stations.files.patch_metadata(ip(req), ua(req), token(req), req.params.station, req.params.file, req.body);
    }))

  api.route("/stations/:station/now-playing")
    .get(json(async req => {
      return await client.stations.get_now_playing(ip(req), ua(req), token(req), req.params.station);
    }))

  // api.route("/stations/:station/dashboard-stats")
  //   .get(json(async req => {
  //     return await client.stations.get_dashboard_stats(ip(req), ua(req), token(req), req.params.station);
  //   }))

  // TODO: deprecate this endpoint (go directly to storage rs backend)
  api.route("/stations/:station/files/:file/stream")
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

  api.route("/devices")
    .get(json(async req => {
      return await client.devices.list(ip(req), ua(req), token(req), req.query as any);
    }))

  api.route("/devices/:device")
    .delete(json(async req => {
      return await client.devices.delete(ip(req), ua(req), token(req), req.params.device);
    }))

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