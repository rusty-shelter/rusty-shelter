//! shelter-fs is a zero-details, privacy-focused file system.
//!
//! It keeps your app files securely, privately and reliably on underlying
//! storages. By encapsulating files and directories into an encrypted
//! repository, it provides a virtual file system and exclusive access to
//! the authorised application.
//!
//! The most core parts of this module are [`Repo`] and [`File`], which provides
//! most API for file system operations and file data I/O.
//!
//! - [`Repo`] provides similar file system manipulation methods to [`std::fs`]
//! - [`File`] provides similar file I/O methods to [`std::fs::File`]
//!
//! # Examples
//!
//! Create and open a [`Repository`] using memory as underlying storage.
//!
//! TODO[epic=doc,seq=21] Intro documentation
//!
//! [`std::fs`]: https://doc.rust-lang.org/std/fs/index.html
//! [`std::fs::File`]: https://doc.rust-lang.org/std/fs/struct.File.html
//! [`Read`]: https://doc.rust-lang.org/std/io/trait.Read.html
//! [`Write`]: https://doc.rust-lang.org/std/io/trait.Write.html
//! [`Path`]: https://doc.rust-lang.org/std/path/struct.Path.html
//! [`PathBuf`]: https://doc.rust-lang.org/std/path/struct.PathBuf.html
//! [`Repo`]: struct.Repo.html
//! [`File`]: struct.File.html

#![forbid(unsafe_code)]
// #![warn(missing_debug_implementations)]
// #![deny(missing_docs)]

#[macro_use]
extern crate bitflags;
extern crate camino;
extern crate crdt_tree;
extern crate data_encoding;
extern crate fast_cdc;
// extern crate machine_uid;
extern crate orion;
extern crate serde;
extern crate serde_derive;
extern crate serde_with;
extern crate shelter_block;
extern crate shelter_storage;
extern crate thiserror;

mod error;
mod filesystem;
mod repository;
mod time;

// External API
pub use error::Error;
pub use filesystem::FileSystemOptions;
pub use repository::Repository;
