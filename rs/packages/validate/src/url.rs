pub mod patterns {
  use once_cell::sync::Lazy;
  use regex_static::{lazy_regex, Regex};

  pub static WEBSITE: Lazy<Regex> = lazy_regex!(r#"^https?://.+"#);
  pub static TWITTER: Lazy<Regex> = lazy_regex!(r#"^https://twitter\.com/.+"#);
  pub static FACEBOOK: Lazy<Regex> = lazy_regex!(r#"^https://www\.facebook\.com/.+"#);
  pub static INSTAGRAM: Lazy<Regex> = lazy_regex!(r#"^https://www\.instagram\.com/.+"#);
  pub static YOUTUBE: Lazy<Regex> = lazy_regex!(r#"^https://www\.youtube\.com/.+"#);
  pub static TWITCH: Lazy<Regex> = lazy_regex!(r#"^https://twitch\.tv/.+"#);
  pub static TIKTOK: Lazy<Regex> = lazy_regex!(r#"https://www\.tiktok\.com/.+"#);
  pub static GOOGLE_PLAY: Lazy<Regex> = lazy_regex!(r#"^https://play\.google\.com/.+"#);
  pub static APP_STORE: Lazy<Regex> = lazy_regex!(r#"^https://apps\.apple\.com/.+"#);
}
