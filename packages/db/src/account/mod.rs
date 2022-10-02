
use mongodb::Collection;
use serde::{Serialize, Deserialize};

use serde_util::datetime;
use chrono::{DateTime, Utc};

use uid::{uid as _uid}; 

pub const NAME: &str = "accounts";
pub const UID_LEN: usize = 8;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all="camelCase")]
pub struct Account {
  #[serde(rename="_id")]
  id: String,
  name: String,
  email: String,
  password: Option<String>,
  #[serde(with="datetime")]
  created_at: DateTime<Utc>,
  #[serde(with="datetime")]
  updated_at: DateTime<Utc>,
}

pub fn uid() -> String {
  _uid(UID_LEN) 
}

pub fn cl() -> Collection<Account> {
  cl_as::<Account>()
}

pub fn cl_as<T>() -> Collection<T> {
  super::db().collection::<T>(NAME)
}