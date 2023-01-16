use crate::json::JsonHandler;
use crate::request_ext::{self, GetAccessTokenScopeError};

use async_trait::async_trait;
use db::account::Account;
use prex::Request;
use serde::{Deserialize, Serialize};
use ts_rs::TS;

pub mod get {

  use db::{
    audio_file::AudioFile,
    media_session::{MediaSession, MediaSessionKind},
    Model,
  };

  use super::*;

  #[derive(Debug, Clone)]
  pub struct Endpoint {}

  #[derive(Debug, Clone)]
  pub struct Input {
    account: Account,
  }

  #[derive(Debug, Clone, Serialize, Deserialize, TS)]
  #[ts(export)]
  #[ts(export_to = "../../defs/api/accounts/[account]/now-playing/GET/")]
  #[serde(tag = "kind")]
  pub enum Output {
    #[serde(rename = "none")]
    None,
    #[serde(rename = "live")]
    Live,
    #[serde(rename = "playlist")]
    Playilist { file: AudioFile },
  }

  #[async_trait]
  impl JsonHandler for Endpoint {
    type Input = Input;
    type Output = Output;
    type ParseError = GetAccessTokenScopeError;
    type HandleError = mongodb::error::Error;

    async fn parse(&self, req: Request) -> Result<Self::Input, Self::ParseError> {
      let account_id = req.param("account").unwrap();
      let access_token_scope = request_ext::get_access_token_scope(&req).await?;
      let account = access_token_scope.grant_account_scope(account_id).await?;
      Ok(Self::Input { account })
    }

    async fn perform(&self, input: Self::Input) -> Result<Self::Output, Self::HandleError> {
      let Self::Input { account } = input;

      let out = match MediaSession::get_current_for_account(&account.id).await? {
        None => Output::None,
        Some(media_session) => match media_session.kind {
          MediaSessionKind::Live { .. } => Output::Live,
          MediaSessionKind::Playlist {
            last_audio_file_id, ..
          } => match last_audio_file_id {
            None => Output::None,
            Some(file_id) => match AudioFile::get_by_id(&file_id).await? {
              None => Output::None,
              Some(file) => Output::Playilist { file },
            },
          },
        },
      };

      Ok(out)
    }
  }
}
