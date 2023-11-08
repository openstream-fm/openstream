#![allow(clippy::bool_assert_comparison)]

use std::time::Duration;

#[test]
fn truncate() {
  let meta = mp3::read_from_file("assets/trunc_test.mp3").expect("File error");
  if let Some(frame) = meta.frames.first() {
    assert_eq!(frame.size, 417);
    assert_eq!(frame.version, mp3::Version::MPEG1);
    assert_eq!(frame.layer, mp3::Layer::Layer3);
    assert_eq!(frame.crc, mp3::CRC::Added);
    assert_eq!(frame.bitrate, 128);
    assert_eq!(frame.sampling_freq, 44100);
    assert_eq!(frame.padding, false);
    assert_eq!(frame.private_bit, false);
    assert_eq!(frame.chan_type, mp3::ChannelType::SingleChannel);
    assert_eq!(frame.intensity_stereo, false);
    assert_eq!(frame.ms_stereo, false);
    assert_eq!(frame.copyright, mp3::Copyright::None);
    assert_eq!(frame.status, mp3::Status::Copy);
    assert_eq!(frame.emphasis, mp3::Emphasis::None);
  }
  assert_eq!(meta.duration, Duration::new(12, 120815872));
  assert_eq!(meta.tag, None);
}
