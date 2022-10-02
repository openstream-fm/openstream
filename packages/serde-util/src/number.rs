pub mod as_f64 {

  use serde::{Serialize, Deserialize, Serializer, Deserializer};

  pub fn serialize<S: Serializer, T: Into<f64> + Copy>(t: &T, ser: S) -> Result<S::Ok, S::Error> {
    let target: f64 = (*t).into();
    target.serialize(ser)
  }

  pub fn deserialize<'de, D: Deserializer<'de>, T: From<f64>>(de: D) -> Result<T, D::Error> {
    let f = f64::deserialize(de)?;
    Ok(f.into())
  }

  pub mod option {

    use serde::{Serialize, Deserialize, Serializer, Deserializer};

    pub fn serialize<S: Serializer, T: Into<f64> + Copy>(t: &Option<T>, ser: S) -> Result<S::Ok, S::Error> {
      match t {
        None => ().serialize(ser),
        Some(t) => {
          let target: f64 = (*t).into();
          target.serialize(ser)
        }
      }
    }
  
    pub fn deserialize<'de, D: Deserializer<'de>, T: From<f64>>(de: D) -> Result<Option<T>, D::Error> {
      let opt = Option::<f64>::deserialize(de)?;
      match opt {
        None => Ok(None),
        Some(t) => Ok(Some(t.into()))
      }
    }
  }
}