use constants::STREAM_CHUNK_SIZE;
use debug_print::debug_println;
use ffmpeg::{Ffmpeg, FfmpegConfig, FfmpegSpawn};
use hyper::{
  header::{CONTENT_LENGTH, CONTENT_TYPE},
  HeaderMap, Method, StatusCode, Version,
};
use tokio::{
  io::{AsyncReadExt, AsyncWriteExt},
  net::TcpStream,
};
use tokio_stream::StreamExt;

use crate::{
  content_length,
  error::HandlerError,
  headers,
  http::{write_response_head, RequestHead, ResponseHead},
  text_plain,
};

pub async fn source(
  mut socket: TcpStream,
  head: RequestHead,
  leading_buf: Vec<u8>,
  id: String,
) -> Result<(), HandlerError> {
  debug_println!("source: {} {} => id: {}", head.method, head.uri, id);

  // if not PUT is SOURCE checked in router
  let _is_put = head.method == Method::PUT;

  let is_continue = match head.headers.get("expect") {
    None => false,
    Some(h) => h.as_bytes().eq_ignore_ascii_case(b"100-continue"),
  };

  let channel = match channels::transmit(id.clone()) {
    Some(channel) => channel,
    None => {
      let body = b"This mountpoint is already in use, try again later";

      let mut headers = headers!(2);
      headers.append(CONTENT_TYPE, text_plain!());
      headers.append(CONTENT_TYPE, content_length!(body));

      // FORBIDEN (403) is used to communicate all sorts of errors
      let response = ResponseHead {
        status: StatusCode::FORBIDDEN,
        headers: headers!(),
        version: Version::HTTP_10,
      };

      write_response_head(&mut socket, response, true).await?;
      socket.write_all(body).await?;
      socket.flush().await?;

      return Ok(());
    }
  };

  let ffmpeg_config = FfmpegConfig {
    readrate: true,
    ..FfmpegConfig::default()
  };

  let ff_spawn = match Ffmpeg::with_config(ffmpeg_config).spawn() {
    Err(_) => {
      let body = b"Error allocating internal stream converter, try again later or report it to the administrators";

      let mut headers = headers!(2);
      headers.append(CONTENT_TYPE, text_plain!());
      headers.append(CONTENT_LENGTH, content_length!(body));

      let response = ResponseHead {
        version: Version::HTTP_10,
        status: StatusCode::INTERNAL_SERVER_ERROR,
        headers,
      };

      write_response_head(&mut socket, response, true).await?;
      socket.write_all(body).await?;
      socket.flush().await?;
      return Ok(());
    }

    Ok(spawn) => spawn,
  };

  let FfmpegSpawn {
    mut stderr,
    mut stdin,
    stdout,
    mut child,
    config: _,
  } = ff_spawn;

  if is_continue {
    let version = Version::HTTP_10;
    let status = StatusCode::CONTINUE;
    let headers = HeaderMap::new();

    let head = ResponseHead {
      version,
      status,
      headers,
    };

    write_response_head(&mut socket, head, true).await?;
  } else {
    let version = Version::HTTP_10;
    let status = StatusCode::OK;
    let headers = HeaderMap::new();

    let head = ResponseHead {
      version,
      status,
      headers,
    };

    write_response_head(&mut socket, head, true).await?;
  }

  let (mut socket_read, mut socket_write) = socket.into_split();

  let _write_handle = {
    let id = id.clone();

    tokio::spawn(async move {
      if leading_buf.len() != 0 {
        println!(
          "[source] channel {id} writing leading_buf to ffmpeg stdin, {} bytes",
          leading_buf.len()
        );
        stdin.write_all(leading_buf.as_ref()).await?;
      };

      let mut buf = [0u8; 2048];

      let result: Result<(), std::io::Error> = loop {
        match socket_read.read(&mut buf).await {
          Err(e) => {
            println!("[source] channel {id}: net read error: {e}");
            break Err(e);
          }

          Ok(0) => {
            println!("[source] channel {id}: net read end");
            break Ok(());
          }

          Ok(n) => {
            println!("[source] channel {id}: net read data, {n} bytes");

            match stdin.write_all(&buf[0..n]).await {
              Err(e) => {
                println!("[source] channel {id}: ffmpeg write error: {e}");
                break Err(e);
              }

              Ok(()) => {
                println!("[source] channel {id}: ffmpeg write data: {n} bytes")
              }
            }
          }
        }
      };

      result
    })
  };

  let stderr_handle = tokio::spawn(async move {
    let mut buf = vec![];
    stderr.read_to_end(&mut buf).await?;
    Result::<Vec<u8>, std::io::Error>::Ok(buf)
  });

  let _broadcast_handle = {
    use stream_util::*;

    let id = id.clone();

    tokio::spawn(async move {
      let stream = stdout.into_bytes_stream(STREAM_CHUNK_SIZE);

      tokio::pin!(stream);

      loop {
        match stream.next().await {
          None => {
            println!("[source] channel {id}: ffmpeg stdout end");
            break Ok(());
          }

          Some(Err(e)) => {
            println!("[source] channel {id}: ffmpeg stdout error: {e}");
            break Err(e);
          }

          Some(Ok(bytes)) => {
            println!(
              "[source] channel {id}: ffmpeg stdout data: {} bytes",
              bytes.len()
            );
            // this only fails when there are no subscribers but that is ok
            let _ = channel.send(bytes);
          }
        }
      }
    })
  };

  let exit = child.wait().await?;
  println!("[source] channel {id}: ffmpeg child end: exit {exit}");

  if exit.success() {
    let body = b"Data streamed successfully";
    let version = Version::HTTP_10;
    let status = StatusCode::OK;
    let mut headers = headers!(2);
    headers.append(CONTENT_TYPE, text_plain!());
    headers.append(CONTENT_LENGTH, content_length!(body));

    let head = ResponseHead {
      version,
      status,
      headers,
    };

    write_response_head(&mut socket_write, head, true).await?;
    socket_write.write_all(body).await?;
    socket_write.flush().await?;

    Ok(())
  } else {
    let body = match stderr_handle.await {
      Err(_) => format!("Internal error allocating stream converter (stderr panic)"),
      Ok(r) => match r {
        Err(_) => format!("Internal error allocating stream converter (stderr error)"),
        Ok(v) => {
          let stderr_out = String::from_utf8_lossy(v.as_ref());
          format!("Error converting the audio stream, possibly the audio is corrupted or is using a not supported format: {stderr_out}")
        }
      },
    };

    let version = Version::HTTP_10;
    let status = StatusCode::BAD_REQUEST;
    let mut headers = headers!(2);
    headers.append(CONTENT_TYPE, text_plain!());
    headers.append(CONTENT_LENGTH, content_length!(body.as_bytes()));

    let head = ResponseHead {
      version,
      status,
      headers,
    };

    write_response_head(&mut socket_write, head, true).await?;
    socket_write.write_all(body.as_bytes()).await?;
    socket_write.flush().await?;
    Ok(())
  }
}
