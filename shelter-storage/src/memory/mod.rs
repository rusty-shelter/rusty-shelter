use crate::SuperBlock;
use crate::{Crypto, CryptoUtil, SecretKey, Storage};
use std::collections::HashMap;

pub struct MemoryStorage<C: Crypto + serde::Serialize> {
    super_block: SuperBlock<C>,
    block_map: HashMap<String, Vec<u8>>,
    master_key: Option<SecretKey>,
    data_key: Option<SecretKey>,
}

impl<C: Crypto> MemoryStorage<C>
where
    C: serde::de::DeserializeOwned,
{
    const SUPER_BLK_KEY: &'static str = "super_blk";

    pub fn new(crypto: C) -> Self {
        let super_block = SuperBlock::new(crypto);
        Self {
            super_block,
            block_map: HashMap::new(),
            master_key: None, // used to encrypt super block
            data_key: None,   // used to encrypt data block
        }
    }

    #[inline]
    fn get_data_key(&self) -> &SecretKey {
        self.data_key
            .as_ref()
            .expect("init() need to be call first")
    }

    #[inline]
    fn get_master_key(&self) -> &SecretKey {
        self.master_key
            .as_ref()
            .expect("init() need to be call first")
    }

    #[inline]
    fn save_super_block(&mut self) {
        let master_key = self.get_master_key();
        let data = self.super_block.serialize(master_key);
        self.put_block(Self::SUPER_BLK_KEY, &data);
    }

    #[inline]
    fn load_super_block(&mut self) {
        let data: Vec<u8> = self.get_block(Self::SUPER_BLK_KEY);
        let master_key = self.get_master_key();
        self.super_block = SuperBlock::deserialize(&data, master_key);
    }
}

impl<C: Crypto> Storage for MemoryStorage<C>
where
    C: serde::de::DeserializeOwned,
{
    #[inline]
    fn connect(&mut self) {
        unimplemented!();
    }

    #[inline]
    fn open(&mut self, password: &[u8]) -> Vec<u8> {
        // Load super block
        self.load_super_block();

        // Init crypto
        self.data_key = Some(self.super_block.get_data_key());
        self.master_key = Some(self.super_block.get_master_key(password));

        // Return payload
        self.super_block.body.payload.clone()
    }

    #[inline]
    fn is_init(&self) -> bool {
        false
    }

    #[inline]
    fn init(&mut self, password: &[u8], payload: &[u8]) {
        // Init super block (salt)
        self.super_block.init();

        // Init crypto
        let data_key: SecretKey = CryptoUtil::gen_secret_key();
        self.super_block.set_data_key(&data_key);
        self.data_key = Some(data_key);
        self.master_key = Some(self.super_block.get_master_key(password));

        // Save super block with payload
        self.save_payload(payload);
    }

    #[inline]
    fn save_payload(&mut self, payload: &[u8]) {
        self.super_block.set_payload(payload);
        self.save_super_block();
    }

    #[inline]
    fn put_block(&mut self, cid: &str, data: &[u8]) {
        let ciphertext = self
            .super_block
            .head
            .crypto
            .encrypt_with_key(self.get_data_key(), data);
        self.block_map.insert(cid.to_owned(), ciphertext);
    }

    #[inline]
    fn get_block(&self, cid: &str) -> Vec<u8> {
        let buf = self
            .block_map
            .get(cid)
            .expect("To get block referenced by cid");
        self.super_block
            .head
            .crypto
            .decrypt_with_key(self.get_data_key(), buf)
    }

    #[inline]
    fn del_block(&mut self, cid: &str) {
        self.block_map
            .remove(cid)
            .expect("To remove block referenced by cid");
    }

    #[inline]
    fn is_exist(&self, cid: &str) -> bool {
        self.block_map.contains_key(cid)
    }

    #[inline]
    fn flush(&mut self) {
        unimplemented!();
    }

    #[inline]
    fn destroy(&mut self) {
        unimplemented!()
    }
}
