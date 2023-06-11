use crate::error::ApiError;
use crate::{
  json::JsonHandler,
  request_ext::{get_access_token_scope, AccessTokenScope, GetAccessTokenScopeError},
};
use async_trait::async_trait;
use db::account::Account;
use db::account_invitations::{AccountInvitation, AccountInvitationState};
use db::admin::Admin;
use db::user::User;
use db::{Model, Paged};
use mongodb::bson::doc;
use prex::request::ReadBodyJsonError;
use prex::Request;
use serde::{Deserialize, Serialize};
use serde_util::DateTime;
use ts_rs::TS;

pub mod accept;
pub mod get_by_token;
pub mod id;
pub mod reject;

#[derive(Debug, Clone, Serialize, Deserialize, TS)]
#[ts(export, export_to = "../../../defs/api/")]
pub struct PublicInvitation {
  pub id: String,
  pub user_sender_id: Option<String>,
  pub admin_sender_id: Option<String>,
  pub account_id: String,
  pub receiver_email: String,
  pub created_at: DateTime,
  #[serde(flatten)]
  pub state: AccountInvitationState,
  pub is_expired: bool,

  pub account: Option<InvitationAccount>,
  pub user_sender: Option<InvitationUserSender>,
  pub admin_sender: Option<InvitationAdminSender>,
  pub receiver: Option<InvitationReceiver>,
}

#[derive(Debug, Clone, Serialize, Deserialize, TS)]
#[ts(export, export_to = "../../../defs/api/")]
pub struct InvitationAccount {
  #[serde(rename = "_id")]
  pub id: String,
  pub name: String,
}

impl From<Account> for InvitationAccount {
  fn from(account: Account) -> Self {
    InvitationAccount {
      id: account.id,
      name: account.name,
    }
  }
}

#[derive(Debug, Clone, Serialize, Deserialize, TS)]
#[ts(export, export_to = "../../../defs/api/")]
pub struct InvitationUserSender {
  #[serde(rename = "_id")]
  id: String,
  first_name: String,
  last_name: String,
  email: String,
}

impl From<User> for InvitationUserSender {
  fn from(user: User) -> Self {
    Self {
      id: user.id,
      email: user.email,
      first_name: user.first_name,
      last_name: user.last_name,
    }
  }
}

#[derive(Debug, Clone, Serialize, Deserialize, TS)]
#[ts(export, export_to = "../../../defs/api/")]
pub struct InvitationAdminSender {
  #[serde(rename = "_id")]
  id: String,
  first_name: String,
  last_name: String,
}

impl From<Admin> for InvitationAdminSender {
  fn from(admin: Admin) -> Self {
    Self {
      id: admin.id,
      first_name: admin.first_name,
      last_name: admin.last_name,
    }
  }
}

#[derive(Debug, Clone, Serialize, Deserialize, TS)]
#[ts(export, export_to = "../../../defs/api/")]
pub struct InvitationReceiver {
  #[serde(rename = "_id")]
  id: String,
  email: String,
  first_name: String,
  last_name: String,
}

impl From<User> for InvitationReceiver {
  fn from(user: User) -> Self {
    Self {
      id: user.id,
      email: user.email,
      first_name: user.first_name,
      last_name: user.last_name,
    }
  }
}

pub mod get {

  use super::*;

  #[derive(Debug)]
  pub struct Input {
    pub query: Query,
    pub access_token_scope: AccessTokenScope,
  }

  #[derive(Debug, Clone, Serialize, Deserialize, TS)]
  #[ts(export, export_to = "../../../defs/api/invitations/GET/")]
  pub struct Query {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub skip: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_sender_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub admin_sender_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub receiver_email: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_id: Option<String>,
  }

  #[derive(Debug, Clone, Serialize, Deserialize, TS)]
  #[ts(export, export_to = "../../../defs/api/invitations/GET/")]
  pub struct Output(Paged<PublicInvitation>);

  #[derive(Debug, Clone)]
  pub struct Endpoint {}

  #[derive(Debug, thiserror::Error)]
  pub enum ParseError {
    #[error("token: {0}")]
    Token(#[from] GetAccessTokenScopeError),
    #[error("qs: {0}")]
    Query(#[from] serde_qs::Error),
  }

  impl From<ParseError> for ApiError {
    fn from(e: ParseError) -> Self {
      match e {
        ParseError::Token(e) => e.into(),
        ParseError::Query(e) => e.into(),
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
    fn from(e: HandleError) -> Self {
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

    async fn parse(&self, req: Request) -> Result<Input, ParseError> {
      let access_token_scope = get_access_token_scope(&req).await?;
      let query = req.qs()?;
      Ok(Input {
        access_token_scope,
        query,
      })
    }

    async fn perform(&self, input: Input) -> Result<Output, HandleError> {
      let Input {
        access_token_scope,
        query,
      } = input;

      let Query {
        skip,
        limit,
        account_id,
        receiver_email,
        user_sender_id,
        admin_sender_id,
      } = query;

      let skip = skip.unwrap_or(0);
      let limit = limit.unwrap_or(60);

      let sort = doc! { AccountInvitation::KEY_CREATED_AT: 1 };

      let filter = match &access_token_scope {
        AccessTokenScope::Global | AccessTokenScope::Admin(_) => {
          let mut filters = vec![];

          if let Some(user_id) = user_sender_id {
            filters.push(doc! { AccountInvitation::KEY_USER_SENDER_ID: user_id });
          }

          if let Some(admin_id) = admin_sender_id {
            filters.push(doc! { AccountInvitation::KEY_ADMIN_SENDER_ID: admin_id });
          }

          if let Some(email) = receiver_email {
            filters.push(doc! { AccountInvitation::KEY_RECEIVER_EMAIL: email });
          }

          if let Some(account_id) = account_id {
            filters.push(doc! { AccountInvitation::KEY_ACCOUNT_ID: account_id });
          }

          if filters.is_empty() {
            doc! {}
          } else {
            doc! { "$and": filters }
          }
        }

        AccessTokenScope::User(_) => {
          if let Some(account_id) = account_id {
            // only owners can see and send account invitations
            access_token_scope
              .grant_account_owner_scope(&account_id)
              .await?;

            doc! { AccountInvitation::KEY_ACCOUNT_ID: account_id }
          } else {
            // account id is required for user acccess token scope
            return Err(GetAccessTokenScopeError::OutOfScope.into());
          }
        }
      };

      let page = AccountInvitation::paged(filter, sort, skip, limit as i64).await?;

      let page = page
        .try_map_async(10, |item| async move {
          let is_expired = item.is_expired();

          let account = Account::get_by_id(&item.account_id)
            .await?
            .map(InvitationAccount::from);

          let user_sender = match &item.user_sender_id {
            None => None,
            Some(user_id) => User::get_by_id(user_id)
              .await?
              .map(InvitationUserSender::from),
          };

          let admin_sender = match &item.admin_sender_id {
            None => None,
            Some(admin_id) => Admin::get_by_id(admin_id)
              .await?
              .map(InvitationAdminSender::from),
          };

          let receiver = User::find_by_email(&item.receiver_email)
            .await?
            .map(InvitationReceiver::from);

          let target = PublicInvitation {
            id: item.id,
            account_id: item.account_id,
            receiver_email: item.receiver_email,
            user_sender_id: item.user_sender_id,
            admin_sender_id: item.admin_sender_id,
            state: item.state,
            created_at: item.created_at,
            is_expired,
            account,
            user_sender,
            admin_sender,
            receiver,
          };

          Ok::<_, mongodb::error::Error>(target)
        })
        .await?;

      Ok(Output(page))
    }
  }
}

pub mod post {

  use mailer::send::Mailer;

  use super::*;

  #[derive(Debug, Clone, Serialize, Deserialize, TS)]
  #[ts(export, export_to = "../../../defs/api/invitations/POST/")]
  pub struct Payload {
    pub account_id: String,
    pub email: String,
  }

  #[derive(Debug)]
  pub struct Input {
    pub access_token_scope: AccessTokenScope,
    pub payload: Payload,
  }

  #[derive(Debug, Clone, Serialize, Deserialize, TS)]
  #[ts(export, export_to = "../../../defs/api/invitations/POST/")]
  pub struct Output {
    pub invitation: PublicInvitation,
  }

  #[derive(Debug, Clone)]
  pub struct Endpoint {
    pub mailer: Mailer,
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
    #[error("email invalid")]
    EmailInvalid,
  }

  impl From<HandleError> for ApiError {
    fn from(e: HandleError) -> Self {
      match e {
        HandleError::Db(e) => e.into(),
        HandleError::Token(e) => e.into(),
        HandleError::EmailInvalid => ApiError::PayloadInvalid(String::from("Email is invalid")),
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
      let access_token_scope = get_access_token_scope(&req).await?;
      let payload = req.read_body_json(5_000).await?;
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

      let Payload { account_id, email } = payload;

      let email = email.trim().to_lowercase();

      if !validate::email::is_valid_email(&email) {
        return Err(HandleError::EmailInvalid);
      }

      let account = access_token_scope
        .grant_account_owner_scope(&account_id)
        .await?;

      let (admin_sender, user_sender) = match access_token_scope {
        AccessTokenScope::Global => (None, None),
        AccessTokenScope::Admin(admin) => (Some(admin), None),
        AccessTokenScope::User(user) => (None, Some(user)),
      };

      let receiver = User::find_by_email(&email).await?;

      let key = AccountInvitation::random_key();
      let hash = crypt::sha256(&key);

      let invitation = AccountInvitation {
        id: AccountInvitation::uid(),
        account_id: account.id.clone(),
        hash,
        receiver_email: email,
        admin_sender_id: admin_sender.as_ref().map(|s| &s.id).cloned(),
        user_sender_id: user_sender.as_ref().map(|s| &s.id).cloned(),
        state: AccountInvitationState::Pending,
        created_at: DateTime::now(),
      };

      AccountInvitation::insert(&invitation).await?;

      let populated = PublicInvitation {
        id: invitation.id,
        receiver_email: invitation.receiver_email,
        account_id: invitation.account_id,
        admin_sender_id: invitation.admin_sender_id,
        user_sender_id: invitation.user_sender_id,
        state: invitation.state,
        created_at: invitation.created_at,

        account: Some(account.into()),
        receiver: receiver.map(Into::into),
        admin_sender: admin_sender.map(Into::into),
        user_sender: user_sender.map(Into::into),

        is_expired: false,
      };

      Ok(Output {
        invitation: populated,
      })
    }
  }
}
