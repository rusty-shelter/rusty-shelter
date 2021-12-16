use std::convert::TryFrom;

/// Crypto cipher primitives.
#[derive(Default, Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub enum Cipher {
    /// XChaCha20-Poly1305-IETF
    /// - Key size: 256 bits
    /// - Nonce size: 192 bits
    /// - Block size: 512 bits
    /// - MAC size: 128 bits
    #[default]
    XChaCha20Poly1305 = 0x37,
    // AES = 0x38, // TODO[epic=feat] AES
}

impl TryFrom<u64> for Cipher {
    type Error = String;

    fn try_from(raw: u64) -> Result<Self, Self::Error> {
        match raw {
            0x37 => Ok(Self::XChaCha20Poly1305),
            // 0x38 => Ok(Self::AES),
            _ => Err("invalid code".to_string()),
        }
    }
}

impl From<Cipher> for u64 {
    fn from(code: Cipher) -> Self {
        code as u64
    }
}
