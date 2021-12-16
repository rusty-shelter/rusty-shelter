use super::{FileBlob, FileContent};
use shelter_block::ShelterBlock;
use shelter_storage::{Storage, StorageLock};
use std::cmp::min;
use std::io::{Read, Result as IoResult, Seek, SeekFrom};

#[derive(Debug)]
pub struct FileContentReader<S: Storage> {
    storage: StorageLock<S>,
    file_content: FileContent,
    pos: usize,
}

impl<S: Storage> FileContentReader<S> {
    pub fn new(storage: StorageLock<S>, file_content: FileContent) -> Self {
        Self {
            storage,
            file_content,
            pos: 0,
        }
    }
}

impl<S: Storage> Read for FileContentReader<S> {
    // TODO[epic=perf] read()
    // See https://github.com/rust-lang/rust/issues/44099
    #[inline]
    fn read(&mut self, buf: &mut [u8]) -> IoResult<usize> {
        if buf.is_empty() {
            return Ok(0);
        }

        let start_offset = self.pos;
        let mut buf_read = 0;

        for blk_addr in self
            .file_content
            .block_address
            .iter()
            .skip_while(|e| e.end_offset() <= start_offset)
        {
            let start_pos = self.pos - blk_addr.offset();
            let mut data_left = blk_addr.len() - start_pos;

            while data_left > 0 {
                let dst = &mut buf[buf_read..];

                // if destination buffer is full, stop reading
                if dst.is_empty() {
                    return Ok(buf_read);
                }

                let blk_data = self
                    .storage
                    .write()
                    .unwrap()
                    .get_block(&blk_addr.get_address());
                let blob = FileBlob::load_from_vec(&blk_data);

                let read_len = min(data_left, dst.len());
                dst[..read_len].copy_from_slice(&blob.data[start_pos..read_len]);
                data_left -= read_len;
                buf_read += read_len;
                self.pos += read_len;
            }
        }

        Ok(buf_read)
    }
}

impl<S: Storage> Seek for FileContentReader<S> {
    fn seek(&mut self, _pos: SeekFrom) -> IoResult<u64> {
        unimplemented!();
    }
}
