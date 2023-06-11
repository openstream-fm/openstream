use crate::Model;
use mongodb::IndexModel;
use serde::{Deserialize, Serialize};
use serde_util::DateTime;
use ts_rs::TS;

use mongodb::bson::doc;

crate::register!(PaymentMethod);

#[derive(Debug, Clone, Serialize, Deserialize, TS)]
#[ts(export, export_to = "../../../defs/db/")]
#[serde(rename_all = "snake_case")]
#[macros::keys]
pub struct PaymentMethod {
  #[serde(rename = "_id")]
  pub id: String,
  pub user_id: String,

  #[serde(flatten)]
  pub kind: PaymentMethodKind,

  pub created_at: DateTime,
  pub updated_at: DateTime,
  pub deleted_at: Option<DateTime>,
}

#[derive(Debug, Clone, Serialize, Deserialize, TS)]
#[ts(export, export_to = "../../../defs/db/")]
#[serde(tag = "kind")]
pub enum PaymentMethodKind {
  #[serde(rename = "card")]
  Card {
    token: String,
    card_type: String,
    last_4: String,
    expiration_year: Option<String>,
    expiration_month: Option<String>,
  },
}

#[derive(Debug, Clone, Serialize, Deserialize, TS)]
#[ts(export, export_to = "../../../defs/")]
pub struct PublicPaymentMethod {
  #[serde(rename = "_id")]
  pub id: String,
  pub user_id: String,

  #[serde(flatten)]
  pub kind: PublicPaymentMethodKind,

  pub created_at: DateTime,
  pub updated_at: DateTime,
  pub deleted_at: Option<DateTime>,
}

impl From<PaymentMethod> for PublicPaymentMethod {
  fn from(value: PaymentMethod) -> Self {
    let PaymentMethod {
      id,
      user_id,
      kind,
      created_at,
      updated_at,
      deleted_at,
    } = value;

    PublicPaymentMethod {
      id,
      user_id,
      kind: kind.into(),
      created_at,
      updated_at,
      deleted_at,
    }
  }
}

#[derive(Debug, Clone, Serialize, Deserialize, TS)]
#[ts(export, export_to = "../../../defs/")]
#[serde(tag = "kind")]
pub enum PublicPaymentMethodKind {
  #[serde(rename = "card")]
  Card {
    card_type: String,
    last_4: String,
    expiration_year: Option<String>,
    expiration_month: Option<String>,
  },
}

impl From<PaymentMethodKind> for PublicPaymentMethodKind {
  fn from(value: PaymentMethodKind) -> Self {
    match value {
      PaymentMethodKind::Card {
        token: _,
        card_type,
        last_4,
        expiration_year,
        expiration_month,
      } => PublicPaymentMethodKind::Card {
        card_type,
        last_4,
        expiration_year,
        expiration_month,
      },
    }
  }
}

impl Model for PaymentMethod {
  const UID_LEN: usize = 12;
  const CL_NAME: &'static str = "payment_methods";

  fn indexes() -> Vec<IndexModel> {
    let user_id = IndexModel::builder()
      .keys(doc! { Self::KEY_USER_ID: 1 })
      .build();

    let created_at = IndexModel::builder()
      .keys(doc! { Self::KEY_CREATED_AT: 1 })
      .build();

    let deleted_at = IndexModel::builder()
      .keys(doc! { Self::KEY_DELETED_AT: 1 })
      .build();

    vec![user_id, created_at, deleted_at]
  }
}
