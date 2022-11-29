use crate::error::{ApiError, Kind};
use crate::ip_limit;

use db::PublicScope;
use db::{
  access_token::{AccessToken, Scope},
  account::Account,
  user::User,
  Model,
};
use prex::Request;

pub static X_ACCESS_TOKEN: &str = "x-access-token";

#[derive(Debug, Clone)]
pub enum GetAccessTokenScopeError {
  TooManyRequests,
  Db(mongodb::error::Error),
  Missing,
  NonUtf8,
  NotFound,
  UserNotFound(String),
  AccountNotFound(String),
  OutOfScope,
}

#[derive(Debug, Clone)]
pub enum AccessTokenScope {
  Global,
  Admin,
  User(User),
}

impl AccessTokenScope {
  pub fn as_public_scope(&self) -> PublicScope {
    match self {
      Self::Global | Self::Admin => PublicScope::Admin,
      Self::User(_) => PublicScope::User,
    }
  }

  pub fn has_full_access(&self) -> bool {
    self.is_global() || self.is_admin()
  }

  pub fn is_global(&self) -> bool {
    matches!(self, Self::Global)
  }

  pub fn is_admin(&self) -> bool {
    matches!(self, Self::Admin)
  }

  pub async fn grant_scope(&self, account_id: &str) -> Result<Account, GetAccessTokenScopeError> {
    let account = Account::get_by_id(account_id).await?;
    match account {
      None => Err(GetAccessTokenScopeError::AccountNotFound(
        account_id.to_string(),
      )),

      Some(account) => match self {
        AccessTokenScope::Global => Ok(account),
        AccessTokenScope::Admin => Ok(account),
        AccessTokenScope::User(user) => {
          if !user.account_ids.contains(&account.id) {
            return Err(GetAccessTokenScopeError::OutOfScope);
          }

          Ok(account)
        }
      },
    }
  }
}

pub async fn get_access_token_scope(
  req: &Request,
) -> Result<AccessTokenScope, GetAccessTokenScopeError> {
  let ip = req.isomorphic_ip();

  if ip_limit::should_reject(ip) {
    return Err(GetAccessTokenScopeError::TooManyRequests);
  }

  let token_id = match req.headers().get(X_ACCESS_TOKEN) {
    None => return Err(GetAccessTokenScopeError::Missing),
    Some(v) => match v.to_str() {
      Err(_) => return Err(GetAccessTokenScopeError::NonUtf8),
      Ok(v) => v,
    },
  };

  let doc = match AccessToken::touch(token_id).await? {
    None => {
      ip_limit::hit(ip);
      return Err(GetAccessTokenScopeError::NotFound);
    }
    Some(doc) => doc,
  };

  let scope = match doc.scope {
    Scope::Global => AccessTokenScope::Global,

    Scope::Admin { admin_id: _ } => AccessTokenScope::Admin,

    Scope::User { user_id } => match User::get_by_id(&user_id).await? {
      None => return Err(GetAccessTokenScopeError::UserNotFound(user_id)),
      Some(user) => AccessTokenScope::User(user),
    },
  };

  Ok(scope)
}

impl From<mongodb::error::Error> for GetAccessTokenScopeError {
  fn from(e: mongodb::error::Error) -> Self {
    Self::Db(e)
  }
}

impl From<GetAccessTokenScopeError> for ApiError {
  fn from(v: GetAccessTokenScopeError) -> ApiError {
    use GetAccessTokenScopeError::*;
    match v {
      Db(e) => ApiError::from(e),
      TooManyRequests => ApiError::from(Kind::TooManyRequests),
      Missing => ApiError::from(Kind::TokenMissing),
      NonUtf8 => ApiError::from(Kind::TokenMalformed),
      NotFound => ApiError::from(Kind::TokenNotFound),
      UserNotFound(id) => ApiError::from(Kind::TokenUserNotFound(id)),
      OutOfScope => ApiError::from(Kind::TokenOutOfScope),
      AccountNotFound(id) => ApiError::from(Kind::AccountNotFound(id)),
    }
  }
}

#[derive(Debug, Clone, Copy)]
pub struct GrantScopeError;

impl From<GrantScopeError> for GetAccessTokenScopeError {
  fn from(_: GrantScopeError) -> Self {
    Self::OutOfScope
  }
}
