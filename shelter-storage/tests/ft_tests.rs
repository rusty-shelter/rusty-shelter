extern crate shelter_storage;

use shelter_storage::{MemoryStorage, Storage, XChaCha};

#[test]
fn main() {
    // 1. Configure crypto params
    // let crypto = Crypto::default();
    let crypto = XChaCha::new(256, 3);

    // 2. Create fs storage struct
    // #[encrypt(...params)]
    let mut memory_storage = MemoryStorage::new(crypto);

    // 3. Create memory storage
    memory_storage.init("sengern".as_bytes(), "payload".as_bytes());

    // 4. Write into memory storage
    let block = "my data".as_bytes();
    memory_storage.put_block("test", block);

    // 5. Get back the block
    let block2 = memory_storage.get_block("test");

    // Compare
    assert_eq!(block, &block2)
}
