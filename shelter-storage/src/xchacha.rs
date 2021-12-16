use super::{Cipher, Crypto};
use orion::{aead, kdf};

// Re export SecretKey
pub use aead::SecretKey;
pub use kdf::Salt;

// Crypto utility
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct XChaCha {
    pub cipher: Cipher,
    pub ops_cost: u32,
    pub mem_cost: u32,
}

impl XChaCha {
    /// Choosing the correct cost parameters is important for security.
    /// Please refer to libsodium's docs for a description of how to do this.
    ///
    pub fn new(ops_cost: u32, mem_cost: u32) -> Self {
        Self {
            cipher: Cipher::XChaCha20Poly1305,
            ops_cost,
            mem_cost,
        }
    }

    #[inline]
    pub fn set_cipher(&mut self, cipher: Cipher) {
        self.cipher = cipher;
    }

    #[inline]
    pub fn set_ops_cost(&mut self, ops_cost: u32) {
        self.ops_cost = ops_cost;
    }

    #[inline]
    pub fn set_mem_cost(&mut self, mem_cost: u32) {
        self.mem_cost = mem_cost;
    }
}

impl Crypto for XChaCha {
    fn get_cipher() -> Cipher {
        Cipher::XChaCha20Poly1305
    }

    fn hash_password(&self, password: &[u8], salt: &[u8]) -> aead::SecretKey {
        let salt = kdf::Salt::from_slice(salt).expect("Salt::from_slice should work");
        let pass = kdf::Password::from_slice(password).expect("Password::from_slice should work");
        kdf::derive_key(&pass, &salt, self.ops_cost, self.mem_cost, 32)
            .expect("kdf::derive_key should work")
    }

    #[inline]
    fn encrypt_with_key(&self, key: &aead::SecretKey, data: &[u8]) -> Vec<u8> {
        match self.cipher {
            Cipher::XChaCha20Poly1305 => aead::seal(key, data).expect("Encrypt data failed"),
            // Cipher::AES => unimplemented!(),
        }
    }

    #[inline]
    fn decrypt_with_key(&self, key: &aead::SecretKey, ciphertext: &[u8]) -> Vec<u8> {
        match self.cipher {
            Cipher::XChaCha20Poly1305 => aead::open(key, ciphertext).expect("Decrypt data failed"),
            // Cipher::AES => unimplemented!(),
        }
    }
}

impl Default for XChaCha {
    fn default() -> Self {
        Self {
            cipher: Cipher::XChaCha20Poly1305,
            mem_cost: 1 << 8,
            ops_cost: 3,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn enc_dec() {
        let crypto = XChaCha::default();
        const LEN: usize = 10;
        let data = vec![3u8; LEN];
        let key = SecretKey::default();

        // encryption
        let out = crypto.encrypt_with_key(&key, &data);
        assert!(out.len() > 0);

        // decryption
        let ret = crypto.decrypt_with_key(&key, &out);
        assert_eq!(ret, data);
    }
}
