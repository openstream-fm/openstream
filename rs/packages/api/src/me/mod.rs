use db::{admin::PublicAdmin, user::PublicUser};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use ts_rs::TS;

#[derive(Debug, Clone, Serialize, Deserialize, TS, JsonSchema)]
#[ts(export, export_to = "../../../defs/api/")]
#[serde(rename_all = "snake_case", tag = "scope")]
pub enum Me {
  Global,
  Admin { admin: PublicAdmin },
  User { user: PublicUser },
}
