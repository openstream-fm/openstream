#[derive(Clone, Debug, thiserror::Error)]
#[error("{field} {message}")]
pub struct ValidationError {
  pub field: &'static str,
  pub message: String,
}
