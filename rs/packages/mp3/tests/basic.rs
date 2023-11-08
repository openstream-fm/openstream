#![allow(clippy::bool_assert_comparison)]

use std::fs::File;
use std::time::Duration;

#[test]
fn basic() {
  let meta = mp3::read_from_file("assets/test.mp3").expect("File error");
  let file = File::open("assets/test.mp3").unwrap();
  let decoder = simplemad::Decoder::decode(file).unwrap();
  let mut i = 0;
  let mut sum = Duration::new(0, 0);

  #[allow(clippy::explicit_counter_loop)]
  for decoding_result in decoder {
    match decoding_result {
      Err(_) => {}
      Ok(frame) => {
        if i >= meta.frames.len() {
          println!(
            "==> {} > {} {:?} {:?}",
            i,
            meta.frames.len(),
            meta.frames.last().unwrap().duration,
            frame.duration
          );
        } else {
          if meta.frames[i].sampling_freq as u32 != frame.sample_rate {
            println!(
              "[{}] [SAMPLE_RATE] {} != {}",
              i, meta.frames[i].sampling_freq, frame.sample_rate
            );
            panic!();
          }
          if meta.frames[i].bitrate as u32 * 1000 != frame.bit_rate {
            println!(
              "[{}] [BIT_RATE] {} != {}",
              i,
              meta.frames[i].bitrate as u32 * 1000,
              frame.bit_rate
            );
            panic!();
          }
          if meta.frames[i].duration.unwrap() != frame.duration {
            println!(
              "[{}] [DURATION] {:?} != {:?}",
              i, meta.frames[i].duration, frame.duration
            );
            panic!();
          }
          if meta.frames[i].position != frame.position {
            println!(
              "[{}] [POSITION] {:?} != {:?}",
              i, meta.frames[i].position, frame.position
            );
            panic!();
          }
        }
        sum += frame.duration;
      }
    }
    i += 1;
  }
  if let Some(frame) = meta.frames.first() {
    assert_eq!(frame.size, 417, "frame size");
    assert_eq!(frame.version, mp3::Version::MPEG1, "version");
    assert_eq!(frame.layer, mp3::Layer::Layer3, "layer");
    assert_eq!(frame.crc, mp3::CRC::Added, "crc");
    assert_eq!(frame.bitrate, 128, "bitrate");
    assert_eq!(frame.sampling_freq, 44100, "sampling freq");
    assert_eq!(frame.padding, false, "padding");
    assert_eq!(frame.private_bit, false, "private bit");
    assert_eq!(
      frame.chan_type,
      mp3::ChannelType::SingleChannel,
      "channel type"
    );
    assert_eq!(frame.intensity_stereo, false, "intensity stereo");
    assert_eq!(frame.ms_stereo, false, "ms stereo");
    assert_eq!(frame.copyright, mp3::Copyright::None, "copyright");
    assert_eq!(frame.status, mp3::Status::Copy, "status");
    assert_eq!(frame.emphasis, mp3::Emphasis::None, "emphasis");
  }
  assert_eq!(meta.frames.len(), 475, "number of frames");
  assert_eq!(meta.duration, Duration::new(12, 408162800), "duration");
  assert_eq!(
    meta.tag,
    Some(mp3::AudioTag {
      title: "Test of MP3 File              ".to_owned(),
      artist: "Me                            ".to_owned(),
      album: "Me                            ".to_owned(),
      year: 2006,
      comment: "test                        ".to_owned(),
      genre: mp3::Genre::Other,
    }),
    "tag"
  );
  assert_eq!(meta.duration, sum, "time check");
}
