use crate::{Crypto, SecretKey};
use bincode::config::Options;
use orion::util;

/// Stands for Shelter Super Block Version 1
const SIGNATURE: (char, char, char, char, char) = ('S', 'S', 'B', 'V', '1');

/// Super block head, not encrypted
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub(super) struct SuperBlockHead<C: Crypto> {
    pub signature: (char, char, char, char, char),
    #[serde(with = "serde_bytes")]
    pub salt: Vec<u8>,
    pub crypto: C,
}

/// Super block body, encrypted
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub(super) struct SuperBlockBody {
    #[serde(with = "serde_bytes")]
    data_key: Vec<u8>,
    #[serde(with = "serde_bytes")]
    pub payload: Vec<u8>,
}

/// The Shelter SuperBlock type has the following binary format :
///
/// <signature><is encrypted><crypto?><content size><content>
///   - 5-byte signature: { 'S', 'S', 'B', 'V', '1' }
///   - salt size (u64, 8 bytes)
///   - salt (buffer)
///   - cipher code (u32)
///   - ops limit (u32)
///   - mem limit (u32)
///   - data key size (varint, encrypted)
///   - data encrypt/decrypt key (buffer, encrypted)
///   - content size (varint, encrypted)
///   - content payload (buffer, encrypted)
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub(super) struct SuperBlock<C: Crypto> {
    pub head: SuperBlockHead<C>,
    pub body: SuperBlockBody,
}

impl<C: Crypto> SuperBlock<C>
where
    C: serde::de::DeserializeOwned,
{
    const SALT_SIZE: usize = 16;
    const HEAD_LEN: usize = 5 + 8 + Self::SALT_SIZE + 4 + 4 + 4;

    pub fn new(crypto: C) -> Self {
        Self {
            head: SuperBlockHead {
                signature: SIGNATURE,
                salt: Vec::with_capacity(Self::SALT_SIZE),
                crypto,
            },
            body: SuperBlockBody {
                data_key: vec![],
                payload: vec![],
            },
        }
    }

    pub fn init(&mut self) {
        let mut salt = [0u8; 16];
        util::secure_rand_bytes(&mut salt).expect("to work");
        self.head.salt = Vec::from(salt);
    }

    pub fn deserialize(block: &[u8], key: &SecretKey) -> Self {
        // Load super block header
        let head: SuperBlockHead<C> = bincode::options()
            .with_fixint_encoding()
            .deserialize(&block[..Self::HEAD_LEN])
            .unwrap();

        // Decrypt body
        let body = head.crypto.decrypt_with_key(key, &block[Self::HEAD_LEN..]);

        // Create super block
        let body: SuperBlockBody = bincode::options().deserialize(&body).unwrap();
        Self { head, body }
    }

    pub fn serialize(&self, key: &SecretKey) -> Vec<u8> {
        let mut head = bincode::options()
            .with_fixint_encoding()
            .serialize(&self.head)
            .expect("to work");
        let body = bincode::options().serialize(&self.body).expect("to work");

        let mut body_crypt = self.head.crypto.encrypt_with_key(key, &body);

        let mut ret = vec![];
        ret.append(&mut head);
        ret.append(&mut body_crypt);
        ret
    }

    #[inline]
    pub fn set_payload(&mut self, payload: &[u8]) {
        self.body.payload = Vec::from(payload);
    }

    #[inline]
    pub fn set_data_key(&mut self, data_key: &SecretKey) {
        self.body.data_key = Vec::from(data_key.unprotected_as_bytes());
    }

    #[inline]
    pub fn get_data_key(&self) -> SecretKey {
        SecretKey::from_slice(&self.body.data_key).expect("to work")
    }

    #[inline]
    pub fn get_master_key(&self, password: &[u8]) -> SecretKey {
        // TODO[epic=feat]: check salt is initialized
        self.head.crypto.hash_password(password, &self.head.salt)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::XChaCha;

    #[test]
    fn serialize_work() {
        let crypto = XChaCha::new(3, 1 << 4);
        let mut super_block = SuperBlock::new(crypto);

        super_block.init();
        // println!("SB {:x?}\n", super_block);
        // println!("SB serialize {:x?}\n", bincode::options().with_fixint_encoding().serialize(&super_block));

        let master_key = super_block.get_master_key("42".as_bytes());

        let seri = super_block.serialize(&master_key);

        let deseri = SuperBlock::deserialize(&seri, &master_key);

        assert_eq!(super_block, deseri);
    }
}
