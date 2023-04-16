#[derive(Debug, thiserror::Error)]
pub enum ServerStartError {
  #[error("hyper: {0}")]
  Hyper(#[from] hyper::Error),
  #[error("io: {0}")]
  Io(#[from] std::io::Error),
}
