#![allow(clippy::module_inception)]
mod dir_entry;
mod file;
mod file_blob;
mod file_content;
mod file_content_reader;
mod file_content_writer;
mod file_node;
mod file_node_reader;
mod file_node_writer;
mod file_type;
mod file_version;
mod metadata;
mod open_file;
mod open_options;
mod tree;

pub use dir_entry::DirEntry;
pub use file::File;
pub use file_blob::FileBlob;
pub use file_content::FileContent;
pub use file_content_reader::FileContentReader;
pub use file_content_writer::FileContentWriter;
pub use file_node::{FileNode, FileNodeLock};
pub use file_node_reader::FileNodeReader;
pub use file_node_writer::FileNodeWriter;
pub use file_type::FileType;
pub use file_version::FileVersion;
pub use metadata::Metadata;
pub use open_options::OpenOptions;
pub use tree::{Tree, TreeLock};

use crate::error::{Error, Result};
use camino::{Utf8Path, Utf8PathBuf};
use crdt_tree::{Clock, OpMove};
use serde::{Deserialize, Serialize};
use shelter_block::{BlockId, ShelterBlock};
use shelter_storage::{Storage, StorageLock};
use std::sync::{Arc, RwLock};

// define some concrete types to instantiate our Tree data structures with.
type TypeId = Utf8PathBuf;
type TypeMeta = BlockId;
type TypeActor = String;

// Rusty shelter file system options
bitflags! {
    #[derive(Serialize, Deserialize)]
    pub struct FileSystemOptions: u32 {
        const REPO_READ_ONLY     = 0b0000_0001;
        const REPO_VERSIONED     = 0b0000_0010;
        // const REPO_DEDUPLICATION = 0b0000_0010;
    }
}
impl Default for FileSystemOptions {
    fn default() -> FileSystemOptions {
        FileSystemOptions::REPO_VERSIONED & FileSystemOptions::REPO_READ_ONLY
    }
}

/// Rusty shelter file system
pub struct FileSystem<S: Storage> {
    options: FileSystemOptions,
    pub tree: Option<TreeLock>,
    pub storage: StorageLock<S>,
}

impl<S: Storage> FileSystem<S> {
    // const TRAHS_NODE_ID: BlockId = BlockId::get_magic();

    /// New file system
    pub fn new(options: FileSystemOptions, storage: S) -> Self {
        Self {
            options,
            tree: None,
            storage: Arc::new(RwLock::new(storage)),
        }
    }

    // Init file system storage
    #[inline]
    pub fn init(&mut self, _name: &str, password: &str) {
        let mut storage = self.storage.write().unwrap();
        if storage.is_init() {
            let data = storage.open(password.as_bytes());
            self.tree = Some(Arc::new(RwLock::new(Tree::load_from_vec(&data))));
        } else {
            let tree = Tree::new();
            let block = tree.new_block();
            self.tree = Some(Arc::new(RwLock::new(tree)));
            storage.init(password.as_bytes(), &block.serialize());
        }
    }

    pub fn save() -> bool {
        true
    }

    #[inline]
    pub fn is_read_only(&self) -> bool {
        self.options.contains(FileSystemOptions::REPO_READ_ONLY)
    }

    // Get BlockId of the path
    #[inline]
    pub(crate) fn get_path_id(&self, path: &Utf8Path) -> Option<BlockId> {
        self.tree
            .as_ref()
            .unwrap()
            .read()
            .unwrap()
            .replica
            .tree()
            .find(&path.to_owned())
            .map(|tree_node| tree_node.metadata().to_owned())
    }

    #[inline]
    pub fn is_file_node_exist(&self, path: &Utf8Path) -> bool {
        self.get_path_id(path).is_none()
    }

    pub(crate) fn open_fnode_with_id(&self, node_id: BlockId) -> FileNode {
        let data = self.storage.read().unwrap().get_block(&node_id.to_string());
        FileNode::load_from_vec(&data)
    }

    /// Open an existing FileNode
    pub fn open_fnode(&self, path: &Utf8Path) -> Result<FileNode> {
        // 1. Check
        if !path.has_root() {
            // only resolve absolute path
            return Err(Error::InvalidPath);
        }

        // 2. Get file node
        let node_id = self.get_path_id(path).ok_or(Error::NotFound)?;
        let node = self.open_fnode_with_id(node_id);

        Ok(node)
    }

    /// Create a file node
    pub fn create_fnode(&mut self, path: &Utf8Path, file_type: FileType) -> Result<FileNode> {
        // 1. Check
        if self.is_read_only() {
            return Err(Error::ReadOnly);
        }
        if !path.has_root() {
            // only resolve absolute path
            return Err(Error::InvalidPath);
        }
        if self.get_path_id(path).is_some() {
            return Err(Error::AlreadyExists);
        }

        // 2. Get parent
        let parent_path = path.parent().ok_or(Error::IsRoot)?;
        self.get_path_id(parent_path).ok_or(Error::InvalidPath)?;

        // 3. Create file node
        let file_name = path.file_name().ok_or(Error::InvalidPath)?;
        let node = FileNode::new(file_name.to_string(), file_type);

        // 4. Write file node into storage
        // self.store_paths.insert(path.to_owned(), node.id);
        self.storage
            .write()
            .unwrap()
            .put_block(&node.id.to_string(), &node.new_block().serialize());

        // 5. Update crdt
        let ops = self
            .tree
            .as_ref()
            .unwrap()
            .read()
            .unwrap()
            .replica
            .opmoves(vec![(parent_path.to_owned(), node.id, path.to_owned())]);
        self.tree
            .as_ref()
            .unwrap()
            .write()
            .unwrap()
            .replica
            .apply_ops_byref(&ops);

        Ok(node)
    }

    pub fn remove_file_node(&self, _id: BlockId) -> Result<()> {
        Ok(())
    }

    /// Recursively create directories along the path
    pub fn create_dir_all(&mut self, path: &Utf8Path) -> Result<()> {
        if self.is_read_only() {
            return Err(Error::ReadOnly);
        }
        if !path.has_root() {
            // only resolve absolute path
            return Err(Error::InvalidPath);
        }

        let mut path_buf = Utf8PathBuf::new();
        for entry in path.iter() {
            path_buf.push(entry);
            let path_part = path_buf.as_path();
            if self.get_path_id(path_part).is_none() && path_part != "/" {
                self.create_fnode(path_part, FileType::Dir)?;
            }
        }
        Ok(())
    }

    /// Read directory entries
    pub fn read_dir(&self, base_path: &Utf8Path) -> Result<Vec<DirEntry>> {
        // let parent_id = self.get_path_id(base_path).ok_or(Error::InvalidPath)?;
        let tree_lock = self.tree.as_ref().unwrap().read().unwrap();
        let tree = tree_lock.replica.tree();
        let childrens = tree.children(&base_path.to_owned());
        let ret = childrens
            .iter()
            .map(|path| {
                let node_id = self.get_path_id(path).expect("path should exist");
                let data = self.storage.read().unwrap().get_block(&node_id.to_string());
                let node = FileNode::load_from_vec(&data);
                DirEntry {
                    path: base_path.join(&node.name),
                    name: node.file_name().to_string(),
                    metadata: node.metadata(),
                }
            })
            .collect();

        Ok(ret)
    }

    /// Get metadata of specified path
    pub fn metadata(&self, path: &Utf8Path) -> Result<Metadata> {
        let node = self.open_fnode(path)?;
        Ok(node.metadata())
    }

    pub fn history(&self, path: &Utf8Path) -> Result<Vec<FileVersion>> {
        let node_id = self.get_path_id(path).ok_or(Error::InvalidPath)?;
        let data = self.storage.read().unwrap().get_block(&node_id.to_string());
        let node = FileNode::load_from_vec(&data);
        if node.is_dir() {
            return Err(Error::IsDir);
        }
        Ok(node.history())
    }

    // Copy a regular file to another
    pub fn copy(&mut self, from: &Utf8Path, to: &Utf8Path) -> Result<()> {
        if self.is_read_only() {
            return Err(Error::ReadOnly);
        }

        let source = self.open_fnode(from)?;
        if !source.is_file() {
            return Err(Error::NotFile);
        }

        let mut target = if self.is_file_node_exist(to) {
            let node = self.open_fnode(to)?;
            if node.is_file() {
                node
            } else {
                return Err(Error::InvalidPath);
            }
        } else {
            self.create_fnode(to, FileType::Dir)?
        };

        let file_content = source.clone_current_content(self.storage.clone());
        target.add_version(&file_content);

        Ok(())
    }

    // TODO[epic=feat] Implement copy_dir_all
    pub fn copy_dir_all(&mut self, from: &Utf8Path, to: &Utf8Path) -> Result<()> {
        if self.is_read_only() {
            return Err(Error::ReadOnly);
        }

        if from == to {
            return Ok(());
        }

        if to.starts_with(from) {
            return Err(Error::InvalidArgument);
        }

        for child in self.read_dir(from)? {
            let child_from = child.path();
            let child_to = to.join(child.file_name());
            match child.metadata().file_type() {
                FileType::File => self.copy(child_from, &child_to)?,
                FileType::Dir => self.copy_dir_all(child_from, &child_to)?,
            }
        }

        Ok(())
    }

    // TODO[epic=feat] Implement remove_file
    pub fn remove_file(&mut self, path: &Utf8Path) -> Result<()> {
        if self.is_read_only() {
            return Err(Error::ReadOnly);
        }

        let mut node = self.open_fnode(path)?;
        if !node.is_file() {
            return Err(Error::NotFile);
        }

        node.clear_versions()?;
        let mut storage = self.storage.write().unwrap();
        storage.del_block(&node.id.to_string());
        // TODO: Remove FileContent and FileBlob
        // Be carreful to deduplication

        Ok(())
    }

    // TODO[epic=feat] Implement remove_dir
    pub fn remove_dir(&mut self, path: &Utf8Path) -> Result<()> {
        if self.is_read_only() {
            return Err(Error::ReadOnly);
        }

        let node = self.open_fnode(path)?;
        if !node.is_dir() {
            return Err(Error::NotDir);
        }
        if node.is_root() {
            return Err(Error::IsRoot);
        }
        // TODO: Check dir is empty

        let mut storage = self.storage.write().unwrap();
        storage.del_block(&node.id.to_string());

        Ok(())
    }

    // TODO[epic=feat] Implement remove_dir_all
    pub fn remove_dir_all(&mut self, path: &Utf8Path) -> Result<()> {
        for child in self.read_dir(path)? {
            let child_path = child.path();
            match child.metadata().file_type() {
                FileType::File => self.remove_file(child_path)?,
                FileType::Dir => self.remove_dir_all(child_path)?,
            }
        }

        self.remove_dir(path)
    }

    // TODO[epic=feat] Implement rename
    pub fn rename(&mut self, from: &Utf8Path, to: &Utf8Path) -> Result<()> {
        if self.is_read_only() {
            return Err(Error::ReadOnly);
        }
        if from == to {
            return Ok(());
        }
        if to.starts_with(from) {
            return Err(Error::InvalidArgument);
        }

        Ok(())
    }

    pub fn get_last_tick(&self) -> Clock<TypeActor> {
        self.tree
            .as_ref()
            .unwrap()
            .read()
            .unwrap()
            .replica
            .time()
            .to_owned()
    }

    pub fn apply_ops(&mut self, ops: Vec<OpMove<TypeId, TypeMeta, TypeActor>>) {
        self.tree
            .as_ref()
            .unwrap()
            .write()
            .unwrap()
            .replica
            .apply_ops(ops);
    }

    #[inline]
    pub fn destroy(&self) -> Result<()> {
        let mut storage = self.storage.write().unwrap();
        storage.destroy();
        Ok(())
    }
}

// impl ShelterBlock for FileSystem {
//     type ItemBlock = Self;

//     fn get_block_id(&self) -> BlockId {
//         self.id
//     }

//     fn get_block_type(&self) -> BlockType {
//         BlockType::FVER
//     }
// }

// TODO[epic=tests] Make Filesystem integration tests
