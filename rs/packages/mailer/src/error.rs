#[derive(Debug, thiserror::Error)]
pub enum RenderError {
  #[error("askama: {0}")]
  Askama(#[from] askama::Error),
  #[error("css inline: {0}")]
  CSSInline(#[from] css_inline::InlineError),
}
