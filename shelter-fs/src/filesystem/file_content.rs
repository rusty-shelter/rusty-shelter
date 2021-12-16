use crate::time::Time;
use serde::{Deserialize, Serialize};
use shelter_block::{BlockAddress, BlockId, BlockType, ShelterBlock};

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct FileContent {
    pub(super) id: BlockId,
    pub(super) len: usize,
    pub(super) ctime: Time,
    pub(super) block_address: Vec<BlockAddress>,
}

impl FileContent {
    /// Create new FileContent
    pub fn new() -> Self {
        Self {
            id: BlockId::new(),
            len: 0,
            ctime: Time::now(),
            block_address: Vec::<_>::new(),
        }
    }

    /// Append address
    pub(super) fn push_block_address(&mut self, address: String, len: usize) {
        let addr = BlockAddress::new(address, len, self.len);
        self.block_address.push(addr);
        self.len += len;
    }
}

impl ShelterBlock for FileContent {
    type ItemBlock = Self;

    fn get_block_id(&self) -> BlockId {
        self.id
    }

    fn get_block_type(&self) -> BlockType {
        BlockType::FVER
    }
}
