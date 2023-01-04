use bytes::Bytes;
use log::*;
use std::fmt::{self, Display, Formatter};
use std::process::{ExitStatus, Stdio};
use stream_util::IntoTryBytesStream;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::process::{Child, ChildStderr, ChildStdin, ChildStdout, Command};
use tokio::sync::mpsc;
use tokio_stream::StreamExt;

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
  pub kbitrate: usize,
  //pub kminrate: usize,
  //pub kmaxrate: usize,
  //pub kbufsize: usize,
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
  pub const KBITRATE: usize = constants::STREAM_KBITRATE;
  //pub const KMINRATE: usize = Self::KBITRATE;
  //pub const KMAXRATE: usize = Self::KBITRATE;
  //pub const KBUFSIZE: usize = Self::KBITRATE;

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
      //kminrate: Self::KMINRATE,
      //kmaxrate: Self::KMAXRATE,
      //kbufsize: Self::KBUFSIZE,
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

    cmd.kill_on_drop(true);

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
    cmd.arg(format!("{}k", self.config.kbitrate));

    cmd.arg("-maxrate");
    cmd.arg(format!("{}k", self.config.kbitrate));

    cmd.arg("-bufsize");
    cmd.arg(format!("{}k", self.config.kbitrate));

    // threads
    cmd.arg("-threads");
    cmd.arg(self.config.threads.to_string());

    // loglevel
    cmd.arg("-loglevel");
    cmd.arg(self.config.loglevel.as_str());

    // output
    cmd.arg("-");

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

#[derive(Debug)]
pub struct FfmpegSpawn {
  pub config: FfmpegConfig,
  pub child: Child,
  pub stdin: ChildStdin,
  pub stderr: ChildStderr,
  pub stdout: ChildStdout,
}

#[derive(Debug)]
pub enum TransformError {
  Io(std::io::Error),
  Exit {
    status: ExitStatus,
    stderr: Option<String>,
  },
}

impl From<std::io::Error> for TransformError {
  fn from(e: std::io::Error) -> Self {
    Self::Io(e)
  }
}

#[allow(clippy::type_complexity)]
pub fn transform(
  config: FfmpegConfig,
  chunk_size: usize,
) -> Result<
  (
    spsc::Sender<Bytes>,
    mpsc::Receiver<Result<Bytes, TransformError>>,
  ),
  std::io::Error,
> {
  let cmd = Ffmpeg::new(config);
  let spawn = cmd.spawn()?;

  let FfmpegSpawn {
    config: _,
    mut child,
    mut stdin,
    stdout,
    mut stderr,
  } = spawn;

  let (producer, receiver) = spsc::channel::<Bytes>();
  let (sender, output) = mpsc::channel(1);

  let stdin_fut = async move {
    loop {
      match receiver.recv().await {
        None => {
          trace!("transform recv end");
          break;
        }
        Some(bytes) => {
          trace!("transform recv: {} bytes", bytes.len());
          match stdin.write_all(bytes.as_ref()).await {
            Err(e) => {
              trace!("transform write err: {:?}", e);
              return Err(e.into());
            }

            Ok(()) => {
              trace!("transform write ok: {} bytes", bytes.len());
            }
          };
        }
      }
    }

    Ok(())
  };

  let stderr_fut = async move {
    let mut buf = vec![];
    stderr.read_to_end(&mut buf).await?;
    Result::<String, std::io::Error>::Ok(String::from_utf8_lossy(&buf).to_string())
  };

  let stdout_fut = {
    let sender = sender.clone();
    async move {
      let mut stream = stdout.into_bytes_stream(chunk_size);
      loop {
        match stream.try_next().await {
          Err(e) => {
            trace!("transform stdout err: {:?}", e);
            return Err(e);
          }

          Ok(None) => {
            trace!("transform stdout end");
            break;
          }

          Ok(Some(bytes)) => {
            let len = bytes.len();
            trace!("transform stdout item: {len} bytes");
            match sender.send(Ok(bytes)).await {
              Ok(()) => {
                trace!("transform stdout send received: {len} bytes");
                continue;
              }
              Err(e) => {
                trace!("transform stdout send err: {:?}", e);
                break;
              }
            }
          }
        }
      }

      Ok(())
    }
  };

  let child_fut = async move { child.wait().await };

  tokio::spawn(async move {
    let (child, stdin, stdout, stderr) = tokio::join!(child_fut, stdin_fut, stdout_fut, stderr_fut);
    let err = match stdin {
      Err(e) => Some(e),
      Ok(()) => match child {
        Err(e) => Some(TransformError::Io(e)),
        Ok(status) => {
          if !status.success() {
            Some(TransformError::Exit {
              status,
              stderr: stderr.ok(),
            })
          } else {
            match stdout {
              Err(e) => Some(TransformError::Io(e)),
              Ok(()) => None,
            }
          }
        }
      },
    };

    if let Some(err) = err {
      trace!("transform result err: {:?}", err);
      let _ = sender.send(Err(err)).await;
    } else {
      trace!("transform result ok");
    }
  });

  Ok((producer, output))
}
