use db::payment_method::PaymentMethodKind;
use macros::pick_from;
use serde::{Deserialize, Serialize};
use serde_util::DateTime;

use super::IntoPublic;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
#[pick_from(db::payment_method::PaymentMethod)]
pub struct PublicPaymentMethod {
  pub id: String,
  pub user_id: String,
  #[serde(flatten)]
  pub kind: PublicPaymentMethodKind,
  pub created_at: DateTime,
  pub updated_at: DateTime,
  pub deleted_at: Option<DateTime>,
}

impl IntoPublic for db::payment_method::PaymentMethod {
  type Target = PublicPaymentMethod;
  fn into_public(self, _: &crate::auth::AccessScope) -> Self::Target {
    From::from(self)
  }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "kind", rename_all = "snake_case")]
pub enum PublicPaymentMethodKind {
  Card {
    card_type: String,
    last_4: String,
    expiration_year: Option<String>,
    expiration_month: Option<String>,
  },
}

impl From<db::payment_method::PaymentMethodKind> for PublicPaymentMethodKind {
  fn from(src: db::payment_method::PaymentMethodKind) -> Self {
    match src {
      PaymentMethodKind::Card {
        card_type,
        last_4,
        expiration_year,
        expiration_month,
        token: _,
      } => PublicPaymentMethodKind::Card {
        card_type,
        last_4,
        expiration_year,
        expiration_month,
      },
    }
  }
}

impl IntoPublic for PaymentMethodKind {
  type Target = PublicPaymentMethodKind;
  fn into_public(self, _: &crate::auth::AccessScope) -> Self::Target {
    From::from(self)
  }
}
