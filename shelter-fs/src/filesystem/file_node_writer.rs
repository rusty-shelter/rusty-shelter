use super::{FileContent, FileContentWriter, FileNodeLock};
use fast_cdc::Chunker;
use shelter_block::ShelterBlock;
use shelter_storage::{Storage, StorageLock};
use std::io::{Result as IoResult, Write};

#[derive(Debug)]
pub struct FileNodeWriter<S: Storage> {
    chunker: Chunker<FileContentWriter<S>>,
    file_node: FileNodeLock,
    storage: StorageLock<S>,
}

impl<S: Storage> FileNodeWriter<S> {
    pub fn new(storage: StorageLock<S>, file_node: FileNodeLock) -> Self {
        let file_content = FileContent::new();
        let file_content_writer = FileContentWriter::new(storage.clone(), file_content);
        Self {
            chunker: Chunker::new(file_content_writer),
            file_node,
            storage,
        }
    }
}

impl<S: Storage> Write for FileNodeWriter<S> {
    fn write(&mut self, chunk: &[u8]) -> IoResult<usize> {
        self.chunker.write(chunk)
    }

    fn flush(&mut self) -> IoResult<()> {
        self.chunker.flush()?;
        let file_content = self.chunker.into_inner().get_file_content();
        let mut node = self.file_node.write().unwrap();
        node.add_version(file_content);
        let block = node.new_block();
        let address = node.get_block_id().to_string();
        let mut storage = self.storage.write().unwrap();
        storage.put_block(&address, &block.serialize());
        Ok(())
    }
}
