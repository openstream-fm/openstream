#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub(crate) struct PrettyDuration(pub u64);

impl std::fmt::Display for PrettyDuration {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    const SEC: u64 = 1_000;
    const MIN: u64 = SEC * 60;
    const HOUR: u64 = MIN * 60;
    const DAY: u64 = HOUR * 24;

    let src = self.0;

    let ms = src % SEC;
    let s = (src % MIN) / SEC;
    let m = (src % HOUR) / MIN;
    let h = (src % DAY) / HOUR;
    let d = src / DAY;

    if d != 0 {
      write!(f, "{d}d {h}h {m}m {s}s {ms}ms")
    } else if h != 0 {
      write!(f, "{h}h {m}m {s}s {ms}ms")
    } else if m != 0 {
      write!(f, "{m}m {s}s {ms}ms")
    } else if s != 0 {
      write!(f, "{s}s {ms}ms")
    } else {
      write!(f, "{ms}ms")
    }
  }
}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub(crate) struct PrettyBytes(pub u64);

impl std::fmt::Display for PrettyBytes {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    const K: u64 = 1_000;
    const M: u64 = 1_000_000;
    const G: u64 = 1_000_000_000;

    let b = self.0;

    if b < K {
      write!(f, "{b} B")
    } else if b < M {
      let k = b as f64 / K as f64;
      write!(f, "{:.2} KB", k)
    } else if b < G {
      let m = b as f64 / M as f64;
      write!(f, "{:.2} MB", m)
    } else {
      write!(f, "{:.2} GB", b as f64 / G as f64)
    }
  }
}
