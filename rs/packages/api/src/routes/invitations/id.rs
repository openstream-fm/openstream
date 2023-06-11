use super::*;

/// we use POST here to not expose the token in system logs or in the database
/// if we decide to save the requests to a mongodb collection  
pub mod post {

  use super::*;

  #[derive(Debug)]
  pub struct Input {
    invitation_id: String,
    access_token_scope: AccessTokenScope,
  }

  #[derive(Debug, Clone, Serialize, Deserialize, TS)]
  #[ts(export, export_to = "../../../defs/api/invitations/[invitation]/GET/")]
  pub struct Output {
    pub invitation: PublicInvitation,
  }

  #[derive(Debug, Clone)]
  pub struct Endpoint {}

  #[derive(Debug, thiserror::Error)]
  pub enum ParseError {
    #[error("payload: {0}")]
    Token(#[from] GetAccessTokenScopeError),
  }

  impl From<ParseError> for ApiError {
    fn from(e: ParseError) -> Self {
      match e {
        ParseError::Token(e) => e.into(),
      }
    }
  }

  #[derive(Debug, thiserror::Error)]
  pub enum HandleError {
    #[error("db: {0}")]
    Db(#[from] mongodb::error::Error),
    #[error("token: {0}")]
    Token(#[from] GetAccessTokenScopeError),
    #[error("invitation not found: {0}")]
    NotFound(String),
  }

  impl From<HandleError> for ApiError {
    fn from(e: HandleError) -> Self {
      match e {
        HandleError::Db(e) => e.into(),
        HandleError::Token(e) => e.into(),
        HandleError::NotFound(id) => ApiError::InvitationNotFound(id),
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
      let invitation_id = req.param("invitation").unwrap().to_string();
      let access_token_scope = get_access_token_scope(&req).await?;
      Ok(Input {
        invitation_id,
        access_token_scope,
      })
    }

    async fn perform(&self, input: Input) -> Result<Output, HandleError> {
      let Input {
        invitation_id,
        access_token_scope,
      } = input;

      let invitation = match AccountInvitation::get_by_id(&invitation_id).await? {
        None => return Err(HandleError::NotFound(invitation_id)),
        Some(invitation) => invitation,
      };

      match &access_token_scope {
        AccessTokenScope::Global | AccessTokenScope::Admin(_) => {}
        AccessTokenScope::User(user) => {
          // we only show invitation to this user or sent from accounts that belong to this user
          if user.email != invitation.receiver_email {
            access_token_scope
              .grant_account_owner_scope(&invitation.account_id)
              .await?;
          }
        }
      };

      let account = Account::get_by_id(&invitation.account_id)
        .await?
        .map(From::from);

      let user_sender = match &invitation.user_sender_id {
        None => None,
        Some(id) => User::get_by_id(id).await?.map(From::from),
      };

      let admin_sender = match &invitation.admin_sender_id {
        None => None,
        Some(id) => Admin::get_by_id(id).await?.map(From::from),
      };

      let receiver = User::find_by_email(&invitation.receiver_email)
        .await?
        .map(From::from);

      let is_expired = invitation.is_expired();

      let populated = PublicInvitation {
        id: invitation.id,
        account_id: invitation.account_id,
        admin_sender_id: invitation.admin_sender_id,
        user_sender_id: invitation.user_sender_id,
        receiver_email: invitation.receiver_email,
        state: invitation.state,
        is_expired,
        created_at: invitation.created_at,
        account,
        admin_sender,
        user_sender,
        receiver,
      };

      Ok(Output {
        invitation: populated,
      })
    }
  }
}
