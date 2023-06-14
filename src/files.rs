mod bundlefile;
mod serialzedfile;
mod unityfile;
mod webfile;

pub use bundlefile::BundleFile;
// pub use serialzedfile::SerializedFile;
// pub use webfile::WebFile;
pub use unityfile::UnityFile;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
