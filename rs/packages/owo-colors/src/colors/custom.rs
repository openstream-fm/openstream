use crate::Color;

const U8_TO_STR: [[u8; 3]; 256] = [
  [48, 48, 48],
  [48, 48, 49],
  [48, 48, 50],
  [48, 48, 51],
  [48, 48, 52],
  [48, 48, 53],
  [48, 48, 54],
  [48, 48, 55],
  [48, 48, 56],
  [48, 48, 57],
  [48, 49, 48],
  [48, 49, 49],
  [48, 49, 50],
  [48, 49, 51],
  [48, 49, 52],
  [48, 49, 53],
  [48, 49, 54],
  [48, 49, 55],
  [48, 49, 56],
  [48, 49, 57],
  [48, 50, 48],
  [48, 50, 49],
  [48, 50, 50],
  [48, 50, 51],
  [48, 50, 52],
  [48, 50, 53],
  [48, 50, 54],
  [48, 50, 55],
  [48, 50, 56],
  [48, 50, 57],
  [48, 51, 48],
  [48, 51, 49],
  [48, 51, 50],
  [48, 51, 51],
  [48, 51, 52],
  [48, 51, 53],
  [48, 51, 54],
  [48, 51, 55],
  [48, 51, 56],
  [48, 51, 57],
  [48, 52, 48],
  [48, 52, 49],
  [48, 52, 50],
  [48, 52, 51],
  [48, 52, 52],
  [48, 52, 53],
  [48, 52, 54],
  [48, 52, 55],
  [48, 52, 56],
  [48, 52, 57],
  [48, 53, 48],
  [48, 53, 49],
  [48, 53, 50],
  [48, 53, 51],
  [48, 53, 52],
  [48, 53, 53],
  [48, 53, 54],
  [48, 53, 55],
  [48, 53, 56],
  [48, 53, 57],
  [48, 54, 48],
  [48, 54, 49],
  [48, 54, 50],
  [48, 54, 51],
  [48, 54, 52],
  [48, 54, 53],
  [48, 54, 54],
  [48, 54, 55],
  [48, 54, 56],
  [48, 54, 57],
  [48, 55, 48],
  [48, 55, 49],
  [48, 55, 50],
  [48, 55, 51],
  [48, 55, 52],
  [48, 55, 53],
  [48, 55, 54],
  [48, 55, 55],
  [48, 55, 56],
  [48, 55, 57],
  [48, 56, 48],
  [48, 56, 49],
  [48, 56, 50],
  [48, 56, 51],
  [48, 56, 52],
  [48, 56, 53],
  [48, 56, 54],
  [48, 56, 55],
  [48, 56, 56],
  [48, 56, 57],
  [48, 57, 48],
  [48, 57, 49],
  [48, 57, 50],
  [48, 57, 51],
  [48, 57, 52],
  [48, 57, 53],
  [48, 57, 54],
  [48, 57, 55],
  [48, 57, 56],
  [48, 57, 57],
  [49, 48, 48],
  [49, 48, 49],
  [49, 48, 50],
  [49, 48, 51],
  [49, 48, 52],
  [49, 48, 53],
  [49, 48, 54],
  [49, 48, 55],
  [49, 48, 56],
  [49, 48, 57],
  [49, 49, 48],
  [49, 49, 49],
  [49, 49, 50],
  [49, 49, 51],
  [49, 49, 52],
  [49, 49, 53],
  [49, 49, 54],
  [49, 49, 55],
  [49, 49, 56],
  [49, 49, 57],
  [49, 50, 48],
  [49, 50, 49],
  [49, 50, 50],
  [49, 50, 51],
  [49, 50, 52],
  [49, 50, 53],
  [49, 50, 54],
  [49, 50, 55],
  [49, 50, 56],
  [49, 50, 57],
  [49, 51, 48],
  [49, 51, 49],
  [49, 51, 50],
  [49, 51, 51],
  [49, 51, 52],
  [49, 51, 53],
  [49, 51, 54],
  [49, 51, 55],
  [49, 51, 56],
  [49, 51, 57],
  [49, 52, 48],
  [49, 52, 49],
  [49, 52, 50],
  [49, 52, 51],
  [49, 52, 52],
  [49, 52, 53],
  [49, 52, 54],
  [49, 52, 55],
  [49, 52, 56],
  [49, 52, 57],
  [49, 53, 48],
  [49, 53, 49],
  [49, 53, 50],
  [49, 53, 51],
  [49, 53, 52],
  [49, 53, 53],
  [49, 53, 54],
  [49, 53, 55],
  [49, 53, 56],
  [49, 53, 57],
  [49, 54, 48],
  [49, 54, 49],
  [49, 54, 50],
  [49, 54, 51],
  [49, 54, 52],
  [49, 54, 53],
  [49, 54, 54],
  [49, 54, 55],
  [49, 54, 56],
  [49, 54, 57],
  [49, 55, 48],
  [49, 55, 49],
  [49, 55, 50],
  [49, 55, 51],
  [49, 55, 52],
  [49, 55, 53],
  [49, 55, 54],
  [49, 55, 55],
  [49, 55, 56],
  [49, 55, 57],
  [49, 56, 48],
  [49, 56, 49],
  [49, 56, 50],
  [49, 56, 51],
  [49, 56, 52],
  [49, 56, 53],
  [49, 56, 54],
  [49, 56, 55],
  [49, 56, 56],
  [49, 56, 57],
  [49, 57, 48],
  [49, 57, 49],
  [49, 57, 50],
  [49, 57, 51],
  [49, 57, 52],
  [49, 57, 53],
  [49, 57, 54],
  [49, 57, 55],
  [49, 57, 56],
  [49, 57, 57],
  [50, 48, 48],
  [50, 48, 49],
  [50, 48, 50],
  [50, 48, 51],
  [50, 48, 52],
  [50, 48, 53],
  [50, 48, 54],
  [50, 48, 55],
  [50, 48, 56],
  [50, 48, 57],
  [50, 49, 48],
  [50, 49, 49],
  [50, 49, 50],
  [50, 49, 51],
  [50, 49, 52],
  [50, 49, 53],
  [50, 49, 54],
  [50, 49, 55],
  [50, 49, 56],
  [50, 49, 57],
  [50, 50, 48],
  [50, 50, 49],
  [50, 50, 50],
  [50, 50, 51],
  [50, 50, 52],
  [50, 50, 53],
  [50, 50, 54],
  [50, 50, 55],
  [50, 50, 56],
  [50, 50, 57],
  [50, 51, 48],
  [50, 51, 49],
  [50, 51, 50],
  [50, 51, 51],
  [50, 51, 52],
  [50, 51, 53],
  [50, 51, 54],
  [50, 51, 55],
  [50, 51, 56],
  [50, 51, 57],
  [50, 52, 48],
  [50, 52, 49],
  [50, 52, 50],
  [50, 52, 51],
  [50, 52, 52],
  [50, 52, 53],
  [50, 52, 54],
  [50, 52, 55],
  [50, 52, 56],
  [50, 52, 57],
  [50, 53, 48],
  [50, 53, 49],
  [50, 53, 50],
  [50, 53, 51],
  [50, 53, 52],
  [50, 53, 53],
];

const fn rgb_to_ansi(r: u8, g: u8, b: u8, is_fg: bool) -> [u8; 19] {
  let mut buf = if is_fg {
    *b"\x1b[38;2;rrr;ggg;bbbm"
  } else {
    *b"\x1b[48;2;rrr;ggg;bbbm"
  };

  let r = U8_TO_STR[r as usize];
  let g = U8_TO_STR[g as usize];
  let b = U8_TO_STR[b as usize];

  // r 7
  buf[7] = r[0];
  buf[8] = r[1];
  buf[9] = r[2];

  // g 11
  buf[11] = g[0];
  buf[12] = g[1];
  buf[13] = g[2];

  // b 15
  buf[15] = b[0];
  buf[16] = b[1];
  buf[17] = b[2];

  buf
}

const fn rgb_to_ansi_color(r: u8, g: u8, b: u8, is_fg: bool) -> [u8; 16] {
  let mut buf = if is_fg {
    *b"38;2;rrr;ggg;bbb"
  } else {
    *b"48;2;rrr;ggg;bbb"
  };

  let r = U8_TO_STR[r as usize];
  let g = U8_TO_STR[g as usize];
  let b = U8_TO_STR[b as usize];

  // r 5
  buf[5] = r[0];
  buf[6] = r[1];
  buf[7] = r[2];

  // g 9
  buf[9] = g[0];
  buf[10] = g[1];
  buf[11] = g[2];

  // b 13
  buf[13] = b[0];
  buf[14] = b[1];
  buf[15] = b[2];

  buf
}

/// A custom RGB color, determined at compile time
pub struct CustomColor<const R: u8, const G: u8, const B: u8>;

#[allow(clippy::transmute_bytes_to_str)]
impl<const R: u8, const G: u8, const B: u8> Color for CustomColor<R, G, B> {
  const ANSI_FG: &'static str =
    unsafe { core::mem::transmute(&rgb_to_ansi(R, G, B, true) as &[u8]) };
  const ANSI_BG: &'static str =
    unsafe { core::mem::transmute(&rgb_to_ansi(R, G, B, false) as &[u8]) };

  const RAW_ANSI_FG: &'static str =
    unsafe { core::mem::transmute(&rgb_to_ansi_color(R, G, B, true) as &[u8]) };
  const RAW_ANSI_BG: &'static str =
    unsafe { core::mem::transmute(&rgb_to_ansi_color(R, G, B, false) as &[u8]) };

  #[doc(hidden)]
  type DynEquivelant = crate::Rgb;

  #[doc(hidden)]
  const DYN_EQUIVELANT: Self::DynEquivelant = crate::Rgb(R, G, B);

  #[doc(hidden)]
  fn into_dyncolors() -> crate::DynColors {
    crate::DynColors::Rgb(R, G, B)
  }
}
