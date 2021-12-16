extern crate bincode;
extern crate shelter_block;
#[macro_use]
extern crate serde_derive;

use shelter_block::{Block, BlockId, BlockType, ShelterBlock};

pub use bincode::config::Options;

#[derive(Serialize, Deserialize, PartialEq, Debug)]
struct Entity {
    id: BlockId,
    x: u32,
    y: u32,
}

impl ShelterBlock for Entity {
    type ItemBlock = Self;

    fn get_block_id(&self) -> BlockId {
        self.id.clone()
    }

    fn get_block_type(&self) -> BlockType {
        BlockType::BLOB
    }
}

#[derive(Serialize, Deserialize, PartialEq, Debug)]
struct World(Vec<Entity>);

#[test]
fn main() {
    let data = vec![1, 2, 3];

    let block = Block::new(BlockType::BLOB, data);
    println!("\nblock {:x?}\n", block);

    let encoded: Vec<u8> = block.serialize();
    println!("encoded {:x?}\n", encoded);

    let world = World(vec![
        Entity {
            id: BlockId::new(),
            x: 42,
            y: 15,
        },
        Entity {
            id: BlockId::new(),
            x: 10,
            y: 20,
        },
    ]);
    let encoded_world: Vec<u8> = bincode::serialize(&world).unwrap();
    println!("encoded_world {:x?}\n", encoded_world);

    // let bytes1 = serde_cbor::to_vec(&world).unwrap();
    // println!("bytes1 {:x?}\n", bytes1);
}
