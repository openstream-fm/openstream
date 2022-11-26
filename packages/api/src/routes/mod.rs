use prex::router::builder::Builder;

use crate::error::{ApiError, Kind};
use crate::json::JsonHandler;

pub mod accounts;
pub mod login;
pub mod register;
pub mod users;

pub fn router() -> Builder {
  let mut app = prex::prex();

  app
    .at("/register")
    .post(register::post::Endpoint {}.into_handler());

  app
    .at("/login")
    .post(login::post::Endpoint {}.into_handler());

  app.at("/users").get(users::get::Endpoint {}.into_handler());

  app
    .at("/users/:user")
    .get(users::post::Endpoint {}.into_handler());

  app
    .at("/accounts")
    .get(accounts::get::Endpoint {}.into_handler())
    .post(accounts::post::Endpoint {}.into_handler());

  app
    .at("/accounts/:account")
    .get(accounts::id::get::Endpoint {}.into_handler());

  app
    .at("/accounts/:account/files")
    .get(accounts::files::get::Endpoint {}.into_handler());

  app
    .at("/accounts/:account/files/:file")
    .get(accounts::files::id::get::Endpoint {}.into_handler());

  // 404 catch all
  app.with(|_, _| async { ApiError::from(Kind::ResourceNotFound).into_json_response() });

  app
}
