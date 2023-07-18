use derive_more::From;
use macros::pick_from;
use serde_util::DateTime;

use crate::auth::AccessScope;

use super::IntoPublic;

#[derive(Debug, Clone, Eq, PartialEq)]
#[pick_from(db::user::User)]
pub struct MePublicUser {
  pub id: String,
  pub email: String,
  pub first_name: String,
  pub last_name: String,
  pub user_metadata: db::metadata::Metadata,
  pub created_at: DateTime,
  pub updated_at: DateTime,
  pub deleted_at: Option<DateTime>,
}

#[derive(Debug, Clone, Eq, PartialEq)]
#[pick_from(db::user::User)]
pub struct AdminPublicUser {
  pub id: String,
  pub email: String,
  pub first_name: String,
  pub last_name: String,
  pub user_metadata: db::metadata::Metadata,
  pub system_metadata: db::metadata::Metadata,
  pub created_at: DateTime,
  pub updated_at: DateTime,
  pub deleted_at: Option<DateTime>,
}

#[derive(From)]
pub enum PublicUser {
  Me(MePublicUser),
  Admin(AdminPublicUser),
}

impl IntoPublic for db::user::User {
  type Target = PublicUser;
  fn into_public(self, scope: &AccessScope) -> PublicUser {
    match scope {
      AccessScope::User(user) => {
        if user.id == self.id {
          PublicUser::Me(From::from(self))
        } else {
          panic!("cannot convert a User to public interface with not self user scope");
        }
      }
      AccessScope::Global | AccessScope::Admin(_) => PublicUser::Admin(From::from(self)),
    }
  }
}
