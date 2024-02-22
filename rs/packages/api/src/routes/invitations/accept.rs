use crate::error::ApiError;
use crate::{
  json::JsonHandler,
  request_ext::{AccessTokenScope, GetAccessTokenScopeError},
};
use async_trait::async_trait;
use constants::validate::*;
use db::account_invitations::{AccountInvitation, AccountInvitationState};
use db::user::User;
use db::Model;
use mongodb::bson::doc;
use prex::request::ReadBodyJsonError;
use prex::Request;
use serde::{Deserialize, Serialize};
use serde_util::DateTime;

use modify::Modify;
use validator::Validate;

use db::{
  run_transaction,
  user_account_relation::{UserAccountRelation, UserAccountRelationKind},
};
use ts_rs::TS;

use crate::request_ext::get_optional_access_token_scope;

pub mod post {

  use db::account::Account;
  use schemars::JsonSchema;

  use super::*;

  // TODO: add modify (works in enums?)
  #[derive(Debug, Clone, Serialize, Deserialize, TS, JsonSchema)]
  #[ts(export, export_to = "../../../defs/api/invitations/accept/POST/")]
  #[macros::schema_ts_export]
  #[serde(untagged)]
  pub enum Payload {
    Unauthenticated {
      // #[validate]
      #[serde(flatten)]
      unauthenticated: UnauthenticatedAcceptPayloadData,
    },
    Authenticated {
      invitation_id: String,
    },
  }

  #[derive(Debug, Clone, Serialize, Deserialize, TS, JsonSchema, Modify, Validate)]
  #[ts(export, export_to = "../../../defs/api/invitations/accept/POST/")]
  #[macros::schema_ts_export]
  pub struct UnauthenticatedAcceptPayloadData {
    pub token: String,

    #[modify(trim)]
    #[validate(
      length(
        min = 1,
        max = "VALIDATE_USER_FIRST_NAME_MAX_LEN",
        message = "First name is either too short or too long"
      ),
      non_control_character(message = "First name contains invalid characters")
    )]
    pub first_name: String,

    #[modify(trim)]
    #[validate(
      length(
        min = 1,
        max = "VALIDATE_USER_LAST_NAME_MAX_LEN",
        message = "Last name is either too short or too long"
      ),
      non_control_character(message = "Last name contains invalid characters")
    )]
    pub last_name: String,

    #[modify(trim)]
    #[validate(
      phone(message = "Phone number is invalid"),
      length(
        min = 1,
        max = "VALIDATE_USER_PHONE_MAX_LEN",
        message = "Phone number is either too short or too long"
      ),
      non_control_character(message = "Phone number contains invalid characters")
    )]
    pub phone: Option<String>,

    #[validate(length(
      min = "VALIDATE_USER_PASSWORD_MIN_LEN",
      max = "VALIDATE_USER_PASSWORD_MAX_LEN",
      message = "Password is either too short or too long"
    ))]
    pub password: String,
  }

  #[derive(Debug)]
  pub struct Input {
    pub optional_access_token_scope: Option<AccessTokenScope>,
    pub payload: Payload,
  }

  #[derive(Debug, Clone, Serialize, Deserialize, TS, JsonSchema)]
  #[ts(export, export_to = "../../../defs/api/invitations/accept/POST/")]
  #[macros::schema_ts_export]
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
    #[error("password too short")]
    PasswordTooShort,
    #[error("password too long")]
    PasswordTooLong,
  }

  impl From<HandleError> for ApiError {
    fn from(e: HandleError) -> Self {
      match e {
        HandleError::Db(e) => e.into(),
        HandleError::Token(e) => e.into(),
        HandleError::PasswordTooShort => {
          ApiError::PayloadInvalid(String::from("password must have 8 characters or more"))
        }
        HandleError::PasswordTooLong => {
          ApiError::PayloadInvalid(String::from("password must have 50 characters or less"))
        }
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
      let payload = req.read_body_json_no_validate(10_000).await?;
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

            let mut invitation = match tx_try!(AccountInvitation::get_by_id_with_session(&invitation_id, &mut session).await) {
              None => return Ok(Output::NotFound),
              Some(doc) => {
                if doc.deleted_at.is_some() {
                  return Ok(Output::NotFound)
                }
                doc
              },
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
              Some(account) => {
                if account.deleted_at.is_some() {
                  return Ok(Output::AccountNotFound)
                }
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

            invitation.state = AccountInvitationState::Accepted { used_at: now };
            tx_try!(AccountInvitation::replace_with_session(&invitation.id, &invitation, &mut session).await);
            tx_try!(UserAccountRelation::insert(&user_account_relation).await);
          });

          Ok(Output::Ok)
        }

        Payload::Unauthenticated { unauthenticated } => {
          let UnauthenticatedAcceptPayloadData {
            token,
            first_name,
            last_name,
            password,
            phone,
          } = unauthenticated;

          if password.len() < 8 {
            return Err(HandleError::PasswordTooShort);
          } else if password.len() > 50 {
            return Err(HandleError::PasswordTooLong);
          }

          run_transaction!(session => {
            let mut invitation = match tx_try!(AccountInvitation::get_by_token_with_session(&token, &mut session).await) {
              None => return Ok(Output::NotFound),
              Some(doc) => {
                if doc.deleted_at.is_some() {
                  return Ok(Output::NotFound)
                }
                doc
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

            let email_exists = User::email_exists(&invitation.receiver_email).await?;
            if email_exists {
              return Ok(Output::EmailExists)
            }

            let now = DateTime::now();

            let user_id = User::uid();

            let hash = crypt::hash(&password);

            let user = User {
              id: user_id.clone(),
              email: invitation.receiver_email.clone(),
              first_name: first_name.clone(),
              last_name: last_name.clone(),
              password: Some(hash),
              phone: phone.clone(),
              language: None,
              user_metadata: Default::default(),
              system_metadata: Default::default(),
              created_at: now,
              updated_at: now,
              deleted_at: None,
            };

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
