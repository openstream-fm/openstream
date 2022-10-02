use std::fmt::{self, Display, Formatter};
use tokio::process::{Command, Child, ChildStderr, ChildStdin, ChildStdout};
use std::process::Stdio;

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
      Self::AAC => "aac",
      Self::OGG => "ogg",
      Self::WEBM => "webm"
    }
  }
}

impl Display for Format {
  fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
    write!(f, "{}", self.as_str())
  }
}


#[derive(Debug, Clone, PartialEq, Eq)]
#[non_exhaustive]
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
}

impl FfmpegConfig {
  pub const BIN: &'static str = "ffmpeg";
  pub const LOGLEVEL: LogLevel = LogLevel::Fatal;
  pub const KBITRATE: u16 = 128;
  pub const KMINRATE: u16 = Self::KBITRATE;
  pub const KMAXRATE: u16 = Self::KBITRATE;
  pub const KBUFSIZE: u16 = Self::KBITRATE;
  pub const FORMAT: Format = Format::MP3;
  pub const FREQ: u16 = 44100;
  pub const CHANNELS: u8 = 2;
  pub const THREADS: u8 = 1;
  pub const NOVIDEO: bool = true;
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
    }
  }
}

#[derive(Default, Debug)]
pub struct Ffmpeg {
  config: FfmpegConfig
}

impl Ffmpeg {

  pub fn new() -> Self {
    Self::default()
  }

  pub fn with_config(config: FfmpegConfig) -> Self {
    Self {
      config
    }
  }

  pub fn spawn(self) -> Result<FfmpegSpawn, std::io::Error> {

    let mut cmd = Command::new(self.config.bin);

    // input
    cmd.arg("-");

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
  pub stdout: ChildStdout
}