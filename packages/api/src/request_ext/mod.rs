use crate::error::ApiError;
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

#[derive(Debug, Clone, thiserror::Error)]
pub enum GetAccessTokenScopeError {
  #[error("too many requests")]
  TooManyRequests,

  #[error("mongo error: {0}")]
  Db(#[from] mongodb::error::Error),

  #[error("token missing")]
  Missing,

  #[error("token not utf8")]
  NonUtf8,

  #[error("token not found")]
  NotFound,

  #[error("token user not found: {0}")]
  UserNotFound(String),

  #[error("token account not found: {0}")]
  AccountNotFound(String),

  #[error("token admin not found: {0}")]
  AdminNotFound(String),

  #[error("token out of scope")]
  OutOfScope,

  #[error("admin not found: {0}")]
  ResolveAdminNotFound(String),

  #[error("user not found: {0}")]
  ResolveUserNotFound(String),

  #[error("unresolvable admin 'me'")]
  UnresolvableAdminMe,

  #[error("unresolvable user 'me'")]
  UnresolvableUserMe,
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

  pub async fn grant_user_scope(&self, user_id: &str) -> Result<User, GetAccessTokenScopeError> {
    match self {
      AccessTokenScope::User(user) => {
        if user_id == "me" || user_id == user.id {
          Ok(user.clone())
        } else {
          Err(GetAccessTokenScopeError::OutOfScope)
        }
      }

      AccessTokenScope::Admin(_) | AccessTokenScope::Global => {
        if user_id == "me" {
          return Err(GetAccessTokenScopeError::UnresolvableUserMe);
        }

        match User::get_by_id(user_id).await? {
          None => Err(GetAccessTokenScopeError::ResolveUserNotFound(
            user_id.to_string(),
          )),
          Some(user) => Ok(user),
        }
      }
    }
  }

  pub async fn grant_admin_write_scope(
    &self,
    admin_id: &str,
  ) -> Result<Admin, GetAccessTokenScopeError> {
    match self {
      AccessTokenScope::User(_) => Err(GetAccessTokenScopeError::OutOfScope),
      AccessTokenScope::Admin(admin) => {
        if admin_id == "me" || admin_id == admin.id {
          Ok(admin.clone())
        } else {
          Err(GetAccessTokenScopeError::OutOfScope)
        }
      }

      AccessTokenScope::Global => {
        if admin_id == "me" {
          return Err(GetAccessTokenScopeError::UnresolvableAdminMe);
        }

        match Admin::get_by_id(admin_id).await? {
          None => Err(GetAccessTokenScopeError::ResolveAdminNotFound(
            admin_id.to_string(),
          )),
          Some(admin) => Ok(admin),
        }
      }
    }
  }

  pub async fn grant_admin_read_scope(
    &self,
    admin_id: &str,
  ) -> Result<Admin, GetAccessTokenScopeError> {
    match self {
      AccessTokenScope::User(_) => Err(GetAccessTokenScopeError::OutOfScope),
      AccessTokenScope::Admin(admin) => {
        if admin_id == "me" || admin_id == admin.id {
          Ok(admin.clone())
        } else {
          match Admin::get_by_id(admin_id).await? {
            None => Err(GetAccessTokenScopeError::ResolveAdminNotFound(
              admin_id.to_string(),
            )),
            Some(admin) => Ok(admin),
          }
        }
      }
      AccessTokenScope::Global => match Admin::get_by_id(admin_id).await? {
        None => Err(GetAccessTokenScopeError::ResolveAdminNotFound(
          admin_id.to_string(),
        )),
        Some(admin) => Ok(admin),
      },
    }
  }
}

pub async fn get_access_token(req: &Request) -> Result<AccessToken, GetAccessTokenScopeError> {
  let ip = req.isomorphic_ip();

  if ip_limit::should_reject(ip) {
    return Err(GetAccessTokenScopeError::TooManyRequests);
  }

  let key: String = match req.headers().get(X_ACCESS_TOKEN) {
    None => return Err(GetAccessTokenScopeError::Missing),
    Some(v) => match v.to_str() {
      Err(_) => return Err(GetAccessTokenScopeError::NonUtf8),
      Ok(v) => v.to_string(),
    },
  };

  let doc = match AccessToken::touch(&key).await? {
    None => {
      ip_limit::hit(ip);
      return Err(GetAccessTokenScopeError::NotFound);
    }
    Some(doc) => doc,
  };

  Ok(doc)
}

pub async fn get_access_token_scope(
  req: &Request,
) -> Result<AccessTokenScope, GetAccessTokenScopeError> {
  let doc = get_access_token(req).await?;

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

impl From<GetAccessTokenScopeError> for ApiError {
  fn from(v: GetAccessTokenScopeError) -> ApiError {
    use GetAccessTokenScopeError::*;
    match v {
      Db(e) => ApiError::from(e),
      TooManyRequests => ApiError::TooManyRequests,
      Missing => ApiError::TokenMissing,
      NonUtf8 => ApiError::TokenMalformed,
      NotFound => ApiError::TokenNotFound,
      OutOfScope => ApiError::TokenOutOfScope,
      UserNotFound(id) => ApiError::TokenUserNotFound(id),
      AdminNotFound(id) => ApiError::TokenAdminNotFound(id),
      AccountNotFound(id) => ApiError::AccountNotFound(id),
      ResolveAdminNotFound(id) => ApiError::AdminNotFound(id),
      ResolveUserNotFound(id) => ApiError::UserNotFound(id),
      UnresolvableAdminMe => ApiError::UnresolvableAdminMe,
      UnresolvableUserMe => ApiError::UnresolvableUserMe,
    }
  }
}
