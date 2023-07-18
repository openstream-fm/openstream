use crate::auth::AccessScope;
use crate::public::IntoPublic;
use derive_more::From;
use macros::pick_from;
use serde_util::DateTime;

#[pick_from(db::account::Account)]
pub struct UserPublicAccount {
  pub id: String,
  pub name: String,
  pub limits: db::account::Limits,
  pub user_metadata: db::metadata::Metadata,
  pub created_at: DateTime,
  pub updated_at: DateTime,
  pub deleted_at: Option<DateTime>,
}

#[pick_from(db::account::Account)]
pub struct AdminPublicAccount {
  pub id: String,
  pub name: String,
  pub limits: db::account::Limits,
  pub user_metadata: db::metadata::Metadata,
  pub system_metadata: db::metadata::Metadata,
  pub created_at: DateTime,
  pub updated_at: DateTime,
  pub deleted_at: Option<DateTime>,
}

#[derive(From)]
pub enum PublicAccount {
  User(UserPublicAccount),
  Admin(AdminPublicAccount),
}

impl IntoPublic for db::account::Account {
  type Target = PublicAccount;
  fn into_public(self, scope: &AccessScope) -> PublicAccount {
    match scope {
      AccessScope::Global | AccessScope::Admin(_) => PublicAccount::Admin(From::from(self)),
      AccessScope::User(_) => PublicAccount::User(From::from(self)),
    }
  }
}
