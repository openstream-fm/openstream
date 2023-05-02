import type { Config } from "../config";
import { Router, json as json_body_parser } from "express";
import { ApiError, json_catch_handler } from "../error";
import type { Logger } from "../logger";
import { json } from "../handler";
import { Client } from "../client";
import { session } from "../session";
import { ip } from "../ip";
import { user_token } from "../token";
import { user_id } from "../user-id";
import { StatusCodes } from "http-status-codes";
import { user_media_key } from "../media_key";
import { ua } from "../ua";
import { shared_api } from "./shared-api";

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
  } else if (host === "studio.s1.openstream.fm") {
    port = source_port_map.s1;
  } else if (host === "studio.s2.openstream.fm") {
    port = source_port_map.s2;
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

export const studio_api = ({
  config,
  logger: _logger,
}: {
  config: Config,
  logger: Logger,
}) => {

  const client = new Client(config.openstream.api_base_url, { logger: _logger });

  const logger = _logger.scoped("studio-api");

  let api = Router();

  api.use(json_body_parser())
  api.use(session("studio", config, logger));

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
    const r = await client.auth.user.logout(ip(req), ua(req), user_token(req)).catch(() => {});
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
      const { user } = await client.users.get(ip(req), ua(req), user_token(req), user_id(req))
      return { user,  media_key: user_media_key(req) };
    }))

  api.use(shared_api({
    client,
    get_token: user_token,
    logger,
  }))

  api.use(json(() => {
    throw new ApiError(StatusCodes.NOT_FOUND, "FRONT_RESOURCE_NOT_FOUND", "Resource not found");
  }))

  api.use(json_catch_handler(logger));

  return api;

}