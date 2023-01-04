use prex::router::builder::Builder;
use prex::Request;

use crate::error::ApiError;
use crate::json::JsonHandler;

use async_trait::async_trait;

pub mod accounts;
pub mod admins;
pub mod auth;
pub mod me;
pub mod users;

pub fn router() -> Builder {
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
    .put(accounts::id::patch::Endpoint {}.into_handler());

  app
    .at("/accounts/:account/files")
    .get(accounts::files::get::Endpoint {}.into_handler())
    .post(accounts::files::post::Endpoint {}.into_handler());

  app
    .at("/accounts/:account/files/:file")
    .get(accounts::files::id::get::Endpoint {}.into_handler())
    .delete(accounts::files::id::delete::Endpoint {}.into_handler());

  app
    .at("/accounts/:account/files/:file/stream")
    .get(accounts::files::id::stream::Handler {});

  app
    .at("/admins")
    .get(admins::get::Endpoint {}.into_handler())
    .post(admins::post::Endpoint {}.into_handler());

  app
    .at("/admins/:admin")
    .get(admins::id::get::Endpoint {}.into_handler())
    .put(admins::id::patch::Endpoint {}.into_handler());

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
