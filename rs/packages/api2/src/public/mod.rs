use crate::auth::AccessScope;

pub mod account;
pub mod admin;
pub mod payment_method;
pub mod request;
pub mod stream_connection;
pub mod user;
pub mod user_account_relation;
pub trait IntoPublic {
  type Target;
  fn into_public(self, scope: &AccessScope) -> Self::Target;
}
