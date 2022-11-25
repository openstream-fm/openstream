use prex::router::builder::Builder;

use crate::error::{ApiError, Kind};
use crate::json::JsonHandler;

pub mod accounts;
pub mod login;
pub mod users;

pub fn router() -> Builder {
  let mut app = prex::prex();

  app.post("/login", login::post::Endpoint {}.into_handler());

  app.get("/accounts", accounts::get::Endpoint {}.into_handler());

  app.at("/accounts/:account").nest({
    let mut account = prex::prex();

    account.get("/", accounts::id::get::Endpoint {}.into_handler());

    account.at("/files").nest({
      let mut files = prex::prex();
      files.get("/", accounts::files::get::Endpoint {}.into_handler());
      files.get(
        "/:file",
        accounts::files::id::get::Endpoint {}.into_handler(),
      );
      files
    });

    account
  });

  // 404 catch all
  app.with(|_, _| async { ApiError::from(Kind::ResourceNotFound).into_json_response() });

  app
}
