use super::{File, FileSystem, FileType, OpenOptions};
use crate::error::{Error, Result};
use camino::Utf8Path;
use shelter_storage::Storage;

pub(super) fn open_file<S: Storage>(
    fs: &mut FileSystem<S>,
    path: &Utf8Path,
    open_options: OpenOptions,
) -> Result<File<S>> {
    // 1. Check for errors
    if fs.is_read_only()
        && (open_options.contains(OpenOptions::FILE_WRITE)
            || open_options.contains(OpenOptions::FILE_APPEND)
            || open_options.contains(OpenOptions::FILE_TRUNCATE)
            || open_options.contains(OpenOptions::FILE_CREATE)
            || open_options.contains(OpenOptions::FILE_CREATE_NEW))
    {
        return Err(Error::ReadOnly);
    }
    if fs.is_file_node_exist(path) && open_options.contains(OpenOptions::FILE_CREATE_NEW) {
        return Err(Error::AlreadyExists);
    }

    // 2. Create or open file node
    let file_node = if open_options.contains(OpenOptions::FILE_CREATE) {
        fs.create_fnode(path, FileType::File)?
    } else {
        fs.open_fnode(path)?
    };

    // 3. Create File handle
    // let pos = if open_opts.append {
    //     SeekFrom::Start(curr_len as u64)
    // } else {
    //     SeekFrom::Start(0)
    // };
    Ok(File::new(open_options, fs.storage.clone(), file_node))
}
