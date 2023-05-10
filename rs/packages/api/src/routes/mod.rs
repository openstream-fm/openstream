use db::station_picture::StationPicture;
use db::stream_connection::index::MemIndex;
use drop_tracer::DropTracer;
use media_sessions::MediaSessionMap;
use prex::router::builder::Builder;
use prex::Request;
use shutdown::Shutdown;

use crate::error::ApiError;
use crate::json::JsonHandler;

use async_trait::async_trait;

pub mod accounts;
pub mod admins;
pub mod auth;
pub mod me;
pub mod stations;
pub mod users;

pub mod devices;
pub mod plans;
pub mod runtime;
pub mod station_pictures;
pub mod stream_stats;

pub fn router(
  deployment_id: String,
  media_sessions: MediaSessionMap,
  shutdown: Shutdown,
  drop_tracer: DropTracer,
  stream_connections_index: MemIndex,
) -> Builder {
  let mut app = prex::prex();

  app.at("/me").get(me::get::Endpoint {}.into_handler());

  app
    .at("/auth/user/login")
    .post(auth::user::login::post::Endpoint {}.into_handler());

  app
    .at("/auth/user/logout")
    .post(auth::user::logout::post::Endpoint {}.into_handler());

  app
    .at("/auth/user/register")
    .post(auth::user::register::post::Endpoint {}.into_handler());

  app
    .at("/auth/user/recover")
    .post(auth::user::recover::post::Endpoint {}.into_handler());

  app
    .at("/auth/admin/login")
    .post(auth::admin::login::post::Endpoint {}.into_handler());

  app
    .at("/auth/admin/logout")
    .post(auth::admin::logout::post::Endpoint {}.into_handler());

  app.at("/runtime/source-password-updated/:station").post(
    runtime::source_password_updated::station_id::post::Endpoint {
      media_sessions: media_sessions.clone(),
    }
    .into_handler(),
  );

  app.at("/runtime/restart-playlist/:station").post(
    runtime::restart_playlist::station_id::post::Endpoint {
      media_sessions: media_sessions.clone(),
      drop_tracer: drop_tracer.clone(),
      shutdown: shutdown.clone(),
    }
    .into_handler(),
  );

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
    .patch(users::id::patch::Endpoint {}.into_handler());

  app
    .at("/accounts")
    .get(accounts::get::Endpoint {}.into_handler())
    .post(accounts::post::Endpoint {}.into_handler());

  app
    .at("/accounts/:account")
    .get(accounts::id::get::Endpoint {}.into_handler())
    .patch(accounts::id::patch::Endpoint {}.into_handler());

  app
    .at("/accounts/:account/members")
    .get(accounts::members::get::Endpoint {}.into_handler());

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
    .at("/stations/:station")
    .get(stations::id::get::Endpoint {}.into_handler())
    .patch(stations::id::patch::Endpoint {}.into_handler());

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
    .put(stations::files::metadata::patch::Endpoint {}.into_handler());

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
    .at("/devices")
    .get(devices::get::Endpoint {}.into_handler());

  app
    .at("/devices/:device")
    .delete(devices::id::delete::Endpoint {}.into_handler());

  app
    .at("/station-pictures")
    .post(station_pictures::post::Endpoint {}.into_handler());

  for size in StationPicture::WEBP_SIZES {
    let handler = station_pictures::StationPicHandler::Webp(size);
    let path = format!("/station-pictures/webp/{}/:picture.webp", size as u32);
    app.get(path, handler);
  }

  for size in StationPicture::PNG_SIZES {
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
