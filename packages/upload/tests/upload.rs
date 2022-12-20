use bytes::Bytes;
use std::convert::Infallible;

#[tokio::test]
async fn upload_file() {
  db::test_setup().await;

  let file = Bytes::from_static(include_bytes!("../../../audio-5s.mp3"));
  let item = Result::<Bytes, Infallible>::Ok(file);

  let data = tokio_stream::iter(vec![item; 4]);

  upload::upload_audio_file(
    "test-account-id".into(),
    None,
    usize::MAX,
    "test-filename.mp3".into(),
    data,
  )
  .await
  .expect("upload error");
}
