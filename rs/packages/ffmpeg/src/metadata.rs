use bytes::Bytes;
use std::{collections::BTreeMap, process::Stdio};
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::process::Command;
use tokio::sync::mpsc::Receiver;

pub type FfMetadata = BTreeMap<String, String>;

pub async fn get(mut data: Receiver<Bytes>) -> Result<FfMetadata, std::io::Error> {
  let mut cmd = Command::new("ffmpeg");

  cmd.kill_on_drop(true);

  // verbosity
  cmd.arg("-v");
  cmd.arg("error");

  // input
  cmd.arg("-i");
  cmd.arg("-");

  // metadata
  cmd.arg("-f");
  cmd.arg("ffmetadata");

  // output
  cmd.arg("-");

  cmd.stdin(Stdio::piped());
  cmd.stdout(Stdio::piped());
  cmd.stderr(Stdio::piped());

  let mut child = cmd.spawn()?;

  let mut stdin = child.stdin.take().unwrap();
  let mut stdout = child.stdout.take().unwrap();
  let mut stderr = child.stderr.take().unwrap();

  let write = async move {
    loop {
      match data.recv().await {
        None => break,
        Some(bytes) => match stdin.write_all(bytes.as_ref()).await {
          Err(_) => break,
          Ok(_) => continue,
        },
      }
    }
  };

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

  let (_write, stdout, stderr, status) = tokio::join!(write, read_stdout, read_stderr, wait_status);

  let status = status?;

  if status.success() {
    let stdout = String::from_utf8_lossy(stdout.as_ref());
    let mut map: BTreeMap<String, String> = BTreeMap::new();

    for line in stdout.lines() {
      if line.starts_with(';') {
        continue;
      }

      let mut split = line.splitn(2, '=');
      let name = match split.next() {
        None => continue,
        Some(name) => {
          let name = name.trim();
          if name.is_empty() {
            continue;
          } else {
            name
          }
        }
      };

      let value = match split.next() {
        None => continue,
        Some(value) => {
          let value = value.trim();
          if value.is_empty() {
            continue;
          } else {
            value
          }
        }
      };

      map.insert(name.into(), value.into());
    }

    Ok(map)
  } else {
    let stderr = String::from_utf8_lossy(stderr.as_ref());
    return Err(std::io::Error::new(
      std::io::ErrorKind::Other,
      format!("ffmpeg metadata process ended with non success status: {status}, stderr: {stderr}",),
    ));
  }
}
