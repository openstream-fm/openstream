use pretty_env_logger::{self, env_logger::Target};

pub fn init() {
  let filters = match std::env::var("RUST_LOG") {
    Ok(v) => v,
    _ => "info".into(),
  };

  let style = std::env::var("RUST_LOG_STYLE").ok();

  let mut logger = pretty_env_logger::formatted_timed_builder();

  logger.format_indent(Some(3));

  logger.parse_filters(filters.as_str());

  if let Some(ref v) = style {
    logger.parse_write_style(v.as_str());
  }

  logger.target(Target::Stdout);

  logger.init();
}
