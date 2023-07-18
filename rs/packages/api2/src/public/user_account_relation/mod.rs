use super::IntoPublic;
use db::user_account_relation::UserAccountRelationKind;
use macros::pick_from;
use serde::{Deserialize, Serialize};
use serde_util::DateTime;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[pick_from(db::user_account_relation::UserAccountRelation)]
pub struct PublicUserAccountRelation {
  pub id: String,
  pub user_id: String,
  pub account_id: String,
  pub kind: UserAccountRelationKind,
  pub created_at: DateTime,
}

impl IntoPublic for db::user_account_relation::UserAccountRelation {
  type Target = PublicUserAccountRelation;
  fn into_public(self, _: &crate::auth::AccessScope) -> Self::Target {
    From::from(self)
  }
}
