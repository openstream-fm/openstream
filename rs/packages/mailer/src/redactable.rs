pub trait Redactable {
  fn into_redacted(self) -> Self;
}
