use super::{FileContent, FileContentReader, FileNodeLock};
use shelter_block::ShelterBlock;
use shelter_storage::{Storage, StorageLock};
use std::io::{Read, Result as IoResult, Seek, SeekFrom};

/// Read current version of the FileNode
#[derive(Debug)]
pub struct FileNodeReader<S: Storage> {
    storage: StorageLock<S>,
    file_node: FileNodeLock,
    reader: Option<FileContentReader<S>>,
}

impl<S: Storage> FileNodeReader<S> {
    pub fn new(storage: StorageLock<S>, file_node: FileNodeLock) -> Self {
        Self {
            storage,
            file_node,
            reader: None,
        }
    }

    /// Create a reader for current version
    fn get_reader(&mut self) -> &mut FileContentReader<S> {
        self.reader.get_or_insert_with(|| {
            let node = self.file_node.read().unwrap();
            let block_id = node.get_current_block_id();
            let data = self
                .storage
                .read()
                .unwrap()
                .get_block(&block_id.to_string());
            let file_content = FileContent::load_from_vec(&data);
            FileContentReader::new(self.storage.clone(), file_content)
        })
    }
}

impl<S: Storage> Read for FileNodeReader<S> {
    #[inline]
    fn read(&mut self, buf: &mut [u8]) -> IoResult<usize> {
        let reader = self.get_reader();
        reader.read(buf)
    }
}

impl<S: Storage> Seek for FileNodeReader<S> {
    #[inline]
    fn seek(&mut self, pos: SeekFrom) -> IoResult<u64> {
        let reader = self.get_reader();
        reader.seek(pos)
    }
}
