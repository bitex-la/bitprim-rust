#[derive(Debug)]
#[repr(C, packed)]
pub struct Hash {
  pub hash: [u8; 32usize],
}

impl Hash {
  /* For compatibility with opaque collection that needs a builder */
  pub fn new(raw: Hash) -> Hash {
    raw
  }

  pub fn to_hex(&self) -> String {
    self.hash.iter()
      .rev()
      .map(|b| format!("{:02x}", b))
      .collect::<Vec<String>>()
      .join("")
  }
}
