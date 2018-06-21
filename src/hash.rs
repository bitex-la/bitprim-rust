use hex_error::HexError;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
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
        self.hash
            .iter()
            .rev()
            .map(|b| format!("{:02x}", b))
            .collect::<Vec<String>>()
            .join("")
    }

    pub fn from_hex(s: &str) -> Result<Hash, HexError> {
        if s.len() != 64 {
            return Err(HexError::BadLength(s.len()));
        }

        let bytes = s.as_bytes();
        let mut ret = [0; 32];
        for i in 0..32 {
           let hi = match bytes[2*i] {
               b @ b'0'...b'9' => (b - b'0') as u8,
               b @ b'a'...b'f' => (b - b'a' + 10) as u8,
               b @ b'A'...b'F' => (b - b'A' + 10) as u8,
               b => return Err(HexError::BadCharacter(b as char))
           };
           let lo = match bytes[2*i + 1] {
               b @ b'0'...b'9' => (b - b'0') as u8,
               b @ b'a'...b'f' => (b - b'a' + 10) as u8,
               b @ b'A'...b'F' => (b - b'A' + 10) as u8,
               b => return Err(HexError::BadCharacter(b as char))
           };
           ret[31 - i] = hi * 0x10 + lo;
        }
        Ok(Hash { hash: ret })
    }
}
