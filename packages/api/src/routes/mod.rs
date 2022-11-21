use config::Tokens;
use prex::router::builder::Builder;

use crate::error::{ApiError, Kind};
use crate::json::JsonHandler;

pub mod accounts;

pub fn router(tokens: Tokens) -> Builder {
  let mut account = prex::prex();

  account.get(
    "/",
    accounts::id::get::Endpoint {
      tokens: tokens.clone(),
    }
    .into_handler(),
  );

  account.get(
    "/files",
    accounts::files::get::Endpoint {
      tokens: tokens.clone(),
    }
    .into_handler(),
  );

  account.get(
    "/files/:file",
    accounts::files::id::get::Endpoint {
      tokens: tokens.clone(),
    }
    .into_handler(),
  );

  let mut app = prex::prex();

  app.get(
    "/accounts",
    accounts::get::Endpoint { tokens }.into_handler(),
  );

  app.at("/accounts/:account").nest(account);

  // 404 catch all
  app.with(|_, _| async { ApiError::from(Kind::ResourceNotFound).into_json_response() });

  app
}
