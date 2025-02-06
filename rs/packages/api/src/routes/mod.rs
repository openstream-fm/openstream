pub mod me;

pub mod accounts;
pub mod admins;
pub mod auth;
pub mod stations;
pub mod users;

pub mod analytics;
pub mod app_analytics;
pub mod embed;
pub mod invitations;
pub mod payment_methods;
pub mod plans;
pub mod runtime;
pub mod station_pictures;
pub mod stream_connections;
pub mod stream_connections_lite;
pub mod stream_stats;

use db::station_picture::StationPicture;
use db::stream_connection::index::MemIndex;
use drop_tracer::DropTracer;
use hyper::header::{HeaderValue, CONTENT_TYPE};
use hyper::{Body, StatusCode};
use mailer::send::Mailer;
use media::MediaSessionMap;
use prex::router::builder::Builder;
use prex::{Request, Response};
use shutdown::Shutdown;

use crate::error::ApiError;
use crate::json::JsonHandler;

use payments::client::PaymentsClient;

use async_trait::async_trait;

pub fn router(
  deployment_id: String,
  media_sessions: MediaSessionMap,
  shutdown: Shutdown,
  drop_tracer: DropTracer,
  stream_connections_index: MemIndex,
  payments_client: PaymentsClient,
  mailer: Mailer,
) -> Builder {
  let mut app = prex::prex();

  app.at("/openapi.json").get(|_, _| async move {
    let html = include_str!("../../../../../openapi.json");
    let mut res = Response::new(StatusCode::OK);
    res.headers_mut().insert(
      CONTENT_TYPE,
      HeaderValue::from_static("application/json;charset=utf-8"),
    );
    *res.body_mut() = Body::from(html);
    res
  });

  app.at("/docs").get(|_, _| async move {
    let html = include_str!("../../../../../openapi/redoc.html");
    let mut res = Response::new(StatusCode::OK);
    res.headers_mut().insert(
      CONTENT_TYPE,
      HeaderValue::from_static("text/html;charset=utf-8"),
    );
    *res.body_mut() = Body::from(html);
    res
  });

  app.at("/me").get(me::get::Endpoint {}.into_handler());

  app
    .at("/me/devices")
    .get(me::devices::get::Endpoint {}.into_handler());

  app
    .at("/me/devices/:device")
    .delete(me::devices::id::delete::Endpoint {}.into_handler());

  app
    .at("/me/api-keys")
    .get(me::api_keys::get::Endpoint {}.into_handler())
    .post(me::api_keys::post::Endpoint {}.into_handler());

  app
    .at("/me/api-keys/:id")
    .patch(me::api_keys::id::patch::Endpoint {}.into_handler())
    .delete(me::api_keys::id::delete::Endpoint {}.into_handler());

  app.at("/auth/email-verification/send-code").post(
    auth::email_verification::send_code::post::Endpoint {
      mailer: mailer.clone(),
    }
    .into_handler(),
  );

  app
    .at("/auth/user/login")
    .post(auth::user::login::post::Endpoint {}.into_handler());

  app
    .at("/auth/user/logout")
    .post(auth::user::logout::post::Endpoint {}.into_handler());

  app
    .at("/auth/user/email-exists/:email")
    .get(auth::user::email_exists::get::Endpoint {}.into_handler());

  app.at("/auth/user/register").post(
    auth::user::register::post::Endpoint {
      payments_client: payments_client.clone(),
    }
    .into_handler(),
  );

  app.at("/auth/user/recover").post(
    auth::user::recover::post::Endpoint {
      mailer: mailer.clone(),
    }
    .into_handler(),
  );

  app
    .at("/auth/user/recovery-token/:token")
    .get(auth::user::recovery_token::token::get::Endpoint {}.into_handler());

  app
    .at("/auth/user/recovery-token/:token/set-password")
    .post(auth::user::recovery_token::token::set_password::post::Endpoint {}.into_handler());

  app
    .at("/auth/admin/login")
    .post(auth::admin::login::post::Endpoint {}.into_handler());

  app
    .at("/auth/admin/logout")
    .post(auth::admin::logout::post::Endpoint {}.into_handler());

  app
    .at("/auth/admin/delegate/:user")
    .post(auth::admin::delegate::user::post::Endpoint {}.into_handler());

  app.at("/runtime/station-deleted/:station").post(
    runtime::station_deleted::station_id::post::Endpoint {
      media_sessions: media_sessions.clone(),
    }
    .into_handler(),
  );

  app.at("/runtime/source-password-updated/:station").post(
    runtime::source_password_updated::station_id::post::Endpoint {
      media_sessions: media_sessions.clone(),
    }
    .into_handler(),
  );

  app.at("/runtime/external-relay-updated/:station").post(
    runtime::external_relay_updated::station_id::post::Endpoint {
      media_sessions: media_sessions.clone(),
    }
    .into_handler(),
  );

  app.at("/runtime/restart-playlist/:station").post(
    runtime::restart_playlist::station_id::post::Endpoint {
      deployment_id: deployment_id.clone(),
      media_sessions: media_sessions.clone(),
      drop_tracer: drop_tracer.clone(),
      shutdown: shutdown.clone(),
    }
    .into_handler(),
  );

  app
    .at("/analytics")
    .get(analytics::get::Endpoint {}.into_handler());

  app
    .at("/app-analytics")
    .get(app_analytics::get::Endpoint {}.into_handler());

  app.at("/stream-stats").get(
    stream_stats::get::Endpoint {
      index: stream_connections_index.clone(),
    }
    .into_handler(),
  );

  app.at("/stream-stats/now").get(
    stream_stats::now::get::Endpoint {
      index: stream_connections_index.clone(),
    }
    .into_handler(),
  );

  app.at("/stream-stats/now/count").get(
    stream_stats::now::count::get::Endpoint {
      index: stream_connections_index.clone(),
    }
    .into_handler(),
  );

  app.at("/stream-stats/now/count-by-station").get(
    stream_stats::now::count_by_station::get::Endpoint {
      index: stream_connections_index.clone(),
    }
    .into_handler(),
  );

  app
    .at("/stream-stats/last-:num([0-9]+):unit(ms|s|min|h|d|w)")
    .get(
      stream_stats::since::get::Endpoint {
        index: stream_connections_index.clone(),
      }
      .into_handler(),
    );

  app
    .at("/stream-stats/last-:num([0-9]+):unit(ms|s|min|h|d|w)/count")
    .get(
      stream_stats::since::count::get::Endpoint {
        index: stream_connections_index.clone(),
      }
      .into_handler(),
    );

  app
    .at("/plans")
    .get(plans::get::Endpoint {}.into_handler())
    .post(plans::post::Endpoint {}.into_handler());

  app
    .at("/plans/by-slug/:slug")
    .get(plans::by_slug::slug::get::Endpoint {}.into_handler());

  app
    .at("/plans/:plan")
    .get(plans::id::get::Endpoint {}.into_handler())
    .patch(plans::id::patch::Endpoint {}.into_handler())
    .delete(plans::id::delete::Endpoint {}.into_handler());

  app
    .at("/users")
    .get(users::get::Endpoint {}.into_handler())
    .post(users::post::Endpoint {}.into_handler());

  app
    .at("/users/:user")
    .get(users::id::get::Endpoint {}.into_handler())
    .patch(users::id::patch::Endpoint {}.into_handler())
    .delete(users::id::delete::Endpoint {}.into_handler());

  app
    .at("/users/:user/change-password")
    .post(users::change_password::post::Endpoint {}.into_handler());

  app
    .at("/accounts")
    .get(accounts::get::Endpoint {}.into_handler())
    .post(accounts::post::Endpoint {}.into_handler());

  app
    .at("/accounts/:account")
    .get(accounts::id::get::Endpoint {}.into_handler())
    .patch(accounts::id::patch::Endpoint {}.into_handler())
    .delete(accounts::id::delete::Endpoint {}.into_handler());

  app
    .at("/accounts/:account/members")
    .get(accounts::members::get::Endpoint {}.into_handler());

  app
    .at("/accounts/:account/members/:member")
    .delete(accounts::members::id::delete::Endpoint {}.into_handler());

  app
    .at("/accounts/:account/members/:member/set-role")
    .post(accounts::members::id::set_role::post::Endpoint {}.into_handler());

  app.at("/accounts/:account/stream-stats").get(
    accounts::stream_stats::get::Endpoint {
      index: stream_connections_index.clone(),
    }
    .into_handler(),
  );

  app.at("/accounts/:account/stream-stats/now").get(
    accounts::stream_stats::now::get::Endpoint {
      index: stream_connections_index.clone(),
    }
    .into_handler(),
  );

  app.at("/accounts/:account/stream-stats/now/count").get(
    accounts::stream_stats::now::count::get::Endpoint {
      index: stream_connections_index.clone(),
    }
    .into_handler(),
  );

  app
    .at("/accounts/:account/stream-stats/now/count-by-station")
    .get(
      accounts::stream_stats::now::count_by_station::get::Endpoint {
        index: stream_connections_index.clone(),
      }
      .into_handler(),
    );

  app
    .at("/accounts/:account/stream-stats/last-:num([0-9]+):unit(ms|s|min|h|d|w)")
    .get(
      accounts::stream_stats::since::get::Endpoint {
        index: stream_connections_index.clone(),
      }
      .into_handler(),
    );

  app
    .at("/accounts/:account/stream-stats/last-:num([0-9]+):unit(ms|s|min|h|d|w)/count")
    .get(
      accounts::stream_stats::since::count::get::Endpoint {
        index: stream_connections_index.clone(),
      }
      .into_handler(),
    );

  app
    .at("/stations")
    .get(stations::get::Endpoint {}.into_handler())
    .post(stations::post::Endpoint {}.into_handler());

  app
    .at("/stations/is-slug-available")
    .get(stations::is_slug_available::get::Endpoint {}.into_handler());

  app
    .at("/stations/:station")
    .get(stations::id::get::Endpoint {}.into_handler())
    .delete(
      stations::id::delete::Endpoint {
        deployment_id: deployment_id.clone(),
        media_sessions: media_sessions.clone(),
      }
      .into_handler(),
    )
    .patch(
      stations::id::patch::Endpoint {
        deployment_id: deployment_id.clone(),
        media_sessions: media_sessions.clone(),
      }
      .into_handler(),
    );

  app
    .at("/stations/:station/transfer")
    .post(stations::transfer::post::Endpoint {}.into_handler());

  app.at("/stations/:station/stream-stats").get(
    stations::stream_stats::get::Endpoint {
      index: stream_connections_index.clone(),
    }
    .into_handler(),
  );

  app.at("/stations/:station/stream-stats/now").get(
    stations::stream_stats::now::get::Endpoint {
      index: stream_connections_index.clone(),
    }
    .into_handler(),
  );

  app.at("/stations/:station/stream-stats/now/count").get(
    stations::stream_stats::now::count::get::Endpoint {
      index: stream_connections_index.clone(),
    }
    .into_handler(),
  );

  app
    .at("/stations/:station/stream-stats/last-:num([0-9]+):unit(ms|s|min|h|d|w)")
    .get(
      stations::stream_stats::since::get::Endpoint {
        index: stream_connections_index.clone(),
      }
      .into_handler(),
    );

  app
    .at("/stations/:station/stream-stats/last-:num([0-9]+):unit(ms|s|min|h|d|w)/count")
    .get(
      stations::stream_stats::since::count::get::Endpoint {
        index: stream_connections_index,
      }
      .into_handler(),
    );

  app.at("/stations/:station/restart-playlist").post(
    stations::restart_playlist::post::Endpoint {
      deployment_id: deployment_id.clone(),
      media_sessions: media_sessions.clone(),
      shutdown,
      drop_tracer,
    }
    .into_handler(),
  );

  app
    .at("/stations/:station/files")
    .get(stations::files::get::Endpoint {}.into_handler())
    .post(
      stations::files::post::Endpoint {
        deployment_id: deployment_id.clone(),
      }
      .into_handler(),
    );

  app
    .at("/stations/:station/files/shuffle")
    .post(stations::files::shuffle::post::Endpoint {}.into_handler());

  app
    .at("/stations/:station/files/unshuffle")
    .post(stations::files::unshuffle::post::Endpoint {}.into_handler());

  app
    .at("/stations/:station/files/:file")
    .get(stations::files::id::get::Endpoint {}.into_handler())
    .delete(stations::files::id::delete::Endpoint {}.into_handler());

  app
    .at("/stations/:station/files/:file/stream")
    .get(stations::files::id::stream::Handler {});

  app
    .at("/stations/:station/files/:file/metadata")
    .put(stations::files::metadata::put::Endpoint {}.into_handler());

  app
    .at("/stations/:station/files/:file/order/swap")
    .post(stations::files::order::swap::post::Endpoint {}.into_handler());

  app
    .at("/stations/:station/files/:file/order/move-before")
    .post(stations::files::order::move_before::post::Endpoint {}.into_handler());

  app
    .at("/stations/:station/files/:file/order/move-after")
    .post(stations::files::order::move_after::post::Endpoint {}.into_handler());

  app
    .at("/stations/:station/files/:file/order/move-to-first")
    .post(stations::files::order::move_to_first::post::Endpoint {}.into_handler());

  app
    .at("/stations/:station/files/:file/order/move-to-last")
    .post(stations::files::order::move_to_last::post::Endpoint {}.into_handler());

  app
    .at("/stations/:station/now-playing")
    .get(stations::now_playing::get::Endpoint {}.into_handler());

  // app
  //   .at("/stations/:station/dashboard-stats")
  //   .get(stations::dashboard_stats::get::Endpoint {}.into_handler());

  app.at("/stations/:station/reset-source-password").post(
    stations::reset_source_password::post::Endpoint {
      deployment_id,
      media_sessions,
    }
    .into_handler(),
  );

  app
    .at("/station-pictures")
    .post(station_pictures::post::Endpoint {}.into_handler());

  for size in StationPicture::WEBP_SIZES.iter().copied() {
    let handler = station_pictures::StationPicHandler::Webp(size);
    let path = format!("/station-pictures/webp/{}/:picture.webp", size as u32);
    app.get(path, handler);
  }

  for size in StationPicture::PNG_SIZES.iter().copied() {
    let handler = station_pictures::StationPicHandler::Png(size);
    let path = format!("/station-pictures/png/{}/:picture.png", size as u32);
    app.get(path, handler);
  }

  app.get(
    "/station-pictures/src/:picture",
    station_pictures::StationPicHandler::Source,
  );

  app
    .at("/admins")
    .get(admins::get::Endpoint {}.into_handler())
    .post(admins::post::Endpoint {}.into_handler());

  app
    .at("/admins/:admin")
    .get(admins::id::get::Endpoint {}.into_handler())
    .patch(admins::id::patch::Endpoint {}.into_handler());

  app
    .at("/admins/:admin/change-password")
    .post(admins::change_password::post::Endpoint {}.into_handler());

  app
    .at("/invitations")
    .get(invitations::get::Endpoint {}.into_handler())
    .post(invitations::post::Endpoint { mailer }.into_handler());

  app
    .at("/invitations/:invitation")
    .get(invitations::id::get::Endpoint {}.into_handler())
    .delete(invitations::id::delete::Endpoint {}.into_handler());

  app
    .at("/invitations/get-by-token/:token")
    .get(invitations::get_by_token::get::Endpoint {}.into_handler());

  app
    .at("/invitations/accept")
    .post(invitations::accept::post::Endpoint {}.into_handler());

  app
    .at("/invitations/reject")
    .post(invitations::reject::post::Endpoint {}.into_handler());

  app
    .at("/payment-methods")
    .get(payment_methods::get::Endpoint {}.into_handler())
    .post(payment_methods::post::Endpoint { payments_client }.into_handler());

  app
    .at("/payment-methods/:payment_method")
    .get(payment_methods::id::get::Endpoint {}.into_handler());

  app
    .at("/stream-connections")
    .get(stream_connections::get::Endpoint {}.into_handler());

  app
    .at("/stream-connections-lite")
    .get(stream_connections_lite::get::Endpoint {}.into_handler());

  app
    .at("/embed/station/:station")
    .get(embed::station::id::get::Endpoint {}.into_handler());

  // 404 catch all
  app.with(ResourceNotFound.into_handler());

  app
}

#[derive(Debug, Clone, Copy)]
pub struct ResourceNotFound;

#[async_trait]
impl JsonHandler for ResourceNotFound {
  type Input = ();
  type Output = ();
  type HandleError = ApiError;
  type ParseError = ApiError;

  async fn parse(&self, _: Request) -> Result<(), ApiError> {
    Err(ApiError::ResourceNotFound)
  }

  async fn perform(&self, _: ()) -> Result<(), ApiError> {
    Err(ApiError::ResourceNotFound)
  }
}
