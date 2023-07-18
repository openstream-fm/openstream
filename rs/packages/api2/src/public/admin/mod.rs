use crate::auth::AccessScope;
use crate::public::IntoPublic;
use derive_more::From;
use macros::pick_from;
use serde_util::DateTime;

#[pick_from(db::admin::Admin)]
pub struct MePublicAdmin {
  pub id: String,
  pub email: String,
  pub first_name: String,
  pub last_name: String,
  pub created_at: DateTime,
  pub updated_at: DateTime,
  pub deleted_at: Option<DateTime>,
}

#[pick_from(db::admin::Admin)]
pub struct NotMePublicAdmin {
  pub id: String,
  pub email: String,
  pub first_name: String,
  pub last_name: String,
  pub created_at: DateTime,
  pub updated_at: DateTime,
  pub deleted_at: Option<DateTime>,
}

#[pick_from(db::admin::Admin)]
pub struct GlobalPublicAdmin {
  pub id: String,
  pub email: String,
  pub first_name: String,
  pub last_name: String,
  pub system_metadata: db::metadata::Metadata,
  pub created_at: DateTime,
  pub updated_at: DateTime,
  pub deleted_at: Option<DateTime>,
}

#[derive(From)]
pub enum PublicAdmin {
  Me(MePublicAdmin),
  NotMe(NotMePublicAdmin),
  Global(GlobalPublicAdmin),
}

impl IntoPublic for db::admin::Admin {
  type Target = PublicAdmin;
  fn into_public(self, scope: &AccessScope) -> PublicAdmin {
    match scope {
      AccessScope::Global => PublicAdmin::Global(From::from(self)),
      AccessScope::Admin(admin) => {
        if admin.id == self.id {
          PublicAdmin::Me(From::from(self))
        } else {
          PublicAdmin::NotMe(From::from(self))
        }
      }
      AccessScope::User(_) => {
        panic!("cannot convert an Admin to a public interface with user scope")
      }
    }
  }
}
