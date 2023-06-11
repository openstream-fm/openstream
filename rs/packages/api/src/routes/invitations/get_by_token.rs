use super::*;

/// we use POST here to not expose the token in system logs or in the database
/// if we decide to save the requests to a mongodb collection  
pub mod post {

  use super::*;

  #[derive(Debug, Clone, Serialize, Deserialize, TS)]
  #[ts(export, export_to = "../../../defs/api/invitations/get-by-token/POST/")]
  pub struct Payload {
    token: String,
  }

  #[derive(Debug)]
  pub struct Input {
    payload: Payload,
  }

  #[derive(Debug, Clone, Serialize, Deserialize, TS)]
  #[ts(export, export_to = "../../../defs/api/invitations/get-by-token/POST/")]
  #[serde(tag = "kind")]
  #[allow(clippy::large_enum_variant)]
  pub enum Output {
    #[serde(rename = "ok")]
    Ok { invitation: PublicInvitation },
    #[serde(rename = "not-found")]
    NotFound,
  }

  #[derive(Debug, Clone)]
  pub struct Endpoint {}

  #[derive(Debug, thiserror::Error)]
  pub enum ParseError {
    #[error("payload: {0}")]
    Payload(#[from] ReadBodyJsonError),
  }

  impl From<ParseError> for ApiError {
    fn from(e: ParseError) -> Self {
      match e {
        ParseError::Payload(e) => e.into(),
      }
    }
  }

  #[derive(Debug, thiserror::Error)]
  pub enum HandleError {
    #[error("db: {0}")]
    Db(#[from] mongodb::error::Error),
  }

  impl From<HandleError> for ApiError {
    fn from(e: HandleError) -> Self {
      match e {
        HandleError::Db(e) => e.into(),
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
      let payload = req.read_body_json(5_000).await?;
      Ok(Input { payload })
    }

    async fn perform(&self, input: Input) -> Result<Output, HandleError> {
      let Input { payload } = input;
      let Payload { token } = payload;

      let invitation = match AccountInvitation::get_by_token(&token).await? {
        None => return Ok(Output::NotFound),
        Some(invitation) => invitation,
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

      Ok(Output::Ok {
        invitation: populated,
      })
    }
  }
}
