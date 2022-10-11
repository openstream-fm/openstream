pub mod as_f64 {

  pub trait AsF64 {
    fn as_f64(&self) -> f64;
    fn from_f64(f: f64) -> Self;
  }

  macro_rules! imp {
    ($ty:ty) => {
      impl AsF64 for $ty {
        fn as_f64(&self) -> f64 {
          (*self) as f64
        }
        fn from_f64(f: f64) -> Self {
          f as $ty
        }
      }
    };
  }

  imp!(u8);
  imp!(i8);
  imp!(u16);
  imp!(i16);
  imp!(u32);
  imp!(i32);
  imp!(u64);
  imp!(i64);
  imp!(u128);
  imp!(i128);
  imp!(usize);
  imp!(isize);
  imp!(f32);

  use serde::{Deserialize, Deserializer, Serialize, Serializer};

  pub fn serialize<S: Serializer, T: AsF64>(t: &T, ser: S) -> Result<S::Ok, S::Error> {
    let target: f64 = t.as_f64();
    target.serialize(ser)
  }

  pub fn deserialize<'de, D: Deserializer<'de>, T: AsF64>(de: D) -> Result<T, D::Error> {
    let f = f64::deserialize(de)?;
    Ok(T::from_f64(f))
  }

  pub mod option {

    use super::AsF64;

    use serde::{Deserialize, Deserializer, Serialize, Serializer};

    pub fn serialize<S: Serializer, T: AsF64>(t: &Option<T>, ser: S) -> Result<S::Ok, S::Error> {
      match t {
        None => ().serialize(ser),
        Some(t) => {
          let target: f64 = t.as_f64();
          target.serialize(ser)
        }
      }
    }

    pub fn deserialize<'de, D: Deserializer<'de>, T: AsF64>(de: D) -> Result<Option<T>, D::Error> {
      let opt = Option::<f64>::deserialize(de)?;
      match opt {
        None => Ok(None),
        Some(f) => Ok(Some(T::from_f64(f))),
      }
    }
  }
}
