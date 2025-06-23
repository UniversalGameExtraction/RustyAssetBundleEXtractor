//! A crate for working with Unity Engine asset files.
//! It supports reading and writing [bundle files](files::bundlefile) and [serialized files](files::serializedfile),
//! as well as reading typetrees with [serde integration](serde_typetree).
//!
//! It also contains support for type tree dumps in [tpk].
#![allow(non_snake_case)]
#![allow(dead_code)]
mod commonstring;
pub mod files;
pub mod objects;
pub mod serde_typetree;
pub mod tpk;
pub mod typetree;
mod unity_version;

mod read_ext;
mod write_ext;

mod archive_storage_manager;

pub use unity_version::UnityVersion;
