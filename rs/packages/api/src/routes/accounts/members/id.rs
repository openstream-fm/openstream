use super::*;

pub mod delete {

  use crate::error::ApiError;

  use super::*;
  use db::models::user_account_relation::UserAccountRelation;
  use serde_util::empty_struct::EmptyStruct;

  #[derive(Debug, Clone)]
  pub struct Endpoint {}

  #[derive(Debug, Clone)]
  pub struct Input {
    member_id: String,
    account_id: String,
  }

  #[derive(Debug, Clone, Serialize, Deserialize, TS)]
  #[ts(export)]
  #[ts(export_to = "../../../defs/api/accounts/[account]/members/[member]/DELETE/")]
  pub struct Output(EmptyStruct);

  #[derive(Debug, thiserror::Error)]
  pub enum HandleError {
    #[error("db: {0}")]
    Db(#[from] mongodb::error::Error),
    #[error("user not found: {0}")]
    UserNotFound(String),
  }

  impl From<HandleError> for ApiError {
    fn from(e: HandleError) -> Self {
      match e {
        HandleError::Db(e) => e.into(),
        HandleError::UserNotFound(id) => ApiError::UserNotFound(id),
      }
    }
  }

  #[async_trait]
  impl JsonHandler for Endpoint {
    type Input = Input;
    type Output = Output;
    type ParseError = GetAccessTokenScopeError;
    type HandleError = HandleError;

    async fn parse(&self, req: Request) -> Result<Self::Input, Self::ParseError> {
      let account_id = req.param("account").unwrap();
      let member_id = req.param("member").unwrap().to_string();
      let access_token_scope = request_ext::get_access_token_scope(&req).await?;
      let account = access_token_scope
        .grant_account_owner_scope(account_id)
        .await?;

      Ok(Input {
        account_id: account.id,
        member_id,
      })
    }

    async fn perform(&self, input: Input) -> Result<Output, Self::HandleError> {
      let Self::Input {
        account_id,
        member_id,
      } = input;

      let filter = doc! { UserAccountRelation::KEY_ACCOUNT_ID: &account_id, UserAccountRelation::KEY_USER_ID: &member_id };
      let r = UserAccountRelation::cl().delete_many(filter, None).await?;
      if r.deleted_count == 0 {
        return Err(HandleError::UserNotFound(member_id));
      };

      Ok(Output(EmptyStruct(())))
    }
  }
}

pub mod set_role {
  use super::*;

  pub mod post {
    use super::*;

    use crate::error::ApiError;

    use db::{
      models::user_account_relation::UserAccountRelation,
      user_account_relation::UserAccountRelationKind,
    };
    use prex::request::ReadBodyJsonError;
    use serde_util::empty_struct::EmptyStruct;

    #[derive(Debug, Clone)]
    pub struct Endpoint {}

    #[derive(Debug, Clone)]
    pub struct Input {
      member_id: String,
      account_id: String,
      payload: Payload,
    }

    #[derive(Debug, Clone, Serialize, Deserialize, TS)]
    #[ts(export)]
    #[ts(export_to = "../../../defs/api/accounts/[account]/members/[member]/set-role/POST/")]
    pub struct Payload {
      role: AccessKind,
    }

    #[derive(Debug, Clone, Serialize, Deserialize, TS)]
    #[ts(export)]
    #[ts(export_to = "../../../defs/api/accounts/[account]/members/[member]/set-role/POST/")]
    pub enum AccessKind {
      #[serde(rename = "owner")]
      Owner,
      #[serde(rename = "staff")]
      Staff,
    }

    #[derive(Debug, Clone, Serialize, Deserialize, TS)]
    #[ts(export)]
    #[ts(export_to = "../../../defs/api/accounts/[account]/members/[member]/set-role/POST/")]
    pub struct Output(EmptyStruct);

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
      #[error("user not found: {0}")]
      UserNotFound(String),
    }

    impl From<HandleError> for ApiError {
      fn from(e: HandleError) -> Self {
        match e {
          HandleError::Db(e) => e.into(),
          HandleError::UserNotFound(id) => ApiError::UserNotFound(id),
        }
      }
    }

    #[async_trait]
    impl JsonHandler for Endpoint {
      type Input = Input;
      type Output = Output;
      type ParseError = ParseError;
      type HandleError = HandleError;

      async fn parse(&self, mut req: Request) -> Result<Self::Input, Self::ParseError> {
        let account_id = req.param("account").unwrap().to_string();
        let member_id = req.param("member").unwrap().to_string();
        let payload = req.read_body_json(1_000).await?;

        let access_token_scope = request_ext::get_access_token_scope(&req).await?;

        access_token_scope
          .grant_account_owner_scope(&account_id)
          .await?;

        Ok(Input {
          account_id,
          member_id,
          payload,
        })
      }

      async fn perform(&self, input: Input) -> Result<Output, Self::HandleError> {
        let Self::Input {
          account_id,
          member_id,
          payload,
        } = input;

        let Payload { role } = payload;

        let kind_str = match role {
          AccessKind::Owner => UserAccountRelationKind::KEY_ENUM_VARIANT_OWNER,
          AccessKind::Staff => UserAccountRelationKind::KEY_ENUM_VARIANT_STAFF,
        };

        let filter = doc! { UserAccountRelation::KEY_ACCOUNT_ID: &account_id, UserAccountRelation::KEY_USER_ID: &member_id };
        let update = doc! {
          "$set": { UserAccountRelation::KEY_KIND: kind_str }
        };

        let r = UserAccountRelation::cl()
          .update_many(filter, update, None)
          .await?;

        if r.matched_count == 0 {
          return Err(HandleError::UserNotFound(member_id));
        };

        Ok(Output(EmptyStruct(())))
      }
    }
  }
}
