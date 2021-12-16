#![forbid(unsafe_code)]
#[macro_use]
extern crate serde_derive;
extern crate bincode;
extern crate moka;
extern crate orion;
extern crate serde;
extern crate serde_bytes;

mod cipher;
mod filesystem;
mod memory;
mod super_block;
mod vio;
mod xchacha;

pub use cipher::Cipher;
pub use filesystem::FileSystem;
pub use memory::MemoryStorage;
use orion::aead::SecretKey;
use std::sync::{Arc, RwLock};
use super_block::SuperBlock;
pub use xchacha::XChaCha;

pub trait Storage: Send + Sync {
    // Check if storage is init
    fn is_init(&self) -> bool;

    // Init the new storage and store payload
    fn init(&mut self, password: &[u8], payload: &[u8]);

    // open an existing storage and return payload
    fn open(&mut self, password: &[u8]) -> Vec<u8>;

    // make connection to storage
    fn connect(&mut self);

    // save payload into the super block
    fn save_payload(&mut self, payload: &[u8]);

    // block read/write, can be buffered
    // storage doesn't need to gurantee update is persistent
    fn get_block(&self, cid: &str) -> Vec<u8>;
    fn put_block(&mut self, cid: &str, data: &[u8]);
    fn del_block(&mut self, cid: &str);

    fn is_exist(&self, cid: &str) -> bool;

    // flush blocks
    // storage must gurantee write is persistent
    fn flush(&mut self);

    // permanently destroy this storage
    fn destroy(&mut self);
}

pub type StorageLock<S> = Arc<RwLock<S>>;

// TODO[epic=feat]: generic Cipher
pub trait Crypto: Send + Sync + serde::Serialize {
    fn hash_password(&self, password: &[u8], salt: &[u8]) -> SecretKey;

    fn get_cipher() -> Cipher;

    fn encrypt_with_key(&self, key: &SecretKey, data: &[u8]) -> Vec<u8>;

    fn decrypt_with_key(&self, key: &SecretKey, ciphertext: &[u8]) -> Vec<u8>;
}

struct CryptoUtil {}

impl CryptoUtil {
    pub fn gen_secret_key() -> SecretKey {
        SecretKey::default()
    }
}

/// Dummy storage
#[derive(Debug, Default)]
pub struct DummyStorage;

impl Storage for DummyStorage {
    fn is_init(&self) -> bool {
        unimplemented!()
    }

    #[inline]
    fn init(&mut self, _password: &[u8], _payload: &[u8]) {
        unimplemented!()
    }

    #[inline]
    fn open(&mut self, _password: &[u8]) -> Vec<u8> {
        unimplemented!()
    }

    #[inline]
    fn connect(&mut self) {
        unimplemented!()
    }

    #[inline]
    fn save_payload(&mut self, _payload: &[u8]) {
        unimplemented!()
    }

    #[inline]
    fn get_block(&self, _cid: &str) -> Vec<u8> {
        unimplemented!()
    }

    #[inline]
    fn put_block(&mut self, _cid: &str, _data: &[u8]) {
        unimplemented!()
    }

    #[inline]
    fn del_block(&mut self, _cid: &str) {
        unimplemented!()
    }

    #[inline]
    fn is_exist(&self, _cid: &str) -> bool {
        unimplemented!()
    }

    #[inline]
    fn flush(&mut self) {
        unimplemented!()
    }

    #[inline]
    fn destroy(&mut self) {
        unimplemented!()
    }
}
