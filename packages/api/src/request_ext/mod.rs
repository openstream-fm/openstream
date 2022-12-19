use crate::error::{ApiError, Kind};
use crate::ip_limit;

use db::admin::Admin;
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
  AdminNotFound(String),
  OutOfScope,
}

#[derive(Debug, Clone)]
pub enum AccessTokenScope {
  Global,
  Admin(Admin),
  User(User),
}

impl AccessTokenScope {
  pub fn as_public_scope(&self) -> PublicScope {
    match self {
      Self::Global | Self::Admin(_) => PublicScope::Admin,
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
    matches!(self, Self::Admin(_))
  }

  pub fn is_user(&self) -> bool {
    matches!(self, Self::User(_))
  }

  pub async fn grant_account_scope(
    &self,
    account_id: &str,
  ) -> Result<Account, GetAccessTokenScopeError> {
    match self {
      AccessTokenScope::Global | AccessTokenScope::Admin(_) => {}
      AccessTokenScope::User(user) => {
        if !user.account_ids.iter().any(|id| id == account_id) {
          return Err(GetAccessTokenScopeError::OutOfScope);
        }
      }
    }

    let account = Account::get_by_id(account_id).await?;

    match account {
      None => Err(GetAccessTokenScopeError::AccountNotFound(
        account_id.to_string(),
      )),

      Some(account) => Ok(account),
    }
  }

  pub async fn grant_admin_write_scope(
    &self,
    admin_id: &str,
  ) -> Result<Admin, GetAccessTokenScopeError> {
    match self {
      AccessTokenScope::User(_) => Err(GetAccessTokenScopeError::OutOfScope),
      AccessTokenScope::Admin(admin) => {
        if admin.id == admin_id {
          Ok(admin.clone())
        } else {
          Err(GetAccessTokenScopeError::OutOfScope)
        }
      }
      AccessTokenScope::Global => match Admin::get_by_id(admin_id).await? {
        Some(admin) => Ok(admin),
        None => Err(GetAccessTokenScopeError::AdminNotFound(
          admin_id.to_string(),
        )),
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

  let token_key = match req.headers().get(X_ACCESS_TOKEN) {
    None => return Err(GetAccessTokenScopeError::Missing),
    Some(v) => match v.to_str() {
      Err(_) => return Err(GetAccessTokenScopeError::NonUtf8),
      Ok(v) => v,
    },
  };

  let doc = match AccessToken::touch(token_key).await? {
    None => {
      ip_limit::hit(ip);
      return Err(GetAccessTokenScopeError::NotFound);
    }
    Some(doc) => doc,
  };

  let scope = match doc.scope {
    Scope::Global => AccessTokenScope::Global,

    Scope::Admin { admin_id } => match Admin::get_by_id(&admin_id).await? {
      None => return Err(GetAccessTokenScopeError::AdminNotFound(admin_id)),
      Some(admin) => AccessTokenScope::Admin(admin),
    },

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
      OutOfScope => ApiError::from(Kind::TokenOutOfScope),
      UserNotFound(id) => ApiError::from(Kind::TokenUserNotFound(id)),
      AdminNotFound(id) => ApiError::from(Kind::TokenAdminNotFound(id)),
      AccountNotFound(id) => ApiError::from(Kind::TokenAccountNotFound(id)),
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
