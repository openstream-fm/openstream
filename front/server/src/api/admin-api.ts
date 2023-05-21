import type { Config, HostConfig } from "../config";
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
import { admin_media_key } from "../media_key";
import { admin_id } from "../admin-id";
import { host } from "../host";

export type PublicConfig = {
  storage_public_url: string
  stream_public_url: string
  source_public_host: string
  source_public_port: number
}

export const public_config = (hosts: HostConfig & { id: string }): PublicConfig => {
  const config: PublicConfig = {
    storage_public_url: `https://${hosts.storage.host}`,
    stream_public_url: `https://${hosts.stream.host}`,
    source_public_host: hosts.source.host,
    source_public_port: hosts.source.port,
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
  api.use(session("admin", config, logger));

  api.get("/status", (req, res) => {
    res.json({ ok: true })
  })

  api.get("/config", json(async (req) => {
    const hosts = host("admin", config.hosts, req);
    return public_config(hosts);
  }))

  api.post("/auth/admin/login", json(async (req, res) => {
    const sess = req.cookie_session;
    const r = await client.auth.admin.login(ip(req), ua(req), { ...req.body, device_id: sess.device_id });
    const data = req.cookie_session;
    res.set_session({ ...data, admin: { _id: r.admin._id, token: r.token, media_key: r.media_key  } });
    return { admin: r.admin, media_key: r.media_key }
  }))

  api.post("/auth/admin/logout", json(async (req, res) => {
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

  api.route("/plans")
  .get(json(async req => {
    return await client.plans.list(ip(req), ua(req), admin_token(req), req.query as any);
  }))
  .post(json(async req => {
    return await client.plans.post(ip(req), ua(req), admin_token(req), req.body)
  }))

  api.route("/plans/:plan")
    .get(json(async req => {
      return await client.plans.get(ip(req), ua(req), admin_token(req), req.params.plan);
    }))
    .patch(json(async req => {
      return await client.plans.patch(ip(req), ua(req), admin_token(req), req.params.plan, req.body)
    }))
    .delete(json(async req => {
      return await client.plans.delete(ip(req), ua(req), admin_token(req), req.params.plan)
    }))

  
  api.route("/admins")
    .get(json(async req => {
      return await client.admins.list(ip(req), ua(req), admin_token(req), req.query);
    }))
    .post(json(async req => {
      return await client.admins.post(ip(req), ua(req), admin_token(req), req.body);
    }))

  api.route("/admins/me")
    .get(json(async req => {
      const { admin } = await client.admins.get(ip(req), ua(req), admin_token(req), admin_id(req))
      return { admin, media_key: admin_media_key(req) };
    }))

  api.route("/admins/:admin")
    .get(json(async req => {
      return await client.admins.get(ip(req), ua(req), admin_token(req), req.params.admin);
    }))
    .patch(json(async req => {
      return await client.admins.patch(ip(req), ua(req), admin_token(req), req.params.admin, req.body);
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