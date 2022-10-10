use debug_print::debug_println;
use ffmpeg::{FfmpegSpawn, FfmpegConfig, Ffmpeg};
use hyper::{header::{CONTENT_LENGTH, CONTENT_TYPE}, Version, StatusCode, HeaderMap, Method};
use stream_util::{IntoTryBytesStream, IntoTryBytesStreamRated};
use tokio::{net::TcpStream, io::{AsyncWriteExt, AsyncReadExt}};
use tokio_stream::StreamExt;

use crate::{http::{ResponseHead, write_response_head, RequestHead}, headers, text_plain, content_length, error::HandlerError};

pub async fn source(mut socket: TcpStream, head: RequestHead, leading_buf: Vec<u8>, id: String) -> Result<(), HandlerError> {
    
  debug_println!("source: {} {} => id: {}", head.method, head.uri, id);

  // if not PUT is SOURCE checked in router
  let _is_put = head.method == Method::PUT; 

  let is_continue = match head.headers.get("expect") {
    None => false,
    Some(h) => h.as_bytes().eq_ignore_ascii_case(b"100-continue")
  };

  let channel = match channels::transmit(id) {
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
        version: Version::HTTP_10
      };

      write_response_head(&mut socket, response, true).await?;
      socket.write_all(body).await?;
      socket.flush().await?;
  
      return Ok(())
    }
  };

  let ff_spawn  = match Ffmpeg::with_config(FfmpegConfig::default()).spawn() {
    
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
        return Ok(())
    },
    
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

    let head = ResponseHead { version, status, headers };

    write_response_head(&mut socket, head, false).await?;
  }

  let (mut socket_read, mut socket_write) = socket.into_split();

  let _write_handle = tokio::spawn(async move {
    if leading_buf.len() != 0 {
      stdin.write_all(leading_buf.as_ref()).await?;
    };

    // TODO: implement chunked encoding 
    tokio::io::copy(&mut socket_read, &mut stdin).await.expect("io::copy to ffmpeg stdin");

    Result::<(), std::io::Error>::Ok(())
  });

  let stderr_handle = tokio::spawn(async move {
    let mut buf = vec![];
    stderr.read_to_end(&mut buf).await?;
    Result::<Vec<u8>, std::io::Error>::Ok(buf)
  });
  
  let _broadcast_handle = tokio::spawn(async move {
    let stream = stdout.into_bytes_stream(2048).rated(16 * 1024);
    tokio::pin!(stream);
    while let Some(result) = stream.next().await {
      let bytes = result?;
      // this only fails when no subscribers but that is ok
      let _ = channel.send(bytes);
    }

    Result::<(), std::io::Error>::Ok(())
  });

  let exit = child.wait().await?;

  if exit.success() {
    let body = b"Data streamed successfully";
    let version = Version::HTTP_10;
    let status = StatusCode::OK;
    let mut headers = headers!(2);
    headers.append(CONTENT_TYPE, text_plain!());
    headers.append(CONTENT_LENGTH, content_length!(body));

    let head = ResponseHead{ version, status, headers };
    
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
      }
    };

    let version = Version::HTTP_10;
    let status = StatusCode::BAD_REQUEST;
    let mut headers = headers!(2);
    headers.append(CONTENT_TYPE, text_plain!());
    headers.append(CONTENT_LENGTH, content_length!(body.as_bytes()));
    
    let head = ResponseHead { version, status, headers };

    write_response_head(&mut socket_write, head, true).await?;
    socket_write.write_all(body.as_bytes()).await?;
    socket_write.flush().await?;
    Ok(())
  }
}