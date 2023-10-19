use serde_json::{Map, Value};
use std::process::Stdio;
use tokio::io::AsyncReadExt;
use tokio::process::Command;

#[derive(Debug, thiserror::Error)]
pub enum FfprobeError {
  #[error("io: {0}")]
  Io(#[from] std::io::Error),

  #[error("json: {0}")]
  Json(#[from] serde_json::Error),

  #[error("exit status: {code}")]
  Status {
    code: i32,
    stdout: String,
    stderr: String,
  },
}

pub type Object = Map<String, Value>;

pub async fn get(url: &str) -> Result<Object, FfprobeError> {
  let mut cmd = Command::new("ffprobe");

  cmd.kill_on_drop(true);

  // verbosity
  cmd.arg("-loglevel");
  cmd.arg("error");

  cmd.arg("-show_streams");
  cmd.arg("-show_format");
  cmd.arg("-hide_banner");

  cmd.arg("-print_format");
  cmd.arg("json");

  cmd.arg(url);

  cmd.stdin(Stdio::null());
  cmd.stdout(Stdio::piped());
  cmd.stderr(Stdio::piped());

  let mut child = cmd.spawn()?;

  // let mut stdin = child.stdin.take().unwrap();
  let mut stdout = child.stdout.take().unwrap();
  let mut stderr = child.stderr.take().unwrap();

  let read_stderr = async move {
    let mut buff = vec![];
    let _ = stderr.read_to_end(&mut buff).await;
    buff
  };

  let read_stdout = async move {
    let mut buff = vec![];
    let _ = stdout.read_to_end(&mut buff).await;
    buff
  };

  let wait_status = async move { child.wait().await };

  let (stdout, stderr, status) = tokio::join!(read_stdout, read_stderr, wait_status);

  let status = status?;

  if status.success() {
    let object = serde_json::from_slice::<Object>(stdout.as_ref())?;
    Ok(object)
  } else {
    let stdout = String::from_utf8_lossy(stdout.as_ref());
    let stderr = String::from_utf8_lossy(stderr.as_ref());
    Err(FfprobeError::Status {
      code: status.code().unwrap_or(-1),
      stdout: stdout.to_string(),
      stderr: stderr.to_string(),
    })
  }
}
