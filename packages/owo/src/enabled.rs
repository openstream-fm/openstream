use core::sync::atomic::{AtomicU8, Ordering};

const UNSET: u8 = 0;
const ON: u8 = 1;
const OFF: u8 = 2;

static MASK: AtomicU8 = AtomicU8::new(UNSET);

pub(crate) fn should_display_colors() -> bool {
  let v = MASK.load(Ordering::Relaxed);
  if v == UNSET {
    let v = set_from_env();
    v
  } else if v == OFF {
    false
  } else {
    true
  }
}

fn set_from_env() -> bool {
  match std::env::var("NO_COLOR") {
    Ok(v) => {
      if matches!(v.as_ref(), "true" | "1") {
        MASK.store(OFF, Ordering::Relaxed);
        return false;
      }
    }
    _ => {}
  }

  MASK.store(ON, Ordering::Relaxed);
  true
}
