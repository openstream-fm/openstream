use askama::Template;
use css_inline::{CSSInliner, InlineError, InlineOptions};

#[derive(Debug, Clone)]
pub struct Render {
  pub html: String,
  pub text: String,
}

#[derive(Debug, thiserror::Error)]
pub enum RenderError {
  #[error("askama: {0}")]
  Askama(#[from] askama::Error),
  #[error("css inline: {0}")]
  CSSInline(#[from] InlineError),
}

pub fn render<T: Template>(template: &T) -> Result<Render, RenderError> {
  let rendered = template.render()?;

  let options = InlineOptions {
    base_url: None,
    extra_css: None,
    inline_style_tags: true,
    remove_style_tags: true,
    load_remote_stylesheets: false,
  };

  let inliner = CSSInliner::new(options);

  let html = inliner.inline(&rendered)?;
  let text = nanohtml2text::html2text(&html).trim().to_string();

  let render = Render { html, text };

  Ok(render)
}
