import type { Config } from "../config";
import { Router, json as json_body_parser } from "express";
import { ApiError, json_catch_handler } from "../error";
import type { Logger } from "../logger";
import { json } from "../handler";
import { Client } from "../client";
import { session } from "../session";
import { ip } from "../ip";
import { admin_token } from "../token";
import { StatusCodes } from "http-status-codes";
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
  if(host === "admin.openstream.fm") {
    port = source_port_map.default;
  } else if (host === "admin.test.openstream.fm") {
    port = source_port_map.test;
  } else if (host === "admin.s1.openstream.fm") {
    port = source_port_map.s1;
  } else if (host === "admin.s2.openstream.fm") {
    port = source_port_map.s2;
  } else if (host === "admin.local.openstream.fm") {
    port = source_port_map.local;
  } else {
    port = source_port_map.default;
  }

  const config: PublicConfig = {
    storage_public_url: `https://${host.replace("admin.", "storage.")}`,
    stream_public_url: `https://${host.replace("admin.", "stream.")}`,
    source_public_host: `${host.replace("studio.", "source.")}`,
    source_public_port: port,
  }

  return config;
}

export const admin_api = ({
  config,
  logger: _logger,
}: {
  config: Config,
  logger: Logger,
}) => {

  const client = new Client(config.openstream.api_base_url, { logger: _logger });

  const logger = _logger.scoped("admin-api");

  let api = Router();

  api.use(json_body_parser())
  api.use(session(config, logger));

  api.get("/status", (req, res) => {
    res.json({ ok: true })
  })

  api.get("/config", json(async (req) => {
    return public_config(req.hostname || "admin.openstream.fm", config.source_port);
  }))

  api.post("/login", json(async (req, res) => {
    const sess = req.cookie_session;
    const r = await client.auth.admin.login(ip(req), ua(req), { ...req.body, device_id: sess.device_id });
    const data = req.cookie_session;
    res.set_session({ ...data, admin: { _id: r.admin._id, token: r.token, media_key: r.media_key  } });
    return { admin: r.admin, media_key: r.media_key }
  }))

  api.post("/logout", json(async (req, res) => {
    const r = await client.auth.admin.logout(ip(req), ua(req), admin_token(req)).catch(() => {});
    const data = req.cookie_session;
    res.set_session({ ...data, admin: null });
    return r;
  }))

  api.post("/auth/delegate/:user", json(async (req, res) => {
    const { user, media_key, token } = await client.auth.admin.delegate(ip(req), ua(req), admin_token(req), req.params.user, req.body);
    const data = req.cookie_session;
    res.set_session({ ...data, user: { _id: user._id, token, media_key }});
    return { user, media_key };
  }))

  api.use(shared_api({
    client,
    get_token: admin_token,
    logger,
  }))

  api.use(json(() => {
    throw new ApiError(StatusCodes.NOT_FOUND, "FRONT_RESOURCE_NOT_FOUND", "Resource not found");
  }))

  api.use(json_catch_handler(logger));

  return api;

}