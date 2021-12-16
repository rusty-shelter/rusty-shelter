use super::Metadata;
use camino::{Utf8Path, Utf8PathBuf};

/// Entries returned by the [`read_dir`] function.
///
/// An instance of `DirEntry` represents an entry inside of a directory in the
/// repository. Each entry can be inspected via methods to learn about the
/// absolute path or other metadata.
///
/// [`read_dir`]: struct.Repo.html#method.read_dir
#[derive(Debug)]
pub struct DirEntry {
    pub(super) path: Utf8PathBuf,
    pub(super) name: String,
    pub(super) metadata: Metadata,
}

impl DirEntry {
    /// Returns the absolute path to the file that this entry represents.
    pub fn path(&self) -> &Utf8Path {
        self.path.as_path()
    }

    /// Returns the bare file name of this directory entry without any other
    /// leading path component.
    pub fn file_name(&self) -> &str {
        &self.name
    }

    /// Return the metadata for the file that this entry points at.
    pub fn metadata(&self) -> Metadata {
        self.metadata
    }
}
