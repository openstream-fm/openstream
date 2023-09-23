use crate::error::ApiError;
use crate::{
  json::JsonHandler,
  request_ext::{AccessTokenScope, GetAccessTokenScopeError},
};
use async_trait::async_trait;
use db::account_invitations::{AccountInvitation, AccountInvitationState};
use db::run_transaction;
use db::Model;
use mongodb::bson::doc;
use prex::request::ReadBodyJsonError;
use prex::Request;
use serde::{Deserialize, Serialize};
use serde_util::DateTime;
use ts_rs::TS;
use validify::ValidationErrors;

use crate::request_ext::get_optional_access_token_scope;

pub mod post {

  use super::*;

  #[derive(Debug, Clone, Serialize, Deserialize, TS)]
  #[ts(export, export_to = "../../../defs/api/invitations/reject/POST/")]
  #[serde(untagged)]
  pub enum Payload {
    Unauthenticated { token: String },
    Authenticated { invitation_id: String },
  }

  #[derive(Debug)]
  pub struct Input {
    pub optional_access_token_scope: Option<AccessTokenScope>,
    pub payload: Payload,
  }

  #[derive(Debug, Clone, Serialize, Deserialize, TS)]
  #[ts(export, export_to = "../../../defs/api/invitations/reject/POST/")]
  #[serde(tag = "result", rename_all = "kebab-case")]
  pub enum Output {
    Ok,
    NotFound,
    Expired,
    AlreadyAccepted,
    AlreadyRejected,
  }

  #[derive(Debug, Clone)]
  pub struct Endpoint {}

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
        ParseError::Token(e) => e.into(),
        ParseError::Payload(e) => e.into(),
      }
    }
  }

  #[derive(Debug, thiserror::Error)]
  pub enum HandleError {
    #[error("db: {0}")]
    Db(#[from] mongodb::error::Error),
    #[error("token: {0}")]
    Token(#[from] GetAccessTokenScopeError),
    #[error("validate")]
    Validate(#[from] ValidationErrors),
  }

  impl From<HandleError> for ApiError {
    fn from(e: HandleError) -> Self {
      match e {
        HandleError::Db(e) => e.into(),
        HandleError::Token(e) => e.into(),
        HandleError::Validate(errors) => ApiError::PayloadInvalid(format!("{}", errors)),
      }
    }
  }

  #[async_trait]
  impl JsonHandler for Endpoint {
    type Input = Input;
    type Output = Output;
    type ParseError = ParseError;
    type HandleError = HandleError;

    async fn parse(&self, mut req: Request) -> Result<Input, ParseError> {
      let optional_access_token_scope = get_optional_access_token_scope(&req).await?;
      let payload = req.read_body_json(5_000).await?;
      Ok(Input {
        optional_access_token_scope,
        payload,
      })
    }

    async fn perform(&self, input: Input) -> Result<Output, HandleError> {
      let Input {
        optional_access_token_scope,
        payload,
      } = input;

      run_transaction!(session => {
        let mut invitation = match payload {
          Payload::Authenticated { ref invitation_id } => {
            let access_token_scope = match &optional_access_token_scope {
              None => return Err(GetAccessTokenScopeError::Missing.into()),
              Some(scope) => scope,
            };

            let invitation = match tx_try!(AccountInvitation::get_by_id_with_session(invitation_id, &mut session).await) {
              None => return Ok(Output::NotFound),
              Some(doc) => {
                if doc.deleted_at.is_some() {
                  return Ok(Output::NotFound)
                }
                doc
              },
            };

            match access_token_scope {
              AccessTokenScope::Global | AccessTokenScope::Admin(_) => {}
              AccessTokenScope::User(user) => {
                if user.email != invitation.receiver_email {
                  return Ok(Output::NotFound);
                }
              },
            };

            invitation
          },

          Payload::Unauthenticated { ref token } => {
            let invitation = match tx_try!(AccountInvitation::get_by_token_with_session(token, &mut session).await) {
              None => return Ok(Output::NotFound),
              Some(invitation) => invitation,
            };

            #[allow(clippy::let_and_return)]
            invitation
          }
        };

        match &invitation.state {
          AccountInvitationState::Accepted { .. } => return Ok(Output::AlreadyAccepted),
          AccountInvitationState::Rejected { .. } => return Ok(Output::AlreadyRejected),
          AccountInvitationState::Pending => {},
        }

        if invitation.is_expired() {
          return Ok(Output::Expired)
        }

        let now = DateTime::now();

        invitation.state = AccountInvitationState::Rejected { used_at: now };

        tx_try!(AccountInvitation::replace(&invitation.id, &invitation).await);
      });

      Ok(Output::Ok)
    }
  }
}
