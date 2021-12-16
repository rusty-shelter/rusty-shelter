use serde::{Deserialize, Serialize};

/// A structure representing a type of file with accessors for each file type.
#[derive(Default, Debug, Copy, Clone, PartialEq, Deserialize, Serialize)]
pub enum FileType {
    #[default]
    File,
    Dir,
}

impl FileType {
    /// Test whether this file type represents a regular file.
    pub fn is_file(self) -> bool {
        self == FileType::File
    }

    /// Test whether this file type represents a directory.
    pub fn is_dir(self) -> bool {
        self == FileType::Dir
    }
}

impl From<FileType> for i32 {
    fn from(file_type: FileType) -> i32 {
        match file_type {
            FileType::File => 0,
            FileType::Dir => 1,
        }
    }
}

impl From<FileType> for String {
    fn from(file_type: FileType) -> String {
        match file_type {
            FileType::File => String::from("File"),
            FileType::Dir => String::from("Dir"),
        }
    }
}
