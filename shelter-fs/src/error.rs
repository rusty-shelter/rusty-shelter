use std::{io::Error as IoError, result};
use thiserror::Error;

/// The error type for operations with [`Repository`] and [`File`].
///
/// [`Repository`]: struct.Repo.html
/// [`File`]: struct.File.html
#[derive(Error, Debug)]
pub enum Error {
    #[error("Invalid argument")]
    InvalidArgument,

    #[error("Invalid path")]
    InvalidPath,

    #[error("File not found")]
    NotFound,

    #[error("File already exists")]
    AlreadyExists,

    #[error("File is root")]
    IsRoot,

    #[error("Path is directory")]
    IsDir,

    #[error("Path is file")]
    IsFile,

    #[error("Path is not a directory")]
    NotDir,

    #[error("Path is not a file")]
    NotFile,

    #[error("Directory is not empty")]
    NotEmpty,

    #[error("File has no version")]
    NoVersion,

    #[error("Opened as read only")]
    ReadOnly,

    #[error("Cannot read file")]
    CannotRead,

    #[error("Cannot write file")]
    CannotWrite,

    #[error("File does not write ye")]
    NotWrite,

    #[error("File does not finish yet")]
    NotFinish,

    #[error("File is closed")]
    Closed,

    #[error("IoError")]
    Io {
        #[from]
        source: IoError,
    },
}

/// A specialized [`Result`] type for Shelter fs operations.
///
/// See the [`Error`] for all the  errors.
///
/// [`Result`]: https://doc.rust-lang.org/std/result/enum.Result.html
/// [`Error`]: enum.Error.html
pub type Result<T> = result::Result<T, Error>;
