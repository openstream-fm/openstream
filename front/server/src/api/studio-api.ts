import type { Config, HostConfig } from "../config";
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
import { host } from "../host";
import { default_studio_locale, studio_locales } from "../locale/studio/studio.locale";
import type { StudioLocale } from "../locale/studio/studio.locale";
import acceptLanguageParser from "accept-language-parser";

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

export type LocalePayload = {
  locale: StudioLocale
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

  api.get("/config", json(async req => {
    const hosts = host("studio", config.hosts, req);
    return public_config(hosts);
  }))

  api.get("/locale", json(async (req): Promise<LocalePayload> => {
    let langs: ReturnType<typeof acceptLanguageParser.parse> | null = null;
    if(req.cookie_session.user) {
      try {
        const { user } = await client.users.get(ip(req), ua(req), user_token(req), req.cookie_session.user._id);
        if(user.language != null) {
          langs = acceptLanguageParser.parse(user.language);
        }
      } catch(e) { }
    }

    if(langs == null) {
      const header = req.header("accept-language");
      if(header != null) {
        try {
          langs = acceptLanguageParser.parse(header);
        } catch(e) { }
      }
    }

    if(langs != null) {
      for(const lang of langs) {
        for(const locale of studio_locales) {
          if(lang.code.toLowerCase() == locale.lang.toLowerCase() && lang.region?.toLowerCase() == locale.region?.toLowerCase()) {
            return { locale };
          }
        }
      }

      for(const lang of langs) {
        for(const locale of studio_locales) {
          if(locale.lang.toLowerCase() === lang.code.toLowerCase()) {
            return { locale };
          }
        }
      }
    }

    return { locale: default_studio_locale };

  }))

  api.post("/auth/user/login", json(async (req, res) => {
    const sess = req.cookie_session;
    const r = await client.auth.user.login(ip(req), ua(req), { ...req.body, device_id: sess.device_id });
    const data = req.cookie_session;
    res.set_session({
      ...data,
      user: {
        _id: r.user._id,
        token: r.token,
        media_key: r.media_key
      }
    });
    return { user: r.user, media_key: r.media_key }
  }))

  api.post("/auth/user/logout", json(async (req, res) => {
    const r = await client.auth.user.logout(ip(req), ua(req), user_token(req)).catch(() => {});
    const data = req.cookie_session;
    res.set_session({ ...data, user: null });
    return r;
  }))

  api.post("/auth/user/register", json(async (req, res) => {
    const sess = req.cookie_session;
    const { account, user, token, media_key } = await client.auth.user.register(ip(req), ua(req), null, { ...req.body, device_id: sess.device_id });
    res.set_session({ ...sess, user: { _id: user._id, token, media_key }});
    return { user, account }
  }))

  api.post("/auth/user/recover", json(async (req, res) => {
    return await client.auth.user.recover(ip(req), ua(req), req.body);
  }))

  api.get("/auth/user/recovery-token/:token", json(async req => {
    return await client.auth.user.recovery_token.get(ip(req), ua(req), req.params.token);
  }))

  api.post("/auth/user/recovery-token/:token/set-password", json(async req => {
    return await client.auth.user.recovery_token.set_password(ip(req), ua(req), req.params.token, req.body);
  }))

  api.route("/plans")
    .get(json(async req => {
      return await client.plans.list(ip(req), ua(req), null, { show: "active" });
    }))

  api.route("/plans/:plan")
    .get(json(async req => {
      return await client.plans.get(ip(req), ua(req), null, req.params.plan);
    }))

  api.route("/plans/by-slug/:slug")
    .get(json(async req => {
      return await client.plans.get_by_slug(ip(req), ua(req), null, req.params.slug);
    }))

  api.route("/users/me")
    .get(json(async req => {
      const { user } = await client.users.get(ip(req), ua(req), user_token(req), user_id(req))
      return { user, media_key: user_media_key(req) };
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