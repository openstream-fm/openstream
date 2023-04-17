use std::fmt::{Display, Write};

use chrono::{Local, TimeZone};
use crossterm::tty::IsTty;
use log::Level;
use owo_colors::*;
use static_init::dynamic;
use std::io::Write as IoWrite;
use time::{macros::offset, OffsetDateTime, UtcOffset};

#[dynamic]
static OFFSET: UtcOffset = {
  let offset_secs = Local
    .timestamp_opt(0, 0)
    .unwrap()
    .offset()
    .local_minus_utc();

  UtcOffset::from_whole_seconds(offset_secs).unwrap_or(offset!(UTC))
};

pub fn now() -> OffsetDateTime {
  time::OffsetDateTime::now_utc().to_offset(*OFFSET)
}

#[derive(Debug)]
pub enum EnvStyle {
  Always,
  Never,
  Auto,
}

pub fn init() {
  let instance_id: Option<u16> = match std::env::var("INSTANCE_ID") {
    Err(_) => None,
    Ok(v) => v.parse().ok(),
  };

  let filters = match std::env::var("RUST_LOG") {
    Ok(v) => v,
    Err(_) => "info".into(),
  };

  let env_color = match std::env::var("RUST_LOG_STYLE").ok() {
    Some(v) if v.eq_ignore_ascii_case("always") => EnvStyle::Always,
    Some(v) if v.eq_ignore_ascii_case("never") => EnvStyle::Never,
    _ => EnvStyle::Auto,
  };

  let err_color = match env_color {
    EnvStyle::Always => true,
    EnvStyle::Never => false,
    EnvStyle::Auto => std::io::stderr().is_tty(),
  };

  let out_color = match env_color {
    EnvStyle::Always => true,
    EnvStyle::Never => false,
    EnvStyle::Auto => std::io::stdout().is_tty(),
  };

  if !err_color || !out_color {
    owo_colors::force_disable_colors();
  } else {
    owo_colors::force_enable_colors();
  }

  let mut logger = env_logger::builder();

  logger.parse_filters(filters.as_str());

  logger.format(move |buf, record| {
    let date = now();
    let fmt_record = FormattedRecord {
      instance_id,
      record,
      date,
    };
    writeln!(buf, "{fmt_record}")?;
    Ok(())
  });

  logger.init();
}

#[derive(Debug)]
struct FormattedRecord<'a, 'b> {
  pub instance_id: Option<u16>,
  pub record: &'a log::Record<'b>,
  pub date: time::OffsetDateTime,
}

impl<'a, 'b> FormattedRecord<'a, 'b> {
  pub fn format_date(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    let (y, m, d, hr, min, sec, ms) = (
      self.date.year(),
      self.date.month() as u8,
      self.date.day(),
      self.date.hour(),
      self.date.minute(),
      self.date.second(),
      self.date.millisecond(),
    );

    write!(f, "{y:04}-{m:02}-{d:02} {hr:02}:{min:02}:{sec:02}.{ms:03}",)?;

    if self.date.offset().is_utc() {
      write!(f, " Z")?;
    } else {
      if self.date.offset().is_negative() {
        write!(f, " -")?;
      } else {
        write!(f, " +")?;
      }

      let (h, m, s) = self.date.offset().as_hms();

      write!(f, "{:02}:{:02}", h.abs(), m.abs())?;

      if s != 0 {
        write!(f, "{:02}", s.abs())?;
      }
    }

    Ok(())
  }

  pub fn format_module_path(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(f, "{}", self.record.target().bold())
  }

  pub fn format_level(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    match self.record.level() {
      Level::Trace => write!(f, "{}", "TRACE".magenta()),
      Level::Debug => write!(f, "{}", "DEBUG".blue()),
      Level::Info => write!(f, "{}", "INFO ".green()),
      Level::Warn => write!(f, "{}", "WARN ".yellow()),
      Level::Error => write!(f, "{}", "ERROR".red()),
    }
  }

  pub fn format_args(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(f, "{}", self.record.args())
  }

  pub fn format_instance_id(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    if let Some(id) = self.instance_id {
      write!(f, "{: >2} | ", id)?;
    }

    Ok(())
  }
}

impl<'a, 'b> Display for FormattedRecord<'a, 'b> {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    self.format_date(f)?;
    f.write_char(' ')?;
    self.format_level(f)?;
    f.write_char(' ')?;
    self.format_instance_id(f)?;
    self.format_module_path(f)?;
    f.write_str(" > ")?;
    self.format_args(f)?;
    Ok(())
  }
}
