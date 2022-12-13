#[derive(Debug)]
pub enum ApplyPatchError {
  PatchEmpty,
  PatchInvalid(String),
  OutOfScope(String),
}

impl ApplyPatchError {
  pub fn invalid(message: impl ToString) -> Self {
    Self::PatchInvalid(message.to_string())
  }

  pub fn out_of_scope(message: impl ToString) -> Self {
    Self::OutOfScope(message.to_string())
  }
}
