use db::{
  access_token::{AccessToken, Scope},
  account::Account,
  user::User,
  Model,
};
use prex::Request;

use crate::error::{ApiError, Kind};

pub static X_ACCESS_TOKEN: &'static str = "x-access-token";

#[derive(Debug, Clone)]
pub enum GetAccessTokenScopeError {
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
  Admin,
  User(User),
}

impl AccessTokenScope {
  pub fn is_admin(&self) -> bool {
    matches!(self, Self::Admin)
  }

  pub async fn grant_scope(&self, account_id: &str) -> Result<Account, GetAccessTokenScopeError> {
    let account = Account::get_by_id(account_id).await?;
    match account {
      None => {
        return Err(GetAccessTokenScopeError::AccountNotFound(
          account_id.to_string(),
        ))
      }
      Some(account) => match self {
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
  let token_id = match req.headers().get(X_ACCESS_TOKEN) {
    None => return Err(GetAccessTokenScopeError::Missing),
    Some(v) => match v.to_str() {
      Err(_) => return Err(GetAccessTokenScopeError::NonUtf8),
      Ok(v) => v,
    },
  };

  let doc = match AccessToken::touch(token_id).await? {
    None => return Err(GetAccessTokenScopeError::NotFound),
    Some(doc) => doc,
  };

  let scope = match doc.scope {
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

impl Into<ApiError> for GetAccessTokenScopeError {
  fn into(self) -> ApiError {
    match self {
      Self::Db(e) => ApiError::from(e),
      Self::Missing => ApiError::from(Kind::TokenMissing),
      Self::NonUtf8 => ApiError::from(Kind::TokenMalformed),
      Self::NotFound => ApiError::from(Kind::TokenNotFound),
      Self::UserNotFound(id) => ApiError::from(Kind::TokenUserNotFound(id)),
      Self::OutOfScope => ApiError::from(Kind::TokenOutOfScope),
      Self::AccountNotFound(id) => ApiError::from(Kind::AccountNotFound(id)),
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
