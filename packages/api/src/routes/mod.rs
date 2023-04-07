use db::station_picture::StationPicture;
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

pub mod station_pictures;

pub fn router(
  media_sessions: MediaSessionMap,
  shutdown: Shutdown,
  drop_tracer: DropTracer,
) -> Builder {
  let mut app = prex::prex();

  app.at("/me").get(me::get::Endpoint {}.into_handler());

  app
    .at("/auth/user/register")
    .post(auth::user::register::post::Endpoint {}.into_handler());

  app
    .at("/auth/user/login")
    .post(auth::user::login::post::Endpoint {}.into_handler());

  app
    .at("/auth/user/logout")
    .post(auth::user::logout::post::Endpoint {}.into_handler());

  app
    .at("/auth/admin/login")
    .post(auth::admin::login::post::Endpoint {}.into_handler());

  app
    .at("/users")
    .get(users::get::Endpoint {}.into_handler())
    .post(users::post::Endpoint {}.into_handler());

  app
    .at("/users/:user")
    .get(users::id::get::Endpoint {}.into_handler());

  app
    .at("/accounts")
    .get(accounts::get::Endpoint {}.into_handler())
    .post(accounts::post::Endpoint {}.into_handler());

  app
    .at("/accounts/:account")
    .get(accounts::id::get::Endpoint {}.into_handler())
    .patch(accounts::id::patch::Endpoint {}.into_handler());

  app
    .at("/stations")
    .get(stations::get::Endpoint {}.into_handler())
    .post(stations::post::Endpoint {}.into_handler());

  app
    .at("/stations/:station")
    .get(stations::id::get::Endpoint {}.into_handler())
    .patch(stations::id::patch::Endpoint {}.into_handler());

  app.at("/stations/:station/restart-playlist").post(
    stations::restart_playlist::post::Endpoint {
      media_sessions,
      shutdown,
      drop_tracer,
    }
    .into_handler(),
  );

  app
    .at("/stations/:station/files")
    .get(stations::files::get::Endpoint {}.into_handler())
    .post(stations::files::post::Endpoint {}.into_handler());

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

  app
    .at("/stations/:station/dashboard-stats")
    .get(stations::dashboard_stats::get::Endpoint {}.into_handler());

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
