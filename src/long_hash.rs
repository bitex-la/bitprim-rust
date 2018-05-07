#[repr(C, packed)]
pub struct LongHash {
    pub hash: [u8; 64usize],
}

impl LongHash {
    pub fn to_hex(&self) -> String {
        self.hash
            .iter()
            .rev()
            .map(|b| format!("{:02x}", b))
            .collect::<Vec<String>>()
            .join("")
    }
}
