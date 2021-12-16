use crate::SuperBlock;
use crate::{vio, Crypto, CryptoUtil, SecretKey, Storage};
use moka::sync::Cache;
use std::fs::{remove_file, File};
use std::io::prelude::*;
use std::path::{Path, PathBuf};

pub struct FileSystem<C: Crypto + serde::Serialize> {
    base: PathBuf,
    super_block: SuperBlock<C>,
    master_key: Option<SecretKey>,
    data_key: Option<SecretKey>,
    cache: Cache<String, Vec<u8>>,
}

impl<C: Crypto> FileSystem<C>
where
    C: serde::de::DeserializeOwned,
{
    // super block file name
    const SUPER_BLK_FILE_NAME: &'static str = "super_blk";

    pub fn new(base: &Path, crypto: C, cache_size: u64) -> Self {
        let super_block = SuperBlock::new(crypto);
        let cache = Cache::new(cache_size);
        Self {
            base: base.to_path_buf(),
            super_block,
            master_key: None, // used to encrypt super block
            data_key: None,   // used to encrypt data block
            cache,
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
        self.put_block(Self::SUPER_BLK_FILE_NAME, &data);
    }

    #[inline]
    fn load_super_block(&mut self) {
        let data: Vec<u8> = self.get_block(Self::SUPER_BLK_FILE_NAME);
        let master_key = self.get_master_key();
        self.super_block = SuperBlock::deserialize(&data, master_key);
    }
}

impl<C: Crypto> Storage for FileSystem<C>
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

        // Check base directory

        // Return payload
        self.super_block.body.payload.clone()
    }

    #[inline]
    fn init(&mut self, password: &[u8], payload: &[u8]) {
        // Create base directory
        vio::create_dir_all(&self.base).expect("Failed to create base directory");

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
    fn is_init(&self) -> bool {
        self.is_exist(Self::SUPER_BLK_FILE_NAME)
    }

    #[inline]
    fn save_payload(&mut self, payload: &[u8]) {
        self.super_block.set_payload(payload);
        self.save_super_block();
    }

    #[inline]
    fn put_block(&mut self, cid: &str, data: &[u8]) {
        let mut file = File::create(self.base.join(cid)).unwrap();
        let ciphertext = self
            .super_block
            .head
            .crypto
            .encrypt_with_key(self.get_data_key(), data);
        file.write_all(&ciphertext).unwrap();
        // file.write_all(&data).unwrap();
        file.sync_data().unwrap();
    }

    #[inline]
    fn get_block(&self, cid: &str) -> Vec<u8> {
        let buf = self.cache.get(&cid.to_owned()).unwrap_or_else(|| {
            let mut buf = Vec::new();
            let mut file = File::open(self.base.join(cid)).unwrap();
            file.read_to_end(&mut buf).unwrap();
            buf
        });
        self.super_block
            .head
            .crypto
            .decrypt_with_key(self.get_data_key(), &buf)
    }

    #[inline]
    fn del_block(&mut self, cid: &str) {
        remove_file(cid).unwrap();
    }

    #[inline]
    fn is_exist(&self, cid: &str) -> bool {
        Path::new(cid).exists()
    }

    #[inline]
    fn flush(&mut self) {
        unimplemented!();
    }

    #[inline]
    fn destroy(&mut self) {
        vio::remove_dir(&self.base).expect("Failed to remove base directory");
    }
}
