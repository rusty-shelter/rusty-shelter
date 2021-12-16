use crate::time::Time;
use serde::{Deserialize, Serialize};
use shelter_block::BlockId;
use std::time::SystemTime;

/// A representation of a permanent file content.
#[derive(Debug, Clone, Copy, PartialEq, Deserialize, Serialize)]
pub struct FileVersion {
    pub(super) id: BlockId,
    pub(super) version: usize,
    pub(super) len: usize,
    pub(super) ctime: Time,
}

impl FileVersion {
    /// Returns the version number of this content.
    ///
    /// The version number starts from 0 and continuously increases by 1.
    pub fn version(&self) -> usize {
        self.version
    }

    /// Returns the byte length of this version of content.
    pub fn len(&self) -> usize {
        self.len
    }

    /// Returns the creation time of this version of content.
    pub fn created_at(&self) -> SystemTime {
        self.ctime.to_system_time()
    }
}
