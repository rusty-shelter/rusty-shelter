use super::FileContent;
use super::FileType;
use super::FileVersion;
use super::Metadata;
use crate::error::Result;
use crate::time::Time;
use serde::{Deserialize, Serialize};
use shelter_block::{BlockId, BlockType, ShelterBlock};
use shelter_storage::{Storage, StorageLock};
use std::sync::{Arc, RwLock};

pub type FileNodeLock = Arc<RwLock<FileNode>>;

/// FileNode
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct FileNode {
    pub(super) id: BlockId,
    pub name: String,
    pub file_type: FileType,
    pub version: usize,
    versions: Vec<FileVersion>,
    ctime: Time,
    mtime: Time,
}

impl FileNode {
    pub fn new(name: String, file_type: FileType) -> Self {
        let now = Time::now();
        Self {
            id: BlockId::new(),
            name,
            file_type,
            version: 0,
            versions: Vec::new(),
            ctime: now,
            mtime: now,
        }
    }

    /// Check if file node is regular file
    #[inline]
    pub fn is_file(&self) -> bool {
        self.file_type == FileType::File
    }

    /// Check if file node is directory
    #[inline]
    pub fn is_dir(&self) -> bool {
        self.file_type == FileType::Dir
    }

    pub fn file_name(&self) -> &str {
        &self.name
    }

    /// Check if file node is root
    #[inline]
    pub fn is_root(&self) -> bool {
        self.file_name() == "/"
    }

    /// Get fnode metadata
    pub fn metadata(&self) -> Metadata {
        Metadata {
            file_type: self.file_type,
            len: self.len(),
            version: self.get_current_version_number(),
            ctime: self.ctime,
            mtime: self.mtime,
        }
    }

    /// Get fnode version list
    #[inline]
    pub fn history(&self) -> Vec<FileVersion> {
        self.versions.clone()
    }

    /// Returns the byte length of current version.
    #[inline]
    pub fn len(&self) -> usize {
        match self.file_type {
            FileType::File => self.get_current_version().len(),
            FileType::Dir => 0,
        }
    }

    /// Set file to specified length
    ///
    /// if new len is equal to old len, do nothing
    pub fn set_len(_len: usize) -> Result<()> {
        unimplemented!();
    }

    /// Get a specified version
    pub fn get_version(&self, version_number: usize) -> Option<FileVersion> {
        self.versions
            .iter()
            .find(|v| v.version() == version_number)
            .copied()
    }

    /// Get current version number
    #[inline]
    pub fn get_current_version_number(&self) -> usize {
        self.version
    }

    /// Get current version
    pub fn get_current_version(&self) -> FileVersion {
        self.versions[self.version]
    }

    pub(super) fn get_current_block_id(&self) -> BlockId {
        self.versions[self.version].id
    }

    /// Remove a specified version and its associated content
    pub fn remove_version(&self, _version_number: usize) -> Result<()> {
        unimplemented!();
    }

    /// Remove all versions and its associated content
    pub fn clear_versions(&mut self) -> Result<()> {
        self.version = 0;
        self.versions.clear();
        Ok(())
    }

    /// Add a new immutable version
    pub fn add_version(&mut self, file_content: &FileContent) {
        // Erase empty version 0 (NoVersion)
        if !self.versions.is_empty() {
            self.version += 1;
        }
        self.versions.push(FileVersion {
            id: file_content.id,
            version: self.version,
            len: file_content.len,
            ctime: file_content.ctime,
        });
    }

    pub fn clone_current_content<S: Storage>(&self, storage: StorageLock<S>) -> FileContent {
        let file_version = self.get_current_version();
        let content_id = file_version.id;
        let data = storage.read().unwrap().get_block(&content_id.to_string());
        let mut file_content = FileContent::load_from_vec(&data);
        file_content.id = BlockId::new();
        file_content
    }

    // Get reader for sepcified version number
    // pub fn version_reader(&self, ver_num: usize) -> Result<FileNodeReader> {
    //     let ver = self.ver(ver_num).ok_or(Error::NoVersion)?;
    //     let content
    // }
}

impl ShelterBlock for FileNode {
    type ItemBlock = Self;

    fn get_block_id(&self) -> BlockId {
        self.id
    }

    fn get_block_type(&self) -> BlockType {
        BlockType::FILE
    }
}

// TODO[epic=tests,seq=81] Make FileNode unit tests
#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_serialize() {
        let node = FileNode::new(String::from("test"), FileType::File);
        let block = node.new_block();
        let data = block.serialize();
        let node2 = FileNode::load_from_vec(&data);
        assert_eq!(node, node2);
    }

    #[test]
    fn test_clone_content() {}
}
