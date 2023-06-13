import { type Request, Router } from "express";
import type { Logger } from "../logger";
import { json } from "../handler";
import type { Client } from "../client";
import { ua } from "../ua";
import { ip } from "../ip";
import { ACCESS_TOKEN_HEADER, FORWARD_IP_HEADER } from "../constants";
import { pipeline } from "stream/promises";
import { BadRequest } from "../error";

export const shared_api = ({
  logger,
  client,
  get_token,
}: {
  logger: Logger,
  client: Client,
  get_token: (req: Request) => string
}) => {
  const api = Router();

  api.route("/me/devices")
    .get(json(async req => {
      return await client.me.devices.list(ip(req), ua(req), get_token(req), req.query as any);
    }))

  api.route("/me/devices/:device")
    .delete(json(async req => {
      return await client.me.devices.delete(ip(req), ua(req), get_token(req), req.params.device);
    }))


  api.route("/auth/email-verification/send-code")
    .post(json(async req => {
      return await client.auth.send_email_verification_code(ip(req), ua(req), req.body);
    }))

  api.route("/users")
    .get(json(async req => {
      return await client.users.list(ip(req), ua(req), get_token(req), req.query);
    }))
    
  api.route("/users/:user")
    .get(json(async req => {
      return await client.users.get(ip(req), ua(req), get_token(req), req.params.user);
    }))
    .patch(json(async req => {
      return await client.users.patch(ip(req), ua(req), get_token(req), req.params.user, req.body);
    }))
  

  api.route("/accounts")
    .get(json(async req => {
      return await client.accounts.list(ip(req), ua(req), get_token(req), req.query);
    }))
    .post(json(async req => {
      return await client.accounts.post(ip(req), ua(req), get_token(req), req.body);
    }))

  api.route("/accounts/:account")
    .get(json(async req => {
      return await client.accounts.get(ip(req), ua(req), get_token(req), req.params.account)
    }))
    .patch(json(async req => {
      return await client.accounts.patch(ip(req), ua(req), get_token(req), req.params.account, req.body)
    }))

  api.route("/accounts/:account/limits")
    .get(json(async req => {
      const { account: { limits } } = await client.accounts.get(ip(req), ua(req), get_token(req), req.params.account);
      return limits;
    }))

  api.route("/accounts/:account/members")
    .get(json(async req => {
      return await client.accounts.list_members(ip(req), ua(req), get_token(req), req.params.account)
    }))

  api.route("/accounts/:account/members/:member")
    .delete(json(async req => {
      return await client.accounts.delete_member(ip(req), ua(req), get_token(req), req.params.account, req.params.member)
    }))

  api.route("/accounts/:account/members/:member/set-role")
    .post(json(async req => {
      return await client.accounts.set_member_role(ip(req), ua(req), get_token(req), req.params.account, req.params.member, req.body);
    }))

  api.route("/accounts/:account/stream-stats")
    .get(json(async req => {
      return await client.accounts.get_stream_stats(ip(req), ua(req), get_token(req), req.params.account);
    }))

  api.route("/accounts/:account/stream-stats/now")
    .get(json(async req => {
      return await client.accounts.get_stream_stats_item_now(ip(req), ua(req), get_token(req), req.params.account);
    }))

  api.route("/accounts/:account/stream-stats/now/count")
    .get(json(async req => {
      return await client.accounts.get_stream_stats_item_now_count(ip(req), ua(req), get_token(req), req.params.account);
    }))

  api.route("/accounts/:account/stream-stats/last-:num([0-9]+):unit(ms|s|min|h|d|w)")
    .get(json(async req => {
      return await client.accounts.get_stream_stats_item_since(ip(req), ua(req), get_token(req), req.params.account, req.params.num, req.params.unit);
    }))

  api.route("/accounts/:account/stream-stats/last-:num([0-9]+):unit(ms|s|min|h|d|w)/count")
    .get(json(async req => {
      return await client.accounts.get_stream_stats_item_since_count(ip(req), ua(req), get_token(req), req.params.account, req.params.num, req.params.unit);
    }))


  api.route("/stations")
    .get(json(async req => {
      return await client.stations.list(ip(req), ua(req), get_token(req), req.query);
    }))
    .post(json(async req => {
      return await client.stations.post(ip(req), ua(req), get_token(req), req.body);
    }))

  api.route("/stations/:station")
    .get(json(async req => {
      return await client.stations.get(ip(req), ua(req), get_token(req), req.params.station);
    }))
    .delete(json(async req => {
      return await client.stations.delete(ip(req), ua(req), get_token(req), req.params.station);
    }))
    .patch(json(async req => {
      return await client.stations.patch(ip(req), ua(req), get_token(req), req.params.station, req.body);
    }))

  api.route("/stations/:station/stream-stats")
    .get(json(async req => {
      return await client.stations.get_stream_stats(ip(req), ua(req), get_token(req), req.params.station);
    }))

  api.route("/stations/:station/stream-stats/now")
    .get(json(async req => {
      return await client.stations.get_stream_stats_item_now(ip(req), ua(req), get_token(req), req.params.station);
    }))

  api.route("/stations/:station/stream-stats/now/count")
    .get(json(async req => {
      return await client.stations.get_stream_stats_item_now_count(ip(req), ua(req), get_token(req), req.params.station);
    }))

  api.route("/stations/:station/stream-stats/last-:num([0-9]+):unit(ms|s|min|h|d|w)")
    .get(json(async req => {
      return await client.stations.get_stream_stats_item_since(ip(req), ua(req), get_token(req), req.params.station, req.params.num, req.params.unit);
    }))

  api.route("/stations/:station/stream-stats/last-:num([0-9]+):unit(ms|s|min|h|d|w)/count")
    .get(json(async req => {
      return await client.stations.get_stream_stats_item_since_count(ip(req), ua(req), get_token(req), req.params.station, req.params.num, req.params.unit);
    }))

  api.route("/stations/:station/restart-playlist")
    .post(json(async req => {
      return await client.stations.restart_playlist(ip(req), ua(req), get_token(req), req.params.station);
    }))

  api.route("/stations/:station/reset-source-password")
    .post(json(async req => {
      return await client.stations.reset_source_password(ip(req), ua(req), get_token(req), req.params.station);
    }))

  api.route("/stations/:station/files")
    .get(json(async req => {
      return await client.stations.files.list(ip(req), ua(req), get_token(req), req.params.station, req.query)
    }))

    .post(json(async req => {
      const content_type = req.header("content-type") ?? "application/octet-stream";
      const content_length = Number(req.header("content-length"));
      if (!content_length) {
        throw new BadRequest("Content length must be specified (front)", "CONTENT_LENGTH_REQUIRED");
      }
      return await client.stations.files.post(ip(req), ua(req), get_token(req), req.params.station, content_type, content_length, req.query as any, req);
    }))

  api.route("/stations/:station/files/shuffle")
    .post(json(async req => {
      return await client.stations.files.shuffle(ip(req), ua(req), get_token(req), req.params.station);
    }));

  api.route("/stations/:station/files/unshuffle")
    .post(json(async req => {
      return await client.stations.files.unshuffle(ip(req), ua(req), get_token(req), req.params.station);
    }));

  api.route("/stations/:station/files/:file")
    .get(json(async req => {
      return await client.stations.files.get(ip(req), ua(req), get_token(req), req.params.station, req.params.file);
    }))
    .delete(json(async req => {
      return await client.stations.files.delete(ip(req), ua(req), get_token(req), req.params.station, req.params.file);
    }))

  api.route("/stations/:station/files/:file/order/swap")
    .post(json(async req => {
      return await client.stations.files.swap_order(ip(req), ua(req), get_token(req), req.params.station, req.params.file, req.body);
    }))

  api.route("/stations/:station/files/:file/order/move-before")
    .post(json(async req => {
      return await client.stations.files.move_before(ip(req), ua(req), get_token(req), req.params.station, req.params.file, req.body);
    }))

  api.route("/stations/:station/files/:file/order/move-after")
    .post(json(async req => {
      return await client.stations.files.move_after(ip(req), ua(req), get_token(req), req.params.station, req.params.file, req.body);
    }))

  api.route("/stations/:station/files/:file/order/move-to-first")
    .post(json(async req => {
      return await client.stations.files.move_to_first(ip(req), ua(req), get_token(req), req.params.station, req.params.file);
    }))

  api.route("/stations/:station/files/:file/order/move-to-last")
    .post(json(async req => {
      return await client.stations.files.move_to_last(ip(req), ua(req), get_token(req), req.params.station, req.params.file);
    }))



  api.route("/stations/:station/files/:file/metadata")
    .put(json(async req => {
      return await client.stations.files.patch_metadata(ip(req), ua(req), get_token(req), req.params.station, req.params.file, req.body);
    }))

  api.route("/stations/:station/now-playing")
    .get(json(async req => {
      return await client.stations.get_now_playing(ip(req), ua(req), get_token(req), req.params.station);
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
        for (const key of ["user-agent", "if-none-match", "accept", "accept-language", "range"]) {
          const value = req.header(key);
          if (value) headers[key] = value;
        }

        headers[FORWARD_IP_HEADER] = ip(req);
        headers[ACCESS_TOKEN_HEADER] = get_token(req);

        const back = await client.fetch(`/stations/${station}/files/${file}/stream`, {
          method: "GET",
          headers,
        })

        res.status(back.status);

        for (const key of ["etag", "content-type", "content-length", "content-language", "accept-ranges", "content-range"]) {
          const value = back.headers.get(key);
          if (value != null) {
            res.header(key, value);
          }
        }

        res.header("vary", "range");

        if (back.body) {
          await pipeline(back.body, res);
        } else {
          res.end();
        }
      } catch (e) {
        next(e)
      }
    })

  api.route("/station-pictures")
    .post(json(async req => {
      return await client.stations.pictures.post(ip(req), ua(req), get_token(req), req.query as any, req)
    }));
  
  api.route("/analytics").get(json(async req => {
    return await client.analytics.get(ip(req), ua(req), get_token(req), req.query as any);
  }))

  api.route("/invitations")
    .get(json(async req => {
      return await client.invitations.list(ip(req), ua(req), get_token(req), req.query as any)
    }))
    .post(json(async req => {
      return await client.invitations.post(ip(req), ua(req), get_token(req), req.body)
    }))

  api.route("/invitations/:invitation")
    .get(json(async req => {
      return await client.invitations.get(ip(req), ua(req), get_token(req), req.params.invitation)
    }))
    .delete(json(async req => {
      return await client.invitations.delete(ip(req), ua(req), get_token(req), req.params.invitation)
    }))

  api.route("/invitations/accept")
    .post(json(async req => {
      return await client.invitations.accept(ip(req), ua(req), get_token(req), req.body)
    }))

  api.route("/invitations/reject")
    .post(json(async req => {
      return await client.invitations.reject(ip(req), ua(req), get_token(req), req.body)
    }))

  api.route("/invitations/get-by-token/:token")
    .get(json(async req => {
      return await client.invitations.get_by_token(ip(req), ua(req), null, req.params.token)
    }))

    

  api.route("/payment-methods")
    .get(json(async req => {
      return await client.payment_methods.list(ip(req), ua(req), get_token(req), req.query as any)
    }))
    .post(json(async req => {
      return await client.payment_methods.post(ip(req), ua(req), get_token(req), req.body)
    }))

  api.route("/payment-methods/:payment_method")
    .get(json(async req => {
      return await client.payment_methods.get(ip(req), ua(req), get_token(req), req.params.payment_method);
    }))

  return api;
}