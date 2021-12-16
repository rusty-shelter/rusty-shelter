use super::FileType;
use crate::time::Time;
use std::time::SystemTime;

/// Metadata information about a file or a directory.
///
/// This structure is returned from the [`File::metadata`] and
/// [`Repo::metadata`] represents known metadata about a file such as its type,
/// size, modification times and etc.
///
/// [`File::metadata`]: struct.File.html#method.metadata
/// [`Repo::metadata`]: struct.Repo.html#method.metadata
#[derive(Debug, Copy, Clone)]
pub struct Metadata {
    pub(super) file_type: FileType,
    pub(super) len: usize,
    pub(super) version: usize,
    pub(super) ctime: Time,
    pub(super) mtime: Time,
}

impl Metadata {
    /// Returns the file type for this metadata.
    pub fn file_type(&self) -> FileType {
        self.file_type
    }

    /// Returns whether this metadata is for a directory.
    pub fn is_dir(&self) -> bool {
        self.file_type == FileType::Dir
    }

    /// Returns whether this metadata is for a regular file.
    pub fn is_file(&self) -> bool {
        self.file_type == FileType::File
    }

    /// Returns the size of the current version of file, in bytes, this
    /// metadata is for.
    pub fn len(&self) -> usize {
        self.len
    }

    /// Returns current version number of file listed in this metadata.
    pub fn version(&self) -> usize {
        self.version
    }

    /// Returns the creation time listed in this metadata.
    pub fn created_at(&self) -> SystemTime {
        self.ctime.to_system_time()
    }

    /// Returns the last modification time listed in this metadata.
    pub fn modified_at(&self) -> SystemTime {
        self.mtime.to_system_time()
    }
}
