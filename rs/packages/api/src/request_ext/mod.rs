use crate::error::ApiError;
use crate::ip_limit;

use db::station::Station;
use mongodb::bson::doc;

use db::admin::Admin;
use db::models::user_account_relation::UserAccountRelation;
use db::{
  access_token::{AccessToken, Scope},
  account::Account,
  user::User,
  Model,
};
use db::{current_filter_doc, PublicScope};
use prex::Request;
use serde::{Deserialize, Serialize};
use serde_querystring::de::ParseMode;

pub static X_ACCESS_TOKEN: &str = "x-access-token";

#[derive(Debug, Clone, Serialize, Deserialize, ts_rs::TS)]
pub struct DelegateQuery {
  #[serde(default, skip_serializing_if = "Option::is_none")]
  #[ts(optional)]
  as_user: Option<String>,
}

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

  #[error("token station not found: {0}")]
  StationNotFound(String),

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

  pub fn is_admin_or_global(&self) -> bool {
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
        let filter = current_filter_doc! { UserAccountRelation::KEY_USER_ID: &user.id, UserAccountRelation::KEY_ACCOUNT_ID: account_id };
        let exists = UserAccountRelation::exists(filter).await?;
        if !exists {
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

  pub async fn grant_station_scope(
    &self,
    station_id: &str,
  ) -> Result<Station, GetAccessTokenScopeError> {
    let station = match Station::get_by_id(station_id).await? {
      None => {
        return Err(GetAccessTokenScopeError::StationNotFound(
          station_id.to_string(),
        ))
      }
      Some(station) => station,
    };

    self.grant_account_scope(&station.account_id).await?;

    Ok(station)
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
  internal_get_access_token(req, false).await
}

pub async fn get_media_access_token(
  req: &Request,
) -> Result<AccessToken, GetAccessTokenScopeError> {
  internal_get_access_token(req, true).await
}

async fn internal_get_access_token(
  req: &Request,
  media: bool,
) -> Result<AccessToken, GetAccessTokenScopeError> {
  let ip = req.isomorphic_ip();

  if ip_limit::should_reject(ip) {
    return Err(GetAccessTokenScopeError::TooManyRequests);
  }

  let doc = match req.headers().get(X_ACCESS_TOKEN) {
    Some(v) => {
      let id_key = match v.to_str() {
        Err(_) => return Err(GetAccessTokenScopeError::NonUtf8),
        Ok(v) => v,
      };

      match AccessToken::touch_cached(id_key).await? {
        None => {
          ip_limit::hit(ip);
          return Err(GetAccessTokenScopeError::NotFound);
        }

        Some(doc) => doc,
      }
    }

    None => {
      if !media {
        return Err(GetAccessTokenScopeError::Missing);
      } else {
        #[derive(Deserialize)]
        struct TokenQuery {
          token: String,
        }

        let id_media_key = match serde_querystring::from_str::<TokenQuery>(
          req.uri().query().unwrap_or(""),
          ParseMode::UrlEncoded,
        ) {
          Ok(qs) => qs.token,
          Err(_) => {
            return Err(GetAccessTokenScopeError::Missing);
          }
        };

        match AccessToken::touch_by_media_key(&id_media_key).await? {
          None => {
            ip_limit::hit(ip);
            return Err(GetAccessTokenScopeError::NotFound);
          }

          Some(doc) => doc,
        }
      }
    }
  };

  Ok(doc)
}

pub async fn internal_get_access_token_scope(
  req: &Request,
  media: bool,
) -> Result<AccessTokenScope, GetAccessTokenScopeError> {
  let doc = internal_get_access_token(req, media).await?;
  let scope = get_scope_from_token(req, &doc).await?;
  Ok(scope)
}

pub async fn get_scope_from_token(
  req: &Request,
  token: &AccessToken,
) -> Result<AccessTokenScope, GetAccessTokenScopeError> {
  macro_rules! delegate_if_needed {
    ($base:expr) => {
      match req.uri().query() {
        None => return Ok($base),
        Some(qs) => {
          let DelegateQuery { as_user } =
            match serde_querystring::from_str(qs, serde_querystring::de::ParseMode::UrlEncoded) {
              Err(_) => return Ok($base),
              Ok(qs) => qs,
            };

          let user_id = match as_user {
            None => return Ok($base),
            Some(user_id) => user_id,
          };

          match User::get_by_id(&user_id).await? {
            None => return Err(GetAccessTokenScopeError::UserNotFound(user_id.to_string())),
            Some(user) => return Ok(AccessTokenScope::User(user)),
          }
        }
      }
    };
  }

  let scope = match &token.scope {
    Scope::Global => delegate_if_needed!(AccessTokenScope::Global),

    Scope::Admin { admin_id } => match Admin::get_by_id(admin_id).await? {
      None => {
        return Err(GetAccessTokenScopeError::AdminNotFound(
          admin_id.to_string(),
        ))
      }
      Some(admin) => delegate_if_needed!(AccessTokenScope::Admin(admin)),
    },

    Scope::User { user_id } | Scope::AdminAsUser { user_id, .. } => {
      match User::get_by_id(user_id).await? {
        None => return Err(GetAccessTokenScopeError::UserNotFound(user_id.to_string())),
        Some(user) => AccessTokenScope::User(user),
      }
    }
  };

  Ok(scope)
}

pub async fn get_access_token_scope(
  req: &Request,
) -> Result<AccessTokenScope, GetAccessTokenScopeError> {
  internal_get_access_token_scope(req, false).await
}

// TODO: this should be in reverse order
pub async fn get_optional_access_token_scope(
  req: &Request,
) -> Result<Option<AccessTokenScope>, GetAccessTokenScopeError> {
  match get_access_token_scope(req).await {
    Ok(scope) => Ok(Some(scope)),
    Err(e) => match e {
      GetAccessTokenScopeError::Missing => Ok(None),
      _ => Err(e),
    },
  }
}

pub async fn get_media_access_token_scope(
  req: &Request,
) -> Result<AccessTokenScope, GetAccessTokenScopeError> {
  internal_get_access_token_scope(req, true).await
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
      StationNotFound(id) => ApiError::StationNotFound(id),
      ResolveAdminNotFound(id) => ApiError::AdminNotFound(id),
      ResolveUserNotFound(id) => ApiError::UserNotFound(id),
      UnresolvableAdminMe => ApiError::UnresolvableAdminMe,
      UnresolvableUserMe => ApiError::UnresolvableUserMe,
    }
  }
}
