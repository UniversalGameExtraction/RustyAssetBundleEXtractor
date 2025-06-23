use std::io::{Read, Seek, Write};

use crate::files::bundlefile::{self, BundleFileHeader, CompressionType};
use crate::files::unityfile::FileEntry;
use crate::unity_version::UnityVersion;

/// Builder for ergonomically creating a bundlefile.
///
/// # Example
/// ```rust,no_run
/// use std::fs::File;
/// use std::io::{BufReader, BufWriter};
/// use std::path::Path;
///
/// use anyhow::Result;
/// use rabex::files::bundlefile::builder::BundleFileBuilder;
/// use rabex::files::bundlefile::CompressionType;
///
/// fn main() -> Result<()> {
///     let mut builder = BundleFileBuilder::unityfs(7, "2020.2.2f1".parse().unwrap());
///
///     builder.add_file("File1", BufReader::new(File::open("file1")?))?;
///     builder.add_file("File1.sharedAssets", BufReader::new(File::open("file1-assets")?))?;
///
///     let mut out = BufWriter::new(File::open("out.unity3d")?);
///     builder.write(&mut out, CompressionType::None)?;
///
///     Ok(())
/// }
/// ```
pub struct BundleFileBuilder {
    version: u32,
    unity_version: UnityVersion,
    header_compression: CompressionType,

    uncompressed: Vec<u8>,
    dir_infos: Vec<FileEntry>,
}
impl BundleFileBuilder {
    pub fn unityfs(version: u32, unity_version: UnityVersion) -> BundleFileBuilder {
        BundleFileBuilder {
            version,
            unity_version,
            header_compression: CompressionType::Lz4hc,
            uncompressed: Vec::new(),
            dir_infos: Vec::new(),
        }
    }

    pub fn set_header_compression(&mut self, compression: CompressionType) {
        self.header_compression = compression;
    }

    pub fn add_file(&mut self, path: &str, mut file: impl Read) -> Result<(), std::io::Error> {
        let offset = self.uncompressed.len();
        let len = std::io::copy(&mut file, &mut self.uncompressed)?;
        self.dir_infos.push(FileEntry {
            offset: offset as i64,
            size: len as i64,
            flags: 4,
            path: path.to_owned(),
        });

        Ok(())
    }

    pub fn write(
        self,
        mut writer: impl Write + Seek,
        compression: CompressionType,
    ) -> Result<(), std::io::Error> {
        let header = BundleFileHeader {
            signature: bundlefile::BundleSignature::UnityFS,
            version: self.version,
            unity_version: "5.x.x".to_owned(),
            unity_revision: Some(self.unity_version),
            size: 0, // unused
        };

        header.write(&mut writer)?;
        bundlefile::write_fs(
            &header,
            writer,
            self.header_compression,
            compression,
            &self.dir_infos,
            &self.uncompressed,
        )
    }
}
