const SLASH: char = '/';

pub fn join(left: &str, right: &str) -> String {
  let left = left.trim_matches(SLASH);
  let right = right.trim_matches(SLASH);

  if right.is_empty() {
    format!("/{left}")
  } else if left.is_empty() {
    format!("/{right}")
  } else {
    format!("/{left}/{right}")
  }
}

pub fn normalize(path: &str) -> String {
  let mut new_path = String::new();

  if !path.starts_with(SLASH) {
    new_path.push(SLASH)
  }

  let mut add_slash_next = false;

  for ch in path.chars() {
    if ch == SLASH {
      add_slash_next = true;
      continue;
    }

    if add_slash_next {
      new_path.push(SLASH);
      add_slash_next = false;
    }

    new_path.push(ch);
  }

  new_path
}

#[cfg(test)]
mod tests {

  use super::normalize;

  #[test]
  fn normalize_remove_trailing() {
    assert_eq!(String::from("/one/two/three"), normalize("/one/two/three/"));

    assert_eq!(
      String::from("/one/two/three"),
      normalize("/one/two/three//")
    );

    assert_eq!(
      String::from("/one/two/three"),
      normalize("/one/two/three///")
    );
  }

  #[test]
  fn normalize_remove_repeated() {
    assert_eq!(
      normalize("//one//two//three"),
      String::from("/one/two/three"),
    );

    assert_eq!(
      normalize("//one///two////three"),
      String::from("/one/two/three"),
    );
  }

  #[test]
  fn normalize_prepend() {
    assert_eq!(normalize("one/two/three"), String::from("/one/two/three"),);
  }

  #[test]
  fn normalize_all() {
    assert_eq!(
      normalize("/one//two///three////"),
      String::from("/one/two/three"),
    )
  }
}
