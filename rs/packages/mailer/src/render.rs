use crate::{error::RenderError, redactable::Redactable};
use askama::Template;
use css_inline::{CSSInliner, InlineOptions};

#[derive(Debug, Clone)]
pub struct Render {
  pub sendable: RenderPart,
  pub storable: RenderPart,
}

#[derive(Debug, Clone)]
pub struct RenderPart {
  pub text: String,
  pub html: String,
}

pub fn render<T: Template + Redactable>(template: T) -> Result<Render, RenderError> {
  fn render_source(source: String) -> Result<RenderPart, RenderError> {
    let options = InlineOptions {
      base_url: None,
      extra_css: None,
      inline_style_tags: true,
      remove_style_tags: true,
      load_remote_stylesheets: false,
    };

    let inliner = CSSInliner::new(options);

    let html = inliner.inline(&source)?;
    let text = nanohtml2text::html2text(&html).trim().to_string();

    let render = RenderPart { text, html };

    Ok(render)
  }

  let sendable = render_source(template.render()?)?;
  let storable = render_source(template.into_redacted().render()?)?;

  Ok(Render { sendable, storable })
}
