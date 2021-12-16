use crate::error::Result;
use crate::filesystem::{
    DirEntry, File, FileSystem, FileSystemOptions, FileType, FileVersion, Metadata, OpenOptions,
};
use camino::Utf8Path;
use shelter_storage::Storage;

/// TODO[epic=doc] Repository
pub struct Repository<S: Storage> {
    pub fs: FileSystem<S>,
}

impl<S: Storage> Repository<S> {
    /// Create new repository
    ///
    /// # Error
    #[inline]
    pub fn new(config: FileSystemOptions, storage: S) -> Self {
        Self {
            fs: FileSystem::new(config, storage),
        }
    }
    /// Initialize storage
    ///
    /// # Error
    #[inline]
    pub fn init(&mut self, name: &str, password: &str) {
        self.fs.init(name, password);
    }

    /// Returns whether the path points at an existing entity in repository.
    ///
    /// `path` must be an absolute path.
    pub fn path_exists<P: AsRef<Utf8Path>>(&self, path: P) -> Result<bool> {
        Ok(self
            .fs
            .get_path_id(path.as_ref())
            .map(|_| true)
            .unwrap_or(false))
    }

    /// Returns whether the path exists in repository and is pointing at
    /// a regular file.
    ///
    /// `path` must be an absolute path.
    pub fn is_file<P: AsRef<Utf8Path>>(&self, path: P) -> Result<bool> {
        match self.fs.open_fnode(path.as_ref()) {
            Ok(file_node) => Ok(file_node.is_file()),
            Err(_) => Ok(false),
        }
    }

    /// Returns whether the path exists in repository and is pointing at
    /// a directory.
    ///
    /// `path` must be an absolute path.
    pub fn is_dir<P: AsRef<Utf8Path>>(&self, path: P) -> Result<bool> {
        match self.fs.open_fnode(path.as_ref()) {
            Ok(file_node) => Ok(file_node.is_dir()),
            Err(_) => Ok(false),
        }
    }

    /// Create a file in read-write mode.
    ///
    /// This method will create a file if it does not exist, and will
    /// truncate it if it does. See the
    /// [`OpenOptions::open`](struct.OpenOptions.html#method.open) method for
    /// more details.
    ///
    /// `path` must be an absolute path.
    #[inline]
    pub fn create_file(&mut self, path: &Utf8Path) -> File<S> {
        OpenOptions::new()
            .create(true)
            .write(true)
            .truncate(true)
            .open(&mut self.fs, path)
            .unwrap()
    }

    /// Attempts to open a file in read-only mode.
    ///
    /// `path` must be an absolute path.
    ///
    /// See the [`OpenOptions::open`] method for more details.
    pub fn open_file(&mut self, path: &Utf8Path) -> File<S> {
        OpenOptions::new()
            .read(true)
            .open(&mut self.fs, path)
            .unwrap()
    }

    /// Creates a new, empty directory at the specified path.
    ///
    /// `path` must be an absolute path.
    ///
    /// This method is atomic.
    #[inline]
    pub fn create_dir<P: AsRef<Utf8Path>>(&mut self, path: P) -> Result<()> {
        self.fs
            .create_fnode(path.as_ref(), FileType::Dir)
            .map(|_| ())
    }

    /// Recursively create a directory and all of its parent components if they
    /// are missing.
    ///
    /// `path` must be an absolute path.
    ///
    /// This method is **not** atomic in whole, but creating each entry is
    /// atomic.
    #[inline]
    pub fn create_dir_all<P: AsRef<Utf8Path>>(&mut self, path: P) -> Result<()> {
        self.fs.create_dir_all(path.as_ref())
    }

    /// Returns a vector of all the entries within a directory.
    ///
    /// `path` must be an absolute path.
    #[inline]
    pub fn read_dir<P: AsRef<Utf8Path>>(&self, path: P) -> Result<Vec<DirEntry>> {
        self.fs.read_dir(path.as_ref())
    }

    /// Get the metadata about a file or directory at specified path.
    ///
    /// `path` must be an absolute path.
    #[inline]
    pub fn metadata<P: AsRef<Utf8Path>>(&self, path: P) -> Result<Metadata> {
        self.fs.metadata(path.as_ref())
    }

    /// Return a vector of history versions of a regular file at specified path.
    ///
    /// `path` must be an absolute path to a regular file.
    #[inline]
    pub fn history<P: AsRef<Utf8Path>>(&self, path: P) -> Result<Vec<FileVersion>> {
        self.fs.history(path.as_ref())
    }

    /// Copies the content of one file to another.
    ///
    /// This method will **overwrite** the content of `to`.
    ///
    /// `from` and `to` must be absolute paths to regular files.
    ///
    /// If `from` and `to` both point to the same file, this method is no-op.
    ///
    /// This method is **not** atomic.
    #[inline]
    pub fn copy<P: AsRef<Utf8Path>, Q: AsRef<Utf8Path>>(&mut self, from: P, to: Q) -> Result<()> {
        self.fs.copy(from.as_ref(), to.as_ref())
    }

    /// Copies a directory to another recursively.
    ///
    /// This method will **overwrite** the content of files in `to` with
    /// the files in `from` which have same relative location.
    ///
    /// `from` and `to` must be absolute paths to directories.
    ///
    /// If `to` is not empty, the entire directory tree of `from` will be
    /// merged to `to`.
    ///
    /// This method will stop if any errors happened.
    ///
    /// If `from` and `to` both point to the same directory, this method is
    /// no-op.
    ///
    /// This method is **not** atomic.
    #[inline]
    pub fn copy_dir_all<P: AsRef<Utf8Path>, Q: AsRef<Utf8Path>>(
        &mut self,
        from: P,
        to: Q,
    ) -> Result<()> {
        self.fs.copy_dir_all(from.as_ref(), to.as_ref())
    }

    /// Removes a regular file from the repository.
    ///
    /// `path` must be an absolute path.
    ///
    /// This method is atomic.
    #[inline]
    pub fn remove_file<P: AsRef<Utf8Path>>(&mut self, path: P) -> Result<()> {
        self.fs.remove_file(path.as_ref())
    }

    /// Remove an existing empty directory.
    ///
    /// `path` must be an absolute path.
    ///
    /// This method is atomic.
    #[inline]
    pub fn remove_dir<P: AsRef<Utf8Path>>(&mut self, path: P) -> Result<()> {
        self.fs.remove_dir(path.as_ref())
    }

    /// Removes a directory at this path, after removing all its children.
    /// Use carefully!
    ///
    /// `path` must be an absolute path.
    ///
    /// This method is **not** atomic in whole, but removing each entry is
    /// atomic.
    #[inline]
    pub fn remove_dir_all<P: AsRef<Utf8Path>>(&mut self, path: P) -> Result<()> {
        self.fs.remove_dir_all(path.as_ref())
    }

    /// Rename a file or directory to a new name, replacing the original file
    /// if `to` already exists.
    ///
    /// `from` and `to` must be absolute paths.
    ///
    /// This method is atomic.
    #[inline]
    pub fn rename<P: AsRef<Utf8Path>, Q: AsRef<Utf8Path>>(&mut self, from: P, to: Q) -> Result<()> {
        self.fs.rename(from.as_ref(), to.as_ref())
    }

    /// Permanently destroy a repository specified by `uri`.
    ///
    /// This will permanently delete all files and directories in a repository
    /// regardless it is opened or not. Use it with caution.
    #[inline]
    pub fn destroy(&self) -> Result<()> {
        self.fs.destroy()
    }
}
