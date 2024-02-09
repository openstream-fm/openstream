import type { Config, HostConfig } from "../config.js";
import { Router, json as json_body_parser } from "express";
import { ApiError, json_catch_handler } from "../error.js";
import type { Logger } from "../logger.js";
import { handler, json } from "../handler.js";
import type { Client } from "../client.server.js";
import { session } from "../session.js";
import { ip } from "../ip.js";
import { user_token } from "../token.js";
import { user_id } from "../user-id.js";
import { StatusCodes } from "http-status-codes";
import { user_media_key } from "../media_key.js";
import { ua } from "../ua.js";
import { shared_api } from "./shared-api.js";
import { host } from "../host.js";
import { default_studio_locale, studio_locales_map, studio_locales } from "../locale/studio/studio.locale.js";
import type { StudioLocale } from "../locale/studio/studio.locale.js";
import acceptLanguageParser from "accept-language-parser";
import { LOCALE_DIR_HEADER, LOCALE_LANG_HEADER } from "../constants.js";

export type PublicConfig = {
  storage_public_url: string
  stream_public_url: string
  studio_public_url: string
  source_public_host: string
  source_public_port: number
}

export const public_config = (hosts: HostConfig & { id: string }): PublicConfig => {
  const config: PublicConfig = {
    storage_public_url: `//${hosts.storage.host}`,
    stream_public_url: `//${hosts.stream.host}`,
    studio_public_url: `//${hosts.studio.host}`,
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
  client,
  logger: _logger,
}: {
  config: Config,
  client: Client,
  logger: Logger,
}) => {

  const logger = _logger.scoped("studio-api");

  let api = Router();

  api.use(json_body_parser())
  api.use(session("studio", config, logger));

  api.route("/status")
    .get(json(async () => {
      return { ok: true }
    }));

  api.route("/config")
    .get(json(async req => {
      const hosts = host("studio", config.hosts, req);
      return public_config(hosts);
    }))

  api.route("/locale")
    .get(handler(async (req, res) => {
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

      let locale: StudioLocale | null = null;

      locale: if(langs != null) {
        for(const lang of langs) {
          for(const item of studio_locales) {
            if(lang.code.toLowerCase() == item.lang.toLowerCase() && lang.region?.toLowerCase() == item.region?.toLowerCase()) {
              locale = item;
              break locale;
            }
          }
        }

        for(const lang of langs) {
          for(const item of studio_locales) {
            if(item.lang.toLowerCase() === lang.code.toLowerCase()) {
              locale = item;
              break locale;
            }
          }
        }
      }

      if(locale == null) locale = default_studio_locale;

      const dir = locale.lang === "ar" ? "rtl" : "ltr";
      const lang = locale.region ? `${locale.lang}-${locale.region}` : locale.lang;
      
      res.header(LOCALE_LANG_HEADER, lang);
      res.header(LOCALE_DIR_HEADER, dir);
      res.vary("accept-language");
      res.redirect(302, `/api/locale/${lang}.json`);
    }));

  api.route("/locale/:code.json")
    .get(json(async (req, res): Promise<LocalePayload> => {
      const code = req.params.code;
      const locale = studio_locales_map.get(code);
      if(locale == null) {
        throw new ApiError(StatusCodes.NOT_FOUND, "FRONT_RESOURCE_NOT_FOUND", `Locale with code ${code} not found`);
      }
      const dir = locale.lang === "ar" ? "rtl" : "ltr";
      const lang = locale.region ? `${locale.lang}-${locale.region}` : locale.lang;
      res.header(LOCALE_LANG_HEADER, lang);
      res.header(LOCALE_DIR_HEADER, dir);
      return { locale }
    }))

  api.route("/auth/user/email-exists/:email")
    .get(json(async (req, res) => {
      return await client.auth.user.email_exists(ip(req), ua(req), null, req.params.email);
    }))

  api.route("/auth/user/login")
    .post(json(async (req, res) => {
      const sess = req.cookie_session;
      const r = await client.auth.user.login(ip(req), ua(req), null, { ...req.body, device_id: sess.device_id });
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

  api.route("/auth/user/logout")
    .post(json(async (req, res) => {
      const r = await client.auth.user.logout(ip(req), ua(req), user_token(req)).catch(() => {});
      const data = req.cookie_session;
      res.set_session({ ...data, user: null });
      return r;
    }))

  api.route("/auth/user/register")
    .post(json(async (req, res) => {
      const sess = req.cookie_session;
      const { account, user, token, media_key } = await client.auth.user.register(ip(req), ua(req), null, { ...req.body, device_id: sess.device_id });
      res.set_session({ ...sess, user: { _id: user._id, token, media_key }});
      return { user, account }
    }))

  api.route("/auth/user/recover")
    .post(json(async (req, res) => {
      return await client.auth.user.recover(ip(req), ua(req), null, req.body);
    }))

  api.route("/auth/user/recovery-token/:token") 
    .get(json(async req => {
      return await client.auth.user.recovery_token.get(ip(req), ua(req), null, req.params.token);
    }))

  api.route("/auth/user/recovery-token/:token/set-password")
    .post(json(async req => {
      return await client.auth.user.recovery_token.set_password(ip(req), ua(req), null, req.params.token, req.body);
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