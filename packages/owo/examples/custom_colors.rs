use owo::colors::CustomColor;
use owo::OwoColorize;

fn main() {
  println!("{}", "custom purple".fg::<CustomColor<141, 59, 212>>());
  println!("{}", "custom green".fg_rgb::<50, 209, 42>());
}
