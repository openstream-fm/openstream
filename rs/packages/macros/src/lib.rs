use parking_lot::Mutex;
use serde::Serialize;
use std::{collections::BTreeMap, path::Path, sync::Arc};

pub use macros_build::*;
pub use paste::paste;
pub use static_init::dynamic;

mod status;
pub use status::GetStatus;

static GLOBAL_CONST_REGISTRY: once_cell::sync::Lazy<ConstRegistry> =
  once_cell::sync::Lazy::new(ConstRegistry::new);

#[derive(Clone)]
pub struct ConstRegistry {
  map: Arc<Mutex<BTreeMap<String, serde_json::Value>>>,
}

impl ConstRegistry {
  pub fn new() -> Self {
    Self {
      map: Default::default(),
    }
  }

  pub fn global() -> Self {
    GLOBAL_CONST_REGISTRY.clone()
  }

  pub fn register<T: Serialize>(&self, name: impl Into<String>, value: T) {
    let lit = serde_json::to_value(&value).expect("failed to serialize constant as JSON");
    let mut lock = self.map.lock();
    lock.insert(name.into(), lit);
  }

  pub fn export_to_string(&self) -> String {
    let mut buf = String::from(
      "/// This file is auto generated from its Rust definition, do not edit manually\n",
    );
    let lock = self.map.lock();
    for (key, value) in lock.iter() {
      buf.push_str(&format!(
        "\n\nexport const {} = {};",
        key,
        // serde_json::Value to_string will never error
        serde_json::to_string_pretty(value).unwrap()
      ))
    }

    buf
  }

  pub fn export_to_file(&self, path: impl AsRef<Path>) -> Result<(), std::io::Error> {
    std::fs::write(path, self.export_to_string())
  }
}

impl Default for ConstRegistry {
  fn default() -> Self {
    Self::new()
  }
}

#[macro_export]
macro_rules! register_const {
  ($name:ident) => {
      $crate::paste!{
        #[::static_init::dynamic]
        static [<$name _REGISTRATION>]: () = $crate::ConstRegistry::global().register(stringify!($name), $name);
      }
  };
}
