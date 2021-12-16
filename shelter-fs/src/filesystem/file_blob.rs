use serde::{Deserialize, Serialize};
use shelter_block::{BlockId, BlockType, ShelterBlock};

#[derive(Debug, Serialize, Deserialize)]
pub struct FileBlob {
    pub data: Vec<u8>,
}

impl FileBlob {
    /// Create new FileBlob
    pub fn new(data: Vec<u8>) -> Self {
        Self { data }
    }
}

impl ShelterBlock for FileBlob {
    type ItemBlock = Self;

    fn get_block_id(&self) -> BlockId {
        // Not necessary for FileBlob
        unimplemented!();
    }

    fn get_block_type(&self) -> BlockType {
        BlockType::BLOB
    }
}
