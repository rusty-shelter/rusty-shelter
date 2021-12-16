use std::convert::TryFrom;

#[derive(Debug)]
pub enum MultiHashCode {
    Blake3 = 0x1e,
}

impl TryFrom<u64> for MultiHashCode {
    type Error = String;

    fn try_from(raw: u64) -> Result<Self, Self::Error> {
        match raw {
            0x1e => Ok(Self::Blake3),
            _ => Err("invalid code".to_string()),
        }
    }
}

impl From<MultiHashCode> for u64 {
    fn from(code: MultiHashCode) -> Self {
        code as u64
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct MultiHash {
    code: u32, // varint hash function code
    #[serde(with = "serde_bytes")]
    pub digest: Vec<u8>, // hash function output (digest)
}

impl MultiHash {
    /// Returns the algorithm used in this multihash.
    pub fn algorithm(&self) -> MultiHashCode {
        MultiHashCode::try_from(self.code as u64)
            .unwrap_or_else(|_| panic!("Should not occur as multihash is known to be valid"))
    }

    /// Returns the hash digest.
    pub fn digest(&self) -> &[u8] {
        &self.digest
    }
}

#[derive(Clone, Debug)]
pub struct Blacke3;

impl Blacke3 {
    pub const CODE: MultiHashCode = MultiHashCode::Blake3;

    /// Hash some input and return the blacke3 digest.
    pub fn digest(data: &[u8]) -> MultiHash {
        let digest = blake3::hash(data);
        let data: &[u8] = digest.as_bytes();

        MultiHash {
            code: Self::CODE as u32,
            digest: Vec::from(data),
        }
    }
}
