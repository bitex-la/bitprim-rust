#[derive(Debug)]
#[repr(C, packed)]
pub struct ShortHash {
    pub hash: [u8; 20usize],
}

impl ShortHash {
    pub fn to_hex(&self) -> String {
        self.hash
            .iter()
            .rev()
            .map(|b| format!("{:02x}", b))
            .collect::<Vec<String>>()
            .join("")
    }
}
