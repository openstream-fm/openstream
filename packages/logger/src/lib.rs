use std::fmt::{Display, Write};

use chrono::{Local, TimeZone};
use log::{Level, Log};
use owo_colors::*;
use std::io::Write as IoWrite;
use time::{macros::offset, OffsetDateTime, UtcOffset};

pub fn now() -> OffsetDateTime {
  let offset_secs = Local
    .timestamp_opt(0, 0)
    .unwrap()
    .offset()
    .local_minus_utc();

  let offset = UtcOffset::from_whole_seconds(offset_secs).unwrap_or(offset!(UTC));

  time::OffsetDateTime::now_utc().to_offset(offset)
}

#[derive(Debug)]
pub enum Style {
  Always,
  Never,
}

// use pretty_env_logger::{self, env_logger::Target};
// use sensible_env_logger::LOCAL_TIME_FMT;
// use time::macros::offset;
// use time::UtcOffset;

pub fn init() {
  // Logger::default().init();
  //let logger = Logger::default().init();

  let filters = match std::env::var("RUST_LOG") {
    Ok(v) => v,
    Err(_) => "info,hyper=warn,mio=warn,tokio=warn".into(),
  };

  // let mut logger = sensible_env_logger::formatted_local_time_builder_fn(LOCAL_TIME_FMT)();

  // let mut logger = pretty_env_logger::formatted_timed_builder();

  // logger.format_indent(Some(3));

  // logger.parse_filters(filters.as_str());
  // logger.filter_module("hyper", LevelFilter::Error);
  // logger.filter_module("mio", LevelFilter::Error);

  // if let Some(ref v) = style {
  //   logger.parse_write_style(v.as_str());
  // }

  // logger.target(Target::Stdout);

  // logger.init();

  let mut logger = env_logger::builder();

  logger.parse_filters(filters.as_str());
  //logger.filter_module("hyper", LevelFilter::Error);
  //logger.filter_module("mio", LevelFilter::Error);

  logger.format(|buf, record| {
    let date = now();
    let fmt_record = FormattedRecord { record, date };
    writeln!(buf, "{}", fmt_record)?;
    Ok(())
  });

  logger.init();
}

#[derive(Debug)]
pub struct Logger {
  inner: env_logger::Logger,
}

impl Logger {
  pub fn init(self) {
    let logger = Box::leak::<'static>(Box::new(self));
    log::set_logger(logger).expect("logger already initialized");
  }
}

impl Default for Logger {
  fn default() -> Self {
    let filters = match std::env::var("RUST_LOG") {
      Ok(v) => v,
      _ => "info".into(),
    };

    //let style = std::env::var("RUST_LOG_STYLE").ok();

    let inner = env_logger::builder()
      .parse_filters(&filters)
      .filter_module("hyper", log::LevelFilter::Warn)
      .filter_module("mio", log::LevelFilter::Warn)
      .build();

    Self { inner }
  }
}

impl Log for Logger {
  fn enabled(&self, metadata: &log::Metadata) -> bool {
    self.inner.enabled(metadata)
  }

  fn flush(&self) {}

  fn log(&self, record: &log::Record) {
    if !self.inner.matches(record) {
      return;
    }

    let date = now();

    let fmt_record = FormattedRecord { record, date };

    match record.level() {
      Level::Warn | Level::Error => eprintln!("{fmt_record}"),
      Level::Info | Level::Debug | Level::Trace => println!("{fmt_record}"),
    }
  }
}

#[derive(Debug)]
struct FormattedRecord<'a, 'b> {
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

    write!(
      f,
      "{:04}-{:02}-{:02} {:02}:{:02}:{:02}.{:03}",
      y, m, d, hr, min, sec, ms,
    )?;

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
    write!(f, "{}", self.record.module_path().unwrap_or("").bold())
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
}

impl<'a, 'b> Display for FormattedRecord<'a, 'b> {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    self.format_date(f)?;
    f.write_char(' ')?;
    self.format_level(f)?;
    f.write_char(' ')?;
    self.format_module_path(f)?;
    f.write_str(" > ")?;
    self.format_args(f)?;
    Ok(())
  }
}
