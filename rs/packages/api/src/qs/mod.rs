use db::{current_filter_doc, deleted_filter_doc};
use mongodb::bson::{doc, Document};
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};

// serde_as is needed because of a serde limitation on flattened types
// see https://docs.rs/serde_qs/latest/serde_qs/ too se where this workaround is described
#[serde_as]
#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash, Serialize, Deserialize, ts_rs::TS)]
#[ts(export, export_to = "../../../defs/qs/")]
pub struct PaginationQs {
  #[ts(optional)]
  #[serde(default = "PaginationQs::default_skip")]
  #[serde_as(as = "DisplayFromStr")]
  pub skip: u64,
  #[ts(optional)]
  #[serde(default = "PaginationQs::default_limit")]
  #[serde_as(as = "DisplayFromStr")]
  pub limit: i64,
}

impl Default for PaginationQs {
  fn default() -> Self {
    Self {
      skip: Self::DEFAULT_SKIP,
      limit: Self::DEFAULT_LIMIT,
    }
  }
}

impl PaginationQs {
  pub const DEFAULT_SKIP: u64 = 0;
  pub const DEFAULT_LIMIT: i64 = 60;

  const fn default_skip() -> u64 {
    Self::DEFAULT_SKIP
  }

  const fn default_limit() -> i64 {
    Self::DEFAULT_LIMIT
  }
}

#[derive(Debug, Clone, Copy, Default, Eq, PartialEq, Hash, Serialize, Deserialize, ts_rs::TS)]
#[ts(export, export_to = "../../../defs/qs/")]
pub struct VisibilityQs {
  #[ts(optional)]
  #[serde(default)]
  pub show: VisibilityKind,
}

#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash, Serialize, Deserialize, ts_rs::TS)]
#[ts(export, export_to = "../../../defs/qs/")]
#[serde(rename_all = "kebab-case")]
pub enum VisibilityKind {
  All,
  Active,
  Deleted,
}

impl VisibilityKind {
  pub fn to_filter_doc(&self) -> Document {
    match self {
      Self::All => doc! {},
      Self::Active => current_filter_doc! {},
      Self::Deleted => deleted_filter_doc! {},
    }
  }
}

impl Default for VisibilityKind {
  fn default() -> Self {
    Self::Active
  }
}
