//! Support for reading and writing unity files.
pub mod bundlefile;
pub mod serializedfile;
pub mod unityfile;
mod webfile;

pub use bundlefile::BundleFile;
pub use serializedfile::SerializedFile;
// pub use webfile::WebFile;
pub use unityfile::UnityFile;
