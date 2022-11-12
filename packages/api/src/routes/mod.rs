use prex::router::builder::Builder;

use crate::json::JsonHandler;

pub mod account;

pub fn router() -> Builder {
  let mut app = prex::prex();

  app.get(
    "/accounts/:account_id",
    account::id::Endpoint {}.into_handler(),
  );

  app
}
