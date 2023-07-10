pub mod get {

  use async_trait::async_trait;
  use db::station::Station;
  use db::stream_connection::StreamConnection;
  use db::user_account_relation::UserAccountRelation;
  use db::Model;
  use db::Paged;
  use mongodb::bson::doc;
  use prex::Request;
  use serde::{Deserialize, Serialize};
  use ts_rs::TS;

  use crate::request_ext::{self, AccessTokenScope, GetAccessTokenScopeError};
  use crate::{error::ApiError, json::JsonHandler};

  #[derive(Debug, Clone)]
  pub struct Endpoint {}

  #[derive(Debug, Clone, Serialize, Deserialize, TS)]
  #[ts(export, export_to = "../../../defs/api/stream-connections/GET/")]
  #[serde(rename_all = "kebab-case")]
  pub enum ShowQuery {
    All,
    Open,
    Closed,
  }

  #[derive(Debug, Clone, Serialize, Deserialize, TS)]
  #[ts(export, export_to = "../../../defs/api/stream-connections/GET/")]
  #[serde(rename_all = "kebab-case")]
  pub enum SortQuery {
    CreationAsc,
    CreationDesc,
  }

  #[derive(Debug, Clone, Serialize, Deserialize, TS, Default)]
  #[ts(export, export_to = "../../../defs/api/stream-connections/GET/")]
  struct Query {
    #[serde(skip_serializing_if = "Option::is_none")]
    show: Option<ShowQuery>,
    #[serde(skip_serializing_if = "Option::is_none")]
    sort: Option<SortQuery>,
    #[serde(skip_serializing_if = "Option::is_none")]
    limit: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    skip: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    stations: Option<Vec<String>>,
  }

  #[derive(Debug, Clone)]
  pub struct Input {
    access_token_scope: AccessTokenScope,
    query: Query,
  }

  #[derive(Debug, Clone, Serialize, Deserialize, TS)]
  #[ts(export, export_to = "../../../defs/api/stream-connections/GET/")]
  pub struct Output(Paged<StreamConnection>);

  #[derive(Debug, thiserror::Error)]
  pub enum ParseError {
    #[error("token: {0}")]
    Token(#[from] GetAccessTokenScopeError),
    #[error("querystring: {0}")]
    QueryString(#[from] serde_qs::Error),
  }

  impl From<ParseError> for ApiError {
    fn from(e: ParseError) -> ApiError {
      match e {
        ParseError::Token(e) => e.into(),
        ParseError::QueryString(e) => e.into(),
      }
    }
  }

  #[derive(Debug, thiserror::Error)]
  pub enum HandleError {
    #[error("db: {0}")]
    Db(#[from] mongodb::error::Error),
    #[error("token: {0}")]
    Token(#[from] GetAccessTokenScopeError),
  }

  impl From<HandleError> for ApiError {
    fn from(e: HandleError) -> ApiError {
      match e {
        HandleError::Db(e) => e.into(),
        HandleError::Token(e) => e.into(),
      }
    }
  }

  #[async_trait]
  impl JsonHandler for Endpoint {
    type Input = Input;
    type Output = Output;
    type ParseError = ParseError;
    type HandleError = HandleError;

    async fn parse(&self, req: Request) -> Result<Self::Input, Self::ParseError> {
      let query = match req.uri().query() {
        None => Default::default(),
        Some(_) => req.qs()?,
      };

      let access_token_scope = request_ext::get_access_token_scope(&req).await?;

      Ok(Self::Input {
        access_token_scope,
        query,
      })
    }

    async fn perform(&self, input: Input) -> Result<Output, Self::HandleError> {
      let Self::Input {
        access_token_scope,
        query,
      } = input;

      let Query {
        show,
        stations,
        sort,
        limit,
        skip,
      } = query;

      let limit = limit.unwrap_or(60);
      let skip = skip.unwrap_or(0);

      let scope_filter = match access_token_scope {
        AccessTokenScope::Global | AccessTokenScope::Admin(_) => doc! {},
        AccessTokenScope::User(user) => {
          let account_ids = {
            let filter = doc! { UserAccountRelation::KEY_USER_ID: &user.id };
            UserAccountRelation::cl()
              .distinct(UserAccountRelation::KEY_ACCOUNT_ID, filter, None)
              .await?
          };

          let scope_station_ids = {
            let filter = doc! { UserAccountRelation::KEY_ACCOUNT_ID: { "$in": account_ids } };
            Station::cl()
              .distinct(Station::KEY_ID, filter, None)
              .await?
          };

          doc! { StreamConnection::KEY_STATION_ID: { "$in": scope_station_ids } }
        }
      };

      let stations_query_filter = match stations {
        None => doc! {},
        Some(ids) => doc! { StreamConnection::KEY_STATION_ID: { "$in": ids } },
      };

      let show_filter = match show {
        None | Some(ShowQuery::All) => {
          doc! {}
        }
        Some(ShowQuery::Open) => {
          doc! { StreamConnection::KEY_IS_OPEN: true }
        }
        Some(ShowQuery::Closed) => {
          doc! { StreamConnection::KEY_IS_OPEN: false }
        }
      };

      let filter = doc! { "$and": [ scope_filter, stations_query_filter, show_filter ] };

      let sort = match sort {
        None | Some(SortQuery::CreationDesc) => {
          doc! { StreamConnection::KEY_CREATED_AT: -1 }
        }
        Some(SortQuery::CreationAsc) => {
          doc! { StreamConnection::KEY_CREATED_AT: 1 }
        }
      };

      let page = StreamConnection::paged(filter, sort, skip, limit).await?;

      Ok(Output(page))
    }
  }
}
