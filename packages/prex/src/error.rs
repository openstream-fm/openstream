use thiserror::Error;

#[derive(Error, Debug)]
pub enum RouterBuilderError {
  #[error("Failed to compile regex for route {path:?}: {description:?}")]
  RouteRegexError { path: String, description: String },

  #[error("Paths must start with '/': {path:?}")]
  NoSlashAtStart { path: String },
}
