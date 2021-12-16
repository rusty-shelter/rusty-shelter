use super::{FileBlob, FileContent};
use shelter_block::ShelterBlock;
use shelter_storage::{Storage, StorageLock};
use std::io::{Result as IoResult, Seek, SeekFrom, Write};

#[derive(Debug)]
pub struct FileContentWriter<S: Storage> {
    storage: StorageLock<S>,
    file_content: FileContent,
}

impl<S: Storage> FileContentWriter<S> {
    pub fn new(storage: StorageLock<S>, file_content: FileContent) -> Self {
        Self {
            storage,
            file_content,
        }
    }

    pub fn get_file_content(&self) -> &FileContent {
        &self.file_content
    }
}

impl<S: Storage> Write for FileContentWriter<S> {
    fn write(&mut self, chunk: &[u8]) -> IoResult<usize> {
        let block = FileBlob::new(chunk.to_owned()).new_block();
        let address = block.get_block_address();
        let mut storage = self.storage.write().unwrap();
        self.file_content
            .push_block_address(address.clone(), chunk.len());
        storage.put_block(&address, &block.serialize());
        Ok(0)
    }

    fn flush(&mut self) -> IoResult<()> {
        let block = self.file_content.new_block();
        let address = self.file_content.get_block_id().to_string();
        let mut storage = self.storage.write().unwrap();
        storage.put_block(&address, &block.serialize());
        Ok(())
    }
}

impl<S: Storage> Seek for FileContentWriter<S> {
    fn seek(&mut self, _pos: SeekFrom) -> IoResult<u64> {
        unimplemented!();
    }
}
