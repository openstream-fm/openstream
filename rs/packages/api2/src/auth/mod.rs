use db::access_token::{AccessToken, Scope};
use db::admin::Admin;
use db::user::User;
use db::{current_filter_doc, Model};

#[derive(Debug, Clone)]
pub enum AccessScope {
  User(db::user::User),
  Admin(db::admin::Admin),
  Global,
}

pub trait GetHeader {
  fn get_header<'a>(&'a self, key: &str) -> Option<Result<&'a str, Utf8HeaderError>>;
}

#[derive(Debug, Clone, Copy, thiserror::Error)]
#[error("Token header is not utf-8 encoded")]
pub struct Utf8HeaderError {}

#[derive(Debug, Clone, thiserror::Error)]
pub enum GetAccessScopeError {
  #[error("Access token missing")]
  Missing,
  #[error("Access token malformed or invalid")]
  Invalid,
  #[error("Access token not found or already deleted")]
  NotFound,
  #[error("Access token user not found or deleted")]
  UserNotFound(String),
  #[error("Access token admin not found or deleted")]
  AdminNotFound(String),
  #[error("Internal error authorizing request")]
  Db(#[from] mongodb::error::Error),
}

impl From<Utf8HeaderError> for GetAccessScopeError {
  fn from(_: Utf8HeaderError) -> Self {
    Self::Invalid
  }
}

impl AccessScope {
  pub async fn from_request_header<R: GetHeader>(request: &R) -> Result<Self, GetAccessScopeError> {
    match request.get_header(constants::ACCESS_TOKEN_HEADER) {
      None => Err(GetAccessScopeError::Missing),
      Some(Err(e)) => Err(e.into()),
      Some(Ok(id_key)) => Self::from_id_key(id_key).await,
    }
  }

  pub async fn from_db_document(
    token: db::access_token::AccessToken,
  ) -> Result<Self, GetAccessScopeError> {
    match token.scope {
      Scope::Global => Ok(Self::Global),

      Scope::Admin { admin_id } => {
        let filter = current_filter_doc! { Admin::KEY_ID: &admin_id };
        let admin = match Admin::get(filter).await? {
          None => return Err(GetAccessScopeError::AdminNotFound(admin_id)),
          Some(admin) => admin,
        };

        Ok(Self::Admin(admin))
      }

      Scope::User { user_id } => {
        let filter = current_filter_doc! { User::KEY_ID: &user_id };
        let user = match User::get(filter).await? {
          None => return Err(GetAccessScopeError::UserNotFound(user_id)),
          Some(user) => user,
        };

        Ok(Self::User(user))
      }

      Scope::AdminAsUser { admin_id, user_id } => {
        let filter = current_filter_doc! { Admin::KEY_ID: &admin_id };
        let admin_exists = Admin::exists(filter).await?;
        if !admin_exists {
          return Err(GetAccessScopeError::AdminNotFound(admin_id));
        };

        let filter = current_filter_doc! { User::KEY_ID: &user_id };
        let user = match User::get(filter).await? {
          None => return Err(GetAccessScopeError::UserNotFound(user_id)),
          Some(user) => user,
        };

        Ok(Self::User(user))
      }
    }
  }

  pub async fn from_id_key(id_key: &str) -> Result<Self, GetAccessScopeError> {
    let (id, key) = match id_key.split_once('-') {
      None => return Err(GetAccessScopeError::NotFound),
      Some((id, key)) => (id, key),
    };

    let hash = crypt::sha256(key);

    let filter = current_filter_doc! {
      AccessToken::KEY_ID: id,
      AccessToken::KEY_HASH: &hash,
    };

    let token = match AccessToken::get(filter).await? {
      None => return Err(GetAccessScopeError::NotFound),
      Some(token) => token,
    };

    Self::from_db_document(token).await
  }
}
