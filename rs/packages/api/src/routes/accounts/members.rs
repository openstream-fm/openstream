use crate::json::JsonHandler;
use crate::request_ext::{self, GetAccessTokenScopeError};

use async_trait::async_trait;
use db::Model;
use mongodb::bson::doc;
use prex::Request;
use serde::{Deserialize, Serialize};
use ts_rs::TS;

pub mod get {

  use db::{
    models::user_account_relation::{UserAccountRelation, UserAccountRelationKind},
    user::User,
  };
  use futures_util::TryStreamExt;
  use mongodb::options::FindOptions;

  use super::*;

  #[derive(Debug, Clone)]
  pub struct Endpoint {}

  #[derive(Debug, Clone)]
  pub struct Input {
    account_id: String,
  }

  #[derive(Debug, Clone, Serialize, Deserialize, TS)]
  #[ts(export)]
  #[ts(export_to = "../../../defs/api/accounts/[account]/members/GET/")]
  pub struct Member {
    #[serde(rename = "_id")]
    id: String,
    email: String,
    first_name: String,
    last_name: String,
    relation: UserAccountRelationKind,
  }

  #[derive(Debug, Clone, Serialize, Deserialize, TS)]
  #[ts(export)]
  #[ts(export_to = "../../../defs/api/accounts/[account]/members/GET/")]
  // #[serde(rename_all = "camelCase")]
  pub struct Output {
    pub members: Vec<Member>,
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

      Ok(Input {
        account_id: account.id,
      })
    }

    async fn perform(&self, input: Input) -> Result<Output, Self::HandleError> {
      let Self::Input { account_id } = input;

      let filter = doc! { UserAccountRelation::KEY_ACCOUNT_ID: &account_id };
      let sort = doc! { UserAccountRelation::KEY_CREATED_AT: 1 };
      let options = FindOptions::builder().sort(sort).build();
      let mut relations = UserAccountRelation::cl().find(filter, options).await?;

      let mut members: Vec<Member> = vec![];

      while let Some(relation) = relations.try_next().await? {
        let user = match User::get_by_id(&relation.user_id).await? {
          None => continue,
          Some(user) => user,
        };

        let member = Member {
          id: user.id,
          email: user.email,
          first_name: user.first_name,
          last_name: user.last_name,
          relation: relation.kind,
        };

        members.push(member);
      }

      Ok(Output { members })
    }
  }
}
