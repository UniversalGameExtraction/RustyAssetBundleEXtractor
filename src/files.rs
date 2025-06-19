mod bundlefile;
pub mod serialzedfile;
mod unityfile;
mod webfile;

pub use bundlefile::BundleFile;
pub use serialzedfile::{ObjectHandler, SerializedFile};
// pub use webfile::WebFile;
pub use unityfile::UnityFile;
