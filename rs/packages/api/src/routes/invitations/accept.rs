use crate::error::ApiError;
use crate::{
  json::JsonHandler,
  request_ext::{AccessTokenScope, GetAccessTokenScopeError},
};
use async_trait::async_trait;
use db::account_invitations::{AccountInvitation, AccountInvitationState};
use db::user::User;
use db::Model;
use mongodb::bson::doc;
use prex::request::ReadBodyJsonError;
use prex::Request;
use serde::{Deserialize, Serialize};
use serde_util::DateTime;

use validify::Validify;

use db::{
  run_transaction,
  user_account_relation::{UserAccountRelation, UserAccountRelationKind},
};
use ts_rs::TS;
use validify::ValidationErrors;

use crate::request_ext::get_optional_access_token_scope;

pub mod post {

  use db::account::Account;

  use super::*;

  #[derive(Debug, Clone, Serialize, Deserialize, TS)]
  #[ts(export, export_to = "../../../defs/api/invitations/accept/POST/")]
  #[serde(untagged)]
  pub enum Payload {
    Unauthenticated(UnauthenticatedAcceptPayloadData),
    Authenticated { invitation_id: String },
  }

  #[derive(Debug, Clone, Serialize, Deserialize, TS)]
  #[ts(export, export_to = "../../../defs/api/invitations/accept/POST/")]
  pub struct UnauthenticatedAcceptPayloadData {
    pub token: String,
    pub first_name: String,
    pub last_name: String,
    pub phone: Option<String>,
    pub password: String,
  }

  #[derive(Debug)]
  pub struct Input {
    pub optional_access_token_scope: Option<AccessTokenScope>,
    pub payload: Payload,
  }

  #[derive(Debug, Clone, Serialize, Deserialize, TS)]
  #[ts(export, export_to = "../../../defs/api/invitations/accept/POST/")]
  #[serde(tag = "result", rename_all = "kebab-case")]
  pub enum Output {
    Ok,
    NotFound,
    Expired,
    AlreadyAccepted,
    AlreadyRejected,
    AlreadyInAccount,
    AccountDeleted,
    AccountNotFound,
    EmailExists,
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

      match payload {
        Payload::Authenticated { invitation_id } => {
          let access_token_scope = match optional_access_token_scope {
            None => return Err(GetAccessTokenScopeError::Missing.into()),
            Some(scope) => scope,
          };

          let user_id = match access_token_scope {
            AccessTokenScope::Global | AccessTokenScope::Admin(_) => {
              return Err(GetAccessTokenScopeError::OutOfScope.into());
            }
            AccessTokenScope::User(user) => user.id,
          };

          run_transaction!(session => {
            let user = match tx_try!(User::get_by_id_with_session(&user_id, &mut session).await) {
              None => return Err(GetAccessTokenScopeError::UserNotFound(user_id).into()),
              Some(user) => user,
            };

            let invitation = tx_try!(AccountInvitation::get_by_id_with_session(&invitation_id, &mut session).await);
            let invitation = match invitation {
              None => return Ok(Output::NotFound),
              Some(doc) => doc,
            };

            if invitation.receiver_email != user.email {
              return Ok(Output::NotFound);
            }

            match &invitation.state {
              AccountInvitationState::Pending => {},
              AccountInvitationState::Accepted { .. } => return Ok(Output::AlreadyAccepted),
              AccountInvitationState::Rejected { .. } => return Ok(Output::AlreadyRejected),
            };

            let relation_exists_filter = doc! {
              UserAccountRelation::KEY_ACCOUNT_ID: &invitation.account_id,
              UserAccountRelation::KEY_USER_ID: &user.id,
            };

            let relation_exists = tx_try!(UserAccountRelation::exists_with_session(relation_exists_filter, &mut session).await);

            if relation_exists {
              return Ok(Output::AlreadyInAccount)
            }

            let account = tx_try!(Account::get_by_id_with_session(&invitation.account_id, &mut session).await);
            match account {
              None => return Ok(Output::AccountNotFound),
              Some(_account) => {
                // TODO: add deleted_at.is_some() check when Account.deleted_at is added
              }
            }

            let now = DateTime::now();

            let user_account_relation = UserAccountRelation {
              id: UserAccountRelation::uid(),
              user_id: user.id,
              account_id: invitation.account_id.clone(),
              kind: UserAccountRelationKind::Staff,
              created_at: now,
            };

            tx_try!(UserAccountRelation::insert(&user_account_relation).await);
          });

          Ok(Output::Ok)
        }

        Payload::Unauthenticated(data) => {
          let UnauthenticatedAcceptPayloadData {
            token,
            first_name,
            last_name,
            password,
            phone,
          } = data;

          run_transaction!(session => {
            let invitation = tx_try!(AccountInvitation::get_by_token_with_session(&token, &mut session).await);
            let mut invitation = match invitation {
              Some(invitation) => invitation,
              None => return Ok(Output::NotFound),
            };

            match &invitation.state {
              AccountInvitationState::Accepted { .. } => return Ok(Output::AlreadyAccepted),
              AccountInvitationState::Rejected { .. } => return Ok(Output::AlreadyRejected),
              AccountInvitationState::Pending => {},
            }

            if invitation.is_expired() {
              return Ok(Output::Expired)
            }

            let email_exists = User::email_exists(&invitation.receiver_email).await?;
            if email_exists {
              return Ok(Output::EmailExists)
            }

            let now = DateTime::now();

            let user_id = User::uid();

            let user = User {
              id: user_id.clone(),
              email: invitation.receiver_email.clone(),
              password: Some(password.clone()),
              phone: phone.clone(),
              user_metadata: Default::default(),
              system_metadata: Default::default(),
              first_name: first_name.clone(),
              last_name: last_name.clone(),
              created_at: now,
              updated_at: now,
              language: None,
            };

            let user = User::validify(user.into())?;

            let user_account_relation = UserAccountRelation {
              id: UserAccountRelation::uid(),
              account_id: invitation.account_id.clone(),
              user_id: user_id.clone(),
              kind: UserAccountRelationKind::Staff,
              created_at: now,
            };

            invitation.state = AccountInvitationState::Accepted { used_at: now };

            tx_try!(User::insert(&user).await);
            tx_try!(UserAccountRelation::insert(&user_account_relation).await);
            tx_try!(AccountInvitation::replace(&invitation.id, &invitation).await);
          });

          Ok(Output::Ok)
        }
      }
    }
  }
}
