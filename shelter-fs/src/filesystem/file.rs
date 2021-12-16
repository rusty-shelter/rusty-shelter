#![allow(clippy::unused_io_amount)]
use super::{
    open_file::open_file, FileNode, FileNodeLock, FileNodeReader, FileNodeWriter, FileSystem,
    FileVersion, Metadata, OpenOptions,
};
use crate::error::{Error, Result};
use camino::Utf8Path;
use shelter_storage::{Storage, StorageLock};
use std::io::{Error as IoError, ErrorKind, Read, Result as IoResult, Seek, SeekFrom, Write};
use std::sync::{Arc, RwLock};

/// A reference to an opened file in the repository.
/// TODO[epic=doc] File
pub struct File<S: Storage> {
    pub options: OpenOptions,
    storage: StorageLock<S>,
    position: SeekFrom,
    file_node: FileNodeLock,
    reader: Option<FileNodeReader<S>>,
    writer: Option<FileNodeWriter<S>>,
}

impl<S: Storage> File<S> {
    pub(super) fn new(options: OpenOptions, storage: StorageLock<S>, file_node: FileNode) -> Self {
        Self {
            options,
            storage,
            position: SeekFrom::Start(0),
            file_node: Arc::new(RwLock::new(file_node)),
            reader: None,
            writer: None,
        }
    }

    /// Open an existing file
    pub fn open(fs: &mut FileSystem<S>, path: &Utf8Path) -> Result<File<S>> {
        let open_options = OpenOptions::default();
        open_file(fs, path, open_options)
    }

    /// Create a new file, truncate if already exist
    pub fn create<P: AsRef<Utf8Path>>(fs: &mut FileSystem<S>, path: P) -> Result<File<S>> {
        OpenOptions::new()
            .write(true)
            .create(true)
            .truncate(true)
            .open(fs, path.as_ref())
    }

    /// Queries metadata about the file.
    pub fn metadata(&self) -> Result<Metadata> {
        let node = self.file_node.read().unwrap();
        Ok(node.metadata())
    }

    /// Returns a list of all the file versions.
    pub fn history(&self) -> Result<Vec<FileVersion>> {
        let node = self.file_node.read().unwrap();
        Ok(node.history())
    }

    /// Returns byte size of the current version.
    pub fn len(&self) -> usize {
        let node = self.file_node.read().unwrap();
        node.len()
    }

    /// Returns the current version number.
    pub fn version(&self) -> usize {
        let node = self.file_node.read().unwrap();
        node.get_current_version_number()
    }

    /// Single-part write to file and create a new version.
    ///
    /// This method provides a convenient way of combining [`Write`] and
    /// [`flush`].
    ///
    /// This method is atomic.
    ///
    /// [`Write`]: https://doc.rust-lang.org/std/io/trait.Write.html
    /// [`flush`]: struct.File.html#method.flush
    pub fn write_once(&mut self, buf: &[u8]) -> Result<usize> {
        self.write(buf)?;
        self.flush()?;
        Ok(0)
    }

    /// Truncates or extends the underlying file, create a new version of
    /// content which size to become `size`.
    ///
    /// If the size is less than the current content size, then the new
    /// content will be shrunk. If it is greater than the current content size,
    /// then the content will be extended to `size` and have all of the
    /// intermediate data filled in with 0s.
    ///
    /// This method is atomic.
    ///
    /// # Errors
    ///
    /// This method will return an error if the file is not opened for writing
    /// or not finished writing.
    pub fn set_len(&mut self) {
        unimplemented!();
    }
}

impl<S: Storage> Write for File<S> {
    fn write(&mut self, buf: &[u8]) -> IoResult<usize> {
        let storage = self.storage.clone();
        if self.writer.is_none() {
            if self.options.contains(OpenOptions::FILE_WRITE) {
                self.writer = Some(FileNodeWriter::new(storage, self.file_node.clone()));
            } else {
                return Err(IoError::new(
                    ErrorKind::Other,
                    Error::CannotWrite.to_string(),
                ));
            }
        }

        match self.writer {
            Some(ref mut wtr) => {
                wtr.write(buf)?;
                Ok(0)
            }
            None => unreachable!(),
        }
    }

    fn flush(&mut self) -> IoResult<()> {
        match self.writer.take() {
            Some(ref mut writer) => writer.flush(),
            None => Err(IoError::new(
                ErrorKind::PermissionDenied,
                Error::CannotWrite.to_string(),
            )),
        }
    }
}

impl<S: Storage> Read for File<S> {
    fn read(&mut self, buf: &mut [u8]) -> IoResult<usize> {
        let storage = self.storage.clone();
        if self.reader.is_none() {
            if self.options.contains(OpenOptions::FILE_READ) {
                self.reader = Some(FileNodeReader::new(storage, self.file_node.clone()));
            } else {
                return Err(IoError::new(
                    ErrorKind::Other,
                    Error::CannotRead.to_string(),
                ));
            }
        }

        match self.reader {
            Some(ref mut rdr) => {
                let read = rdr.read(buf)?;
                // let new_pos = rdr.seek(SeekFrom::Current(0)).unwrap();
                // self.pos = SeekFrom::Start(new_pos);
                Ok(read)
            }
            None => unreachable!(),
        }
    }
}

impl<S: Storage> Seek for File<S> {
    fn seek(&mut self, pos: SeekFrom) -> IoResult<u64> {
        // TODO: Start here !!!!
        if self.writer.is_some() {
            return Err(IoError::new(ErrorKind::Other, Error::NotFinish.to_string()));
        }

        self.position = match self.reader {
            Some(ref mut rdr) => SeekFrom::Start(rdr.seek(pos)?),
            // None => self.seek_pos(pos),
            None => unreachable!(),
        };

        match self.position {
            SeekFrom::Start(pos) => Ok(pos),
            _ => unreachable!(),
        }
    }
}

// TODO[epic=tests,seq=81] Make File integration tests
