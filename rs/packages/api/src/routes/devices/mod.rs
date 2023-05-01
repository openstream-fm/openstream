pub mod id;

use crate::json::JsonHandler;
use crate::request_ext::{self, AccessTokenScope, GetAccessTokenScopeError};

use crate::error::ApiError;
use async_trait::async_trait;
use db::access_token::{AccessToken, GeneratedBy, Scope};
use db::{Model, Paged};
use mongodb::bson::doc;
use prex::Request;
use serde::{Deserialize, Serialize};
use serde_util::DateTime;
use std::net::IpAddr;
use ts_rs::TS;
use user_agent::UserAgent;

#[derive(Debug, Clone, Serialize, Deserialize, TS)]
#[ts(export, export_to = "../../../defs/")]
pub struct Device {
  #[serde(rename = "_id")]
  id: String,
  is_current: bool,
  #[serde(with = "serde_util::ip")]
  ip: IpAddr,
  ua: UserAgent,
  created_at: DateTime,
  last_used_at: Option<DateTime>,
  user_id: Option<String>,
  admin_id: Option<String>,
}

pub mod get {

  use db::current_filter_doc;

  use super::*;

  #[derive(Debug, Clone)]
  pub struct Endpoint {}

  pub const DEFAULT_SKIP: u64 = 0;
  pub const DEFAULT_LIMIT: i64 = 60;

  pub fn default_skip() -> u64 {
    DEFAULT_SKIP
  }

  pub fn default_limit() -> i64 {
    DEFAULT_LIMIT
  }

  #[derive(Debug, Clone, Serialize, Deserialize, TS, Default)]
  #[ts(export, export_to = "../../../defs/api/devices/GET/")]
  struct Query {
    #[serde(skip_serializing_if = "Option::is_none")]
    skip: Option<u64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    limit: Option<i64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    user_id: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    admin_id: Option<String>,
  }

  #[derive(Debug, Clone)]
  pub struct Input {
    current_token_id: String,
    access_token_scope: AccessTokenScope,
    query: Query,
  }

  #[derive(Debug, Clone, Serialize, Deserialize, TS)]
  #[ts(export, export_to = "../../../defs/api/devices/GET/")]
  pub struct Output(Paged<Device>);

  #[derive(Debug, thiserror::Error)]
  pub enum ParseError {
    #[error("access: {0}")]
    Access(#[from] GetAccessTokenScopeError),
    #[error("querystring: {0}")]
    QueryString(#[from] serde_querystring::de::Error),
  }

  impl From<ParseError> for ApiError {
    fn from(e: ParseError) -> ApiError {
      match e {
        ParseError::Access(e) => e.into(),
        ParseError::QueryString(e) => e.into(),
      }
    }
  }

  #[async_trait]
  impl JsonHandler for Endpoint {
    type Input = Input;
    type Output = Output;
    type ParseError = ParseError;
    type HandleError = mongodb::error::Error;

    async fn parse(&self, req: Request) -> Result<Self::Input, Self::ParseError> {
      let access_token = request_ext::get_access_token(&req).await?;
      let access_token_scope = request_ext::get_scope_from_token(&req, &access_token).await?;

      let query = match req.uri().query() {
        None => Default::default(),
        Some(qs) => serde_querystring::from_str(qs, serde_querystring::de::ParseMode::UrlEncoded)?,
      };

      Ok(Self::Input {
        current_token_id: access_token.id,
        access_token_scope,
        query,
      })
    }

    async fn perform(&self, input: Self::Input) -> Result<Self::Output, Self::HandleError> {
      let Self::Input {
        current_token_id,
        access_token_scope,
        query,
      } = input;

      let generated_filter = doc! { GeneratedBy::KEY_ENUM_TAG: { "$in": [ GeneratedBy::KEY_ENUM_VARIANT_LOGIN, GeneratedBy::KEY_ENUM_VARIANT_REGISTER ] } };

      let scope_filter = match access_token_scope {
        AccessTokenScope::Global => match &query.admin_id {
          Some(admin_id) => {
            doc! { Scope::KEY_ENUM_TAG: Scope::KEY_ENUM_VARIANT_ADMIN , Scope::KEY_ADMIN_ID: admin_id }
          }

          None => match &query.user_id {
            Some(user_id) => {
              doc! { Scope::KEY_ENUM_TAG: Scope::KEY_ENUM_VARIANT_USER, Scope::KEY_USER_ID: user_id }
            }

            None => doc! {},
          },
        },

        AccessTokenScope::Admin(admin) => match &query.user_id {
          None => {
            doc! { Scope::KEY_ENUM_TAG: Scope::KEY_ENUM_VARIANT_ADMIN, Scope::KEY_ADMIN_ID: admin.id }
          }
          Some(user_id) => {
            doc! { Scope::KEY_ENUM_TAG: Scope::KEY_ENUM_VARIANT_USER, Scope::KEY_USER_ID: user_id }
          }
        },

        AccessTokenScope::User(user) => {
          doc! { Scope::KEY_ENUM_TAG: Scope::KEY_ENUM_VARIANT_USER, Scope::KEY_USER_ID: user.id }
        }
      };

      let filter = current_filter_doc! { "$and": [ generated_filter, scope_filter ] };

      let skip = query.skip.unwrap_or_else(default_skip);
      let limit = query.limit.unwrap_or_else(default_limit);

      // null are the smallest in mongodb so this is ok
      let sort = doc! { AccessToken::KEY_LAST_USED_AT: -1 };

      let page = AccessToken::paged(filter, sort, skip, limit).await?;

      let out = page.map(|token| {
        let is_current = token.id == current_token_id;

        let (ip, ua) = match token.generated_by {
          GeneratedBy::Login { ip, user_agent, .. } => (ip, user_agent),
          GeneratedBy::Register { ip, user_agent, .. } => (ip, user_agent),
          // unreachable: the mongodb filter ensures this invariants
          _ => unreachable!(),
        };

        let (admin_id, user_id) = match token.scope {
          Scope::Admin { admin_id } => (Some(admin_id), None),
          Scope::User { user_id } => (None, Some(user_id)),
          Scope::AdminAsUser { .. } => (None, None),
          Scope::Global => (None, None),
        };

        Device {
          id: token.id,
          is_current,
          ip,
          ua,
          created_at: token.created_at,
          last_used_at: token.last_used_at,
          user_id,
          admin_id,
        }
      });

      Ok(Output(out))
    }
  }
}
