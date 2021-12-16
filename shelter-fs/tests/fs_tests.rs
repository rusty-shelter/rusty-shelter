extern crate shelter_fs;
extern crate shelter_storage;

use camino::Utf8Path;
use shelter_fs::{FileSystemOptions, Repository};
use shelter_storage::{FileSystem, Storage, XChaCha};
use std::io::{Read, Write};
use std::path::Path;

#[test]
fn main() -> Result<(), std::io::Error> {
    // 1. Create storage struct
    let crypto = XChaCha::default();
    let file_storage = FileSystem::new(Path::new("./sandbox"), crypto, 1024);

    // 2. Create repository
    let mut repo = Repository::new(FileSystemOptions::REPO_VERSIONED, file_storage);

    // 3. Init repository storage
    repo.init("Jagu", "sengern");

    // 3. Create a file
    let file_path = Utf8Path::new("/test.txt");
    let mut file = repo.create_file(&file_path);

    // 4. Write into file
    // let content = "bonjour maman";
    // file.write(content.as_bytes())?;
    // file.flush()?;

    // drop(file);

    // 5.0 Read history
    // let history = repo.history(&file_path);
    // println!("History {:?}", history);

    // 5. Read file again
    // let mut buf = Vec::<u8>::new();
    // let mut file = repo.open_file(&file_path);
    // file.read_to_end(&mut buf)?;
    // let content2 = std::str::from_utf8(&buf).unwrap();

    // assert_eq!(content, content2);

    // // 6. Create directory
    // repo.create_dir_all(Utf8Path::new("/test")).unwrap();
    // repo.create_dir_all(Utf8Path::new("/hallo")).unwrap();
    // repo.create_dir_all(Utf8Path::new("/boom")).unwrap();

    // // 7. Read directory
    // let dirs = repo.read_dir(Utf8Path::new("/"));
    // println!("dirs {:?}", dirs);

    Ok(())
}
