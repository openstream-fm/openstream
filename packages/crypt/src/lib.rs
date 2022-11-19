pub const COST: u32 = 8;

pub fn hash(plain: impl AsRef<[u8]>) -> String {
  bcrypt::hash(plain, COST).expect("bcrypt hash")
}

pub fn compare(plain: impl AsRef<[u8]>, hashed: impl AsRef<str>) -> bool {
  bcrypt::verify(plain, hashed.as_ref()).expect("bcrypt verify")
}
