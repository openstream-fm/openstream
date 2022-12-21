// use bytes::Bytes;
// use std::convert::Infallible;

// #[test_util::async_test]
// async fn upload_file() {
//   if is_ci::cached() {
//     return;
//   }

//   db::test_setup().await;

//   let file = Bytes::from_static(include_bytes!("../../../audio-5s.mp3"));
//   let item = Result::<Bytes, Infallible>::Ok(file);

//   let data = tokio_stream::iter(vec![item; 4]);

//   upload::upload_audio_file(
//     "test-account-id".into(),
//     None,
//     None,
//     "test-filename.mp3".into(),
//     data,
//   )
//   .await
//   .expect("upload error");
// }

// #[test_util::async_test]
// async fn should_reject_size_exceeded() {
//   if is_ci::cached() {
//     return;
//   }

//   db::test_setup().await;

//   let file = Bytes::from_static(include_bytes!("../../../audio-5s.mp3"));
//   let repeat = 4;

//   let item = Result::<Bytes, Infallible>::Ok(file);

//   let data = tokio_stream::iter(vec![item; repeat]);

//   let err = upload::upload_audio_file(
//     "test-account-id".into(),
//     None,
//     None,
//     "test-filename.mp3".into(),
//     data,
//   )
//   .await
//   .expect_err("upload success");

//   assert!(matches!(err, upload::UploadError::QuotaExceeded))
// }
