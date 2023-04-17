use hex::ToHex;
use sha2::{Digest, Sha256};

pub const COST: u32 = 8;

pub fn hash(plain: impl AsRef<[u8]>) -> String {
  bcrypt::hash(plain, COST).expect("bcrypt hash")
}

pub fn compare(plain: impl AsRef<[u8]>, hashed: impl AsRef<str>) -> bool {
  bcrypt::verify(plain, hashed.as_ref()).expect("bcrypt verify")
}

pub fn sha256(src: impl AsRef<[u8]>) -> String {
  let mut hasher = Sha256::new();
  hasher.update(src.as_ref());
  let hex = hasher.finalize();
  let hex_string: String = ToHex::encode_hex(&hex);
  hex_string
}
