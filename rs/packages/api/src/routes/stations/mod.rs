// pub mod dashboard_stats;
pub mod files;
pub mod id;
pub mod now_playing;
pub mod reset_source_password;
pub mod restart_playlist;
pub mod stream_stats;
pub mod transfer;

use crate::json::JsonHandler;
use crate::request_ext::{self, AccessTokenScope, GetAccessTokenScopeError};

use crate::error::ApiError;
use async_trait::async_trait;
use db::metadata::Metadata;
use db::models::user_account_relation::UserAccountRelation;
use db::station::PublicStation;
use db::station::{validation::*, Station};
use db::{Model, Paged, PublicScope};
use mongodb::bson::doc;
use prex::request::ReadBodyJsonError;
use prex::Request;
use serde::{Deserialize, Serialize};
use ts_rs::TS;

pub mod get {

  use crate::qs::{PaginationQs, VisibilityQs};

  use super::*;

  #[derive(Debug, Clone)]
  pub struct Endpoint {}

  #[derive(Debug, Clone, Serialize, Deserialize, TS, Default)]
  #[ts(export, export_to = "../../../defs/api/stations/GET/")]
  struct Query {
    #[serde(flatten)]
    pub page: PaginationQs,

    #[serde(flatten)]
    pub show: VisibilityQs,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_id: Option<String>,
  }

  #[derive(Debug, Clone)]
  pub struct Input {
    access_token_scope: AccessTokenScope,
    query: Query,
  }

  #[derive(Debug, Clone, Serialize, Deserialize, TS)]
  #[ts(export, export_to = "../../../defs/api/stations/GET/")]
  pub struct Output(Paged<PublicStation>);

  #[derive(Debug, thiserror::Error)]
  pub enum ParseError {
    #[error("access: {0}")]
    Access(#[from] GetAccessTokenScopeError),
    #[error("querystring: {0}")]
    QueryString(#[from] serde_qs::Error),
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
      let access_token_scope = request_ext::get_access_token_scope(&req).await?;

      let query = match req.uri().query() {
        None => Default::default(),
        Some(_) => req.qs()?,
      };

      Ok(Self::Input {
        access_token_scope,
        query,
      })
    }

    async fn perform(&self, input: Self::Input) -> Result<Self::Output, Self::HandleError> {
      let Self::Input {
        access_token_scope,
        query,
      } = input;

      let Query {
        page: PaginationQs { skip, limit },
        show: VisibilityQs { show },
        account_id,
      } = query;

      let mut filters = vec![show.to_filter_doc()];

      #[allow(clippy::single_match)]
      match account_id {
        Some(account_id) => filters.push(doc! { Station::KEY_ACCOUNT_ID: account_id }),
        None => {}
      };

      let sort = doc! { Station::KEY_CREATED_AT: 1 };

      let page = match access_token_scope {
        AccessTokenScope::Global | AccessTokenScope::Admin(_) => {
          Station::paged(doc! { "$and": filters }, sort, skip, limit)
            .await?
            .map(|item| item.into_public(PublicScope::Admin))
        }

        AccessTokenScope::User(user) => {
          let filter = doc! { UserAccountRelation::KEY_USER_ID: &user.id };
          let account_ids = UserAccountRelation::cl()
            .distinct(UserAccountRelation::KEY_ACCOUNT_ID, filter, None)
            .await?;

          if account_ids.is_empty() {
            return Ok(Output(Paged {
              items: vec![],
              limit,
              skip,
              total: 0,
            }));
          }

          filters.push(doc! { Station::KEY_ACCOUNT_ID: { "$in": account_ids } });

          Station::paged(doc! { "$and": filters }, sort, skip, limit)
            .await?
            .map(|item| item.into_public(PublicScope::User))
        }
      };

      Ok(Output(page))
    }
  }
}

pub mod post {
  use db::account::{Account, Limit, Limits};
  use db::run_transaction;
  use db::station::{Station, StationFrequency, StationTypeOfContent};
  use db::station_picture::StationPicture;
  use geoip::CountryCode;
  use serde_util::DateTime;
  use ts_rs::TS;
  use validate::url::patterns::*;
  use validify::{validify, ValidationErrors, Validify};

  use super::*;

  #[derive(Debug, Clone, Serialize, Deserialize, TS)]
  #[ts(export, export_to = "../../../defs/api/stations/POST/")]
  #[serde(rename_all = "snake_case")]
  #[serde(deny_unknown_fields)]
  #[validify]
  pub struct Payload {
    pub account_id: String,

    pub picture_id: String,

    #[modify(trim)]
    #[validate(length(min = "NAME_MIN", max = "NAME_MAX"), non_control_character)]
    pub name: String,

    //#[serde(skip_serializing_if = "Option::is_none")]
    #[modify(trim)]
    #[validate(length(min = "SLOGAN_MIN", max = "SLOGAN_MAX"), non_control_character)]
    pub slogan: Option<String>,

    //#[serde(skip_serializing_if = "Option::is_none")]
    #[modify(trim)]
    #[validate(length(min = "DESC_MIN", max = "DESC_MAX"))]
    pub description: Option<String>,

    pub type_of_content: StationTypeOfContent,
    pub country_code: CountryCode,

    //#[serde(skip_serializing_if = "Option::is_none")]
    #[modify(trim, lowercase)]
    #[validate(email, length(max = "EMAIL_MAX"), non_control_character)]
    pub email: Option<String>,

    //#[serde(skip_serializing_if = "Option::is_none")]
    #[modify(trim)]
    #[validate(phone, length(max = "PHONE_MAX"), non_control_character)]
    pub phone: Option<String>,

    ///#[serde(skip_serializing_if = "Option::is_none")]
    #[modify(trim)]
    #[validate(phone, length(max = "PHONE_MAX"), non_control_character)]
    pub whatsapp: Option<String>,

    //#[serde(skip_serializing_if = "Option::is_none")]
    #[modify(trim)]
    #[validate(
      url,
      regex = "WEBSITE",
      length(max = "URLS_MAX"),
      non_control_character
    )]
    pub website_url: Option<String>,

    //#[serde(skip_serializing_if = "Option::is_none")]
    #[modify(trim)]
    #[validate(
      url,
      regex = "TWITTER",
      length(max = "URLS_MAX"),
      non_control_character
    )]
    pub twitter_url: Option<String>,

    //#[serde(skip_serializing_if = "Option::is_none")]
    #[modify(trim)]
    #[validate(
      url,
      regex = "FACEBOOK",
      length(max = "URLS_MAX"),
      non_control_character
    )]
    pub facebook_url: Option<String>,

    //#[serde(skip_serializing_if = "Option::is_none")]
    #[modify(trim)]
    #[validate(
      url,
      regex = "INSTAGRAM",
      length(max = "URLS_MAX"),
      non_control_character
    )]
    pub instagram_url: Option<String>,

    //#[serde(skip_serializing_if = "Option::is_none")]
    #[modify(trim)]
    #[validate(
      url,
      regex = "THREADS",
      length(max = "URLS_MAX"),
      non_control_character
    )]
    pub threads_url: Option<String>,

    //#[serde(skip_serializing_if = "Option::is_none")]
    #[modify(trim)]
    #[validate(
      url,
      regex = "YOUTUBE",
      length(max = "URLS_MAX"),
      non_control_character
    )]
    pub youtube_url: Option<String>,

    //#[serde(skip_serializing_if = "Option::is_none")]
    #[modify(trim)]
    #[validate(url, regex = "TWITCH", length(max = "URLS_MAX"), non_control_character)]
    pub twitch_url: Option<String>,

    #[modify(trim)]
    #[validate(url, regex = "TIKTOK", length(max = "URLS_MAX"), non_control_character)]
    pub tiktok_url: Option<String>,

    //#[serde(skip_serializing_if = "Option::is_none")]
    #[modify(trim)]
    #[validate(
      url,
      regex = "GOOGLE_PLAY",
      length(max = "URLS_MAX"),
      non_control_character
    )]
    pub google_play_url: Option<String>,

    //#[serde(skip_serializing_if = "Option::is_none")]
    #[modify(trim)]
    #[validate(
      url,
      regex = "APP_STORE",
      length(max = "URLS_MAX"),
      non_control_character
    )]
    pub app_store_url: Option<String>,

    // #[serde(skip_serializing_if = "Option::is_none")]
    // #[validate]
    // pub frequencies: Option<Vec<StationFrequency>>,
    #[validate]
    pub frequency: Option<StationFrequency>,

    //#[serde(skip_serializing_if = "Option::is_none")]
    #[modify(trim)]
    #[validate(
      url,
      regex = "WEBSITE",
      length(max = "URLS_MAX"),
      non_control_character
    )]
    pub external_relay_url: Option<String>,

    #[ts(optional)]
    //#[serde(skip_serializing_if = "Option::is_none")]
    pub user_metadata: Option<Metadata>,

    #[ts(optional)]
    //#[serde(skip_serializing_if = "Option::is_none")]
    pub system_metadata: Option<Metadata>,
  }

  #[derive(Debug, Clone, Default, Serialize, Deserialize, TS)]
  #[ts(export, export_to = "../../../defs/api/stations/POST/")]
  #[serde(rename_all = "snake_case")]
  #[serde(deny_unknown_fields)]
  pub struct PayloadLimits {
    #[serde(skip_serializing_if = "Option::is_none")]
    listeners: Option<u64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    transfer: Option<u64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    storage: Option<u64>,
  }

  #[derive(Debug, Clone)]
  pub struct Input {
    access_token_scope: AccessTokenScope,
    payload: Payload,
  }

  #[derive(Debug, Clone, Serialize, Deserialize, TS)]
  #[ts(export, export_to = "../../../defs/api/stations/POST/")]
  pub struct Output {
    station: PublicStation,
  }

  #[derive(Debug, thiserror::Error)]
  pub enum ParseError {
    #[error("token: {0}")]
    Token(#[from] GetAccessTokenScopeError),
    #[error("payload: {0}")]
    Payload(#[from] ReadBodyJsonError),
  }

  impl From<ParseError> for ApiError {
    fn from(e: ParseError) -> Self {
      match e {
        ParseError::Token(e) => ApiError::from(e),
        ParseError::Payload(e) => ApiError::from(e),
      }
    }
  }

  #[derive(Debug, thiserror::Error)]
  pub enum HandleError {
    #[error("mongodb: {0}")]
    Db(#[from] mongodb::error::Error),
    #[error("token: {0}")]
    Token(#[from] GetAccessTokenScopeError),
    #[error("account not found ({0})")]
    AccountNotFound(String),
    #[error("validation error: {0}")]
    ValidationError(#[from] ValidationErrors),
    #[error("Invalid name (slug)")]
    InvalidNameSlug,
    #[error("Picture with id {0} not found")]
    PictureNotFound(String),
    #[error("Stations limit")]
    StationLimit,
  }

  impl From<HandleError> for ApiError {
    fn from(e: HandleError) -> Self {
      match e {
        HandleError::Db(e) => ApiError::from(e),
        HandleError::Token(e) => ApiError::from(e),
        HandleError::AccountNotFound(id) => ApiError::AccountNotFound(id),
        HandleError::ValidationError(e) => ApiError::PayloadInvalid(format!("{e}")),
        HandleError::PictureNotFound(id) => {
          ApiError::PayloadInvalid(format!("Picture with id {id} not found"))
        }
        HandleError::InvalidNameSlug => {
          ApiError::PayloadInvalid(String::from("Station name is invalid"))
        }
        HandleError::StationLimit => ApiError::CreateStationAccountLimit,
      }
    }
  }

  #[derive(Debug, Clone)]
  pub struct Endpoint {}

  #[async_trait]
  impl JsonHandler for Endpoint {
    type Input = Input;
    type Output = Output;
    type ParseError = ParseError;
    type HandleError = HandleError;

    async fn parse(&self, mut req: Request) -> Result<Input, ParseError> {
      let access_token_scope = request_ext::get_access_token_scope(&req).await?;
      let payload: Payload = req.read_body_json(10_000).await?;
      Ok(Input {
        access_token_scope,
        payload,
      })
    }

    async fn perform(&self, input: Input) -> Result<Output, HandleError> {
      let Input {
        access_token_scope,
        payload,
      } = input;

      //use validify::Validify;
      //let payload = Validify::validify(payload.into())?;

      let Payload {
        account_id,
        picture_id,
        name,
        slogan,
        description,

        type_of_content,
        country_code,

        email,
        phone,
        whatsapp,

        website_url,
        twitter_url,
        facebook_url,
        instagram_url,
        threads_url,
        youtube_url,
        twitch_url,
        tiktok_url,

        google_play_url,
        app_store_url,

        frequency,

        external_relay_url,

        user_metadata,
        system_metadata,
      } = payload;

      access_token_scope.grant_account_scope(&account_id).await?;

      let system_metadata = match &access_token_scope {
        AccessTokenScope::Global | AccessTokenScope::Admin(_) => {
          system_metadata.unwrap_or_default()
        }

        AccessTokenScope::User(_) => Default::default(),
      };

      let user_metadata = user_metadata.unwrap_or_default();

      let slug = slugify::slugify(&name, "", "-", None);
      if slug.is_empty() {
        return Err(HandleError::InvalidNameSlug);
      }

      let now = DateTime::now();

      let station = Station {
        id: Station::uid(),
        account_id: account_id.clone(),
        picture_id,

        name,
        slug,
        slogan,
        description,
        type_of_content,
        country_code,

        email,
        phone,
        whatsapp,
        website_url,

        twitter_url,
        facebook_url,
        instagram_url,
        threads_url,
        youtube_url,
        twitch_url,
        tiktok_url,

        app_store_url,
        google_play_url,

        frequency,

        source_password: Station::random_source_password(),
        playlist_is_randomly_shuffled: false,
        external_relay_url,

        owner_deployment_info: None,

        system_metadata,
        user_metadata,

        created_at: now,
        updated_at: now,
        deleted_at: None,
      };

      // we validate directly the station and not the payload
      let station: Station = Validify::validify(station.into())?;

      run_transaction!(session => {
        {
          let filter = doc!{ StationPicture::KEY_ACCOUNT_ID: &station.account_id, StationPicture::KEY_ID: &station.picture_id };
          match tx_try!(StationPicture::exists_with_session(filter, &mut session).await) {
            true => {}
            false => {
              return Err(HandleError::PictureNotFound(station.picture_id.clone()))
            }
          }
        };

        let account = match tx_try!(Account::get_by_id(&account_id).await) {
          Some(account) => account,
          None => return Err(HandleError::AccountNotFound(account_id))
        };

        if account.limits.stations.avail() == 0 {
          return Err(HandleError::StationLimit);
        }

        const LIMIT_STATION: &str = const_str::concat!(Account::KEY_LIMITS, ".", Limits::KEY_STATIONS, ".", Limit::KEY_USED);
        let account_update = doc!{ "$inc": { LIMIT_STATION: 1 } };
        tx_try!(Account::update_by_id_with_session(&account_id, account_update, &mut session).await);
        tx_try!(Station::insert_with_session(&station, &mut session).await);
      });

      let out = Output {
        station: station.into_public(access_token_scope.as_public_scope()),
      };

      Ok(out)
    }
  }
}
