use std::fmt::{self, Display, Formatter};
use std::process::Stdio;
use tokio::process::{Child, ChildStderr, ChildStdin, ChildStdout, Command};

pub mod metadata;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum LogLevel {
  Quiet,
  Panic,
  Fatal,
  Error,
  Warning,
  Info,
  Verbose,
  Debug,
  Trace,
}

impl LogLevel {
  pub fn as_str(&self) -> &'static str {
    match *self {
      Self::Quiet => "quiet",
      Self::Panic => "panic",
      Self::Fatal => "fatal",
      Self::Error => "error",
      Self::Warning => "warning",
      Self::Info => "info",
      Self::Verbose => "verbose",
      Self::Debug => "debug",
      Self::Trace => "trace",
    }
  }
}

impl Display for LogLevel {
  fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
    write!(f, "{}", self.as_str())
  }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Format {
  MP3,
  AAC,
  OGG,
  WEBM,
}

impl Format {
  pub fn as_str(&self) -> &'static str {
    match *self {
      Self::MP3 => "mp3",
      Self::AAC => "adts",
      Self::OGG => "ogg",
      Self::WEBM => "webm",
    }
  }
}

impl Display for Format {
  fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
    write!(f, "{}", self.as_str())
  }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FfmpegConfig {
  pub bin: &'static str,
  pub loglevel: LogLevel,
  pub format: Format,
  pub kbitrate: u16,
  pub kminrate: u16,
  pub kmaxrate: u16,
  pub kbufsize: u16,
  pub freq: u16,
  pub channels: u8,
  pub novideo: bool,
  pub threads: u8,
  pub readrate: bool,
  pub copycodec: bool,
}

impl FfmpegConfig {
  /// path to ffmpeg bin
  pub const BIN: &'static str = "ffmpeg";

  /// log level
  pub const LOGLEVEL: LogLevel = LogLevel::Error;

  /// output bitrate
  pub const KBITRATE: u16 = 320;
  pub const KMINRATE: u16 = Self::KBITRATE;
  pub const KMAXRATE: u16 = Self::KBITRATE;
  pub const KBUFSIZE: u16 = Self::KBITRATE;

  // output format
  pub const FORMAT: Format = Format::MP3;

  // output frequency
  pub const FREQ: u16 = 44100;

  /// output number of channels
  pub const CHANNELS: u8 = 2;

  /// number of threads to use
  pub const THREADS: u8 = 1;

  /// disable video output
  pub const NOVIDEO: bool = true;

  /// whether to read input at play rate or not
  pub const READRATE: bool = false;

  /// whether to copy as is the audio codec of the source
  pub const COPYCODEC: bool = false;
}

impl Default for FfmpegConfig {
  fn default() -> Self {
    Self {
      bin: Self::BIN,
      loglevel: Self::LOGLEVEL,
      kbitrate: Self::KBITRATE,
      kminrate: Self::KMINRATE,
      kmaxrate: Self::KMAXRATE,
      kbufsize: Self::KBUFSIZE,
      freq: Self::FREQ,
      format: Self::FORMAT,
      channels: Self::CHANNELS,
      threads: Self::THREADS,
      novideo: Self::NOVIDEO,
      readrate: Self::READRATE,
      copycodec: Self::COPYCODEC,
    }
  }
}

#[derive(Default, Debug)]
pub struct Ffmpeg {
  config: FfmpegConfig,
}

impl Ffmpeg {
  pub fn new(config: FfmpegConfig) -> Self {
    Self { config }
  }

  pub fn spawn(self) -> Result<FfmpegSpawn, std::io::Error> {
    let mut cmd = Command::new(self.config.bin);

    if self.config.readrate {
      cmd.arg("-re");
    }

    // input
    cmd.arg("-i");
    cmd.arg("-");

    // copy codec
    if self.config.copycodec {
      cmd.arg("-c:a");
      cmd.arg("copy");
    }

    // format
    cmd.arg("-f");
    cmd.arg(self.config.format.as_str());

    // no video
    if self.config.novideo {
      cmd.arg("-vn");
    }

    // channels
    cmd.arg("-ac");
    cmd.arg(self.config.channels.to_string());

    // frequency
    cmd.arg("-ar");
    cmd.arg(self.config.freq.to_string());

    // bitrate
    cmd.arg("-ab");
    cmd.arg(format!("{}k", self.config.kbitrate));

    cmd.arg("-minrate");
    cmd.arg(format!("{}k", self.config.kminrate));

    cmd.arg("-maxrate");
    cmd.arg(format!("{}k", self.config.kmaxrate));

    cmd.arg("-bufsize");
    cmd.arg(format!("{}k", self.config.kbufsize));

    // threads
    cmd.arg("-threads");
    cmd.arg(self.config.threads.to_string());

    // loglevel
    cmd.arg("-loglevel");
    cmd.arg(self.config.loglevel.as_str());

    // output
    cmd.arg("-");

    cmd.kill_on_drop(true);

    cmd.stdin(Stdio::piped());
    cmd.stderr(Stdio::piped());
    cmd.stdout(Stdio::piped());

    let mut child = cmd.spawn()?;

    let stdin = child.stdin.take().unwrap();
    let stderr = child.stderr.take().unwrap();
    let stdout = child.stdout.take().unwrap();

    Ok(FfmpegSpawn {
      config: self.config,
      child,
      stdin,
      stderr,
      stdout,
    })
  }

  pub fn config(&self) -> &FfmpegConfig {
    &self.config
  }
}

pub struct FfmpegSpawn {
  pub config: FfmpegConfig,
  pub child: Child,
  pub stdin: ChildStdin,
  pub stderr: ChildStderr,
  pub stdout: ChildStdout,
}
