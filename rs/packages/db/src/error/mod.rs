use mongodb::bson::Document;

#[derive(Debug, thiserror::Error)]
pub enum ApplyPatchError {
  #[error("patch empty")]
  PatchEmpty,
  #[error("patch invalid: {0}")]
  PatchInvalid(String),
  #[error("patch out of scope: {0}")]
  OutOfScope(String),
}

impl ApplyPatchError {
  pub fn empty() -> Self {
    Self::PatchEmpty
  }

  pub fn invalid(message: impl ToString) -> Self {
    Self::PatchInvalid(message.to_string())
  }

  pub fn out_of_scope(message: impl ToString) -> Self {
    Self::OutOfScope(message.to_string())
  }
}

#[derive(Debug, thiserror::Error)]
pub enum CheckCollectionError {
  #[error("cl::count_documents error: {0}")]
  Count(#[source] mongodb::error::Error),
  #[error("cl::find error: {0}")]
  Find(#[source] mongodb::error::Error),
  #[error("cursor::try_next error: {0}")]
  Cursor(#[source] mongodb::error::Error),
  #[error("bson::from_document encounter one or more errors")]
  Deserialize(Vec<(u64, Document, mongodb::bson::de::Error)>),
}
