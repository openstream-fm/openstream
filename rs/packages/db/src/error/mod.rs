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
