use data_encoding::HEXUPPER;
use orion::hash::{digest, Digest};
use serde::{Deserialize, Serialize};

bitflags! {
    #[derive(Serialize, Deserialize)]
    pub struct RepositoryOptions: u32 {
        const REPO_READ_ONLY     = 0b0000_0001;
        const REPO_VERSIONED     = 0b0000_0010;
        // const REPO_DEDUPLICATION = 0b0000_0010;
    }
}

// explicit `Default` implementation
impl Default for RepositoryOptions {
    fn default() -> RepositoryOptions {
        RepositoryOptions::REPO_VERSIONED & RepositoryOptions::REPO_READ_ONLY
    }
}

/// TODO[epic=doc] RepositoryConfig
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RepositoryConfig {
    pub options: RepositoryOptions,
    pub name: String,
    pub salt: String,
    super_block_name: String,
}

impl RepositoryConfig {
    pub fn new(name: &str, salt: &str, options: RepositoryOptions) -> RepositoryConfig {
        let hash: Digest = digest(name.as_bytes()).unwrap();

        RepositoryConfig {
            options,
            name: name.to_string(),
            salt: salt.to_string(),
            super_block_name: HEXUPPER.encode(hash.as_ref()),
        }
    }

    #[inline]
    pub fn set_options(&mut self, options: RepositoryOptions) {
        self.options = options;
    }
}
