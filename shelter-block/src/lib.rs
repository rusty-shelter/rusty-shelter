#![forbid(unsafe_code)]
#![warn(missing_debug_implementations)]

#[macro_use]
extern crate serde_derive;
extern crate bincode;
extern crate multibase;
extern crate serde;
extern crate serde_bytes;
extern crate unsigned_varint;

mod block_address;
mod block_id;
mod block_type;
mod multihash;

use bincode::config::Options;
pub use block_address::BlockAddress;
pub use block_id::BlockId;
pub use block_type::BlockType;
use multibase::Base;
use multihash::{Blacke3, MultiHash};
use serde::{Deserialize, Serialize};

/// Stands for Shelter Block Version 1
const SIGNATURE: (char, char, char, char) = ('S', 'B', 'V', '1');

/// The shelter-block type has the following binary format :
///
/// <signature><multihash><type><content size><content>
///   - 4-byte signature: { 'S', 'B', 'V', '1' }
///   - multihash
///   - type
///   - content size (varint)
///   - content of the shelter block
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Block {
    pub signature: (char, char, char, char),
    pub mh: MultiHash,
    pub block_type: BlockType,
    #[serde(with = "serde_bytes")]
    pub data: Vec<u8>,
}

impl Block {
    pub fn new(block_type: BlockType, data: Vec<u8>) -> Self {
        let mh = Blacke3::digest(&data);
        Self {
            signature: SIGNATURE,
            mh,
            block_type,
            data,
        }
    }

    pub fn get_block_address(&self) -> String {
        multibase::encode(Base::Base58Btc, &self.mh.digest)
    }

    pub fn get_block_type(&self) -> BlockType {
        self.block_type
    }

    // TODO: improve perf ?
    pub fn get_data(&self) -> Vec<u8> {
        self.data.clone()
    }

    pub fn serialize(&self) -> Vec<u8> {
        bincode::options().serialize(self).unwrap()
    }
}

pub trait ShelterBlock: Send + Sync + serde::Serialize + serde::de::DeserializeOwned {
    type ItemBlock: ShelterBlock;

    /// Get block id
    fn get_block_id(&self) -> BlockId;

    /// Get block type
    fn get_block_type(&self) -> BlockType;

    /// Get block data
    fn get_block_data(&self) -> Vec<u8> {
        bincode::options().serialize(self).unwrap()
    }

    /// Create a new Block
    fn new_block(&self) -> Block {
        let block_type = self.get_block_type();
        let data = self.get_block_data();
        let mh = Blacke3::digest(&data);
        Block {
            signature: SIGNATURE,
            mh,
            block_type,
            data,
        }
    }

    fn load_block(data: &[u8]) -> Block {
        bincode::options().deserialize(data).unwrap()
    }

    /// Deserialize vec into Self::ItemBlock
    fn load_from_vec(data: &[u8]) -> Self::ItemBlock {
        let block = Self::load_block(data);
        bincode::options().deserialize(&block.data).unwrap()
    }

    /// Serialize a block
    fn serialize(block: &Block) -> Vec<u8> {
        bincode::options().serialize(block).unwrap()
    }
}
