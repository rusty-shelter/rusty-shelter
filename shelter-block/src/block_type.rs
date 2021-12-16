use std::convert::TryFrom;

/// Different type of shelter blocks
// TODO: put in private mode (Range: 0x300000 – 0x3FFFFF)
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum BlockType {
    SBLK = 0x31, // Super block
    BLOB = 0x32,
    FILE = 0x33,
    TREE = 0x34,
    FVER = 0x35, // File version
    INDX = 0x36, // Index
}

impl TryFrom<u8> for BlockType {
    type Error = String;

    fn try_from(raw: u8) -> Result<Self, Self::Error> {
        match raw {
            0x31 => Ok(Self::SBLK),
            0x32 => Ok(Self::BLOB),
            0x33 => Ok(Self::FILE),
            0x34 => Ok(Self::TREE),
            0x35 => Ok(Self::FVER),
            0x36 => Ok(Self::INDX),
            _ => Err("invalid code".to_string()),
        }
    }
}

impl From<BlockType> for u8 {
    fn from(code: BlockType) -> Self {
        code as u8
    }
}
