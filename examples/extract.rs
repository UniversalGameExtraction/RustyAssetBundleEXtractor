use anyhow::Result;
use rabex::files::bundlefile::BundleFileReader;
use rabex::tpk::TpkTypeTreeBlob;
use rabex::typetree::TypeTreeCache;
use std::fs::{DirBuilder, File};
use std::io::{Cursor, Write};
use std::path::Path;

use rabex::files::SerializedFile;
use rabex::files::bundlefile::ExtractionConfig;

fn main() -> Result<()> {
    let tpk = TypeTreeCache::new(TpkTypeTreeBlob::embedded());

    let path =
        "/home/jakob/.local/share/Steam/steamapps/common/Nine Sols/NineSols_Data/data.unity3d";
    let export_dir = Path::new("dump");
    let file = File::open(path)?;

    use memmap2::Mmap;
    let mut reader = Cursor::new(unsafe { Mmap::map(&file)? });

    // parse the bundle file
    let config = ExtractionConfig::default();
    let mut iter = BundleFileReader::from_reader(&mut reader, &config)?;

    // iterate over the files in the bundle
    while let Some(mut directory) = iter.next() {
        println!("- {}", directory.path);
        // generate export dir for cab
        let export_cab_dir = export_dir.join(&directory.path);

        if directory.path.ends_with("resS") {
            continue;
        }
        let mut data = Cursor::new(directory.read()?);

        // try to parse the file as a SerializedFile
        let serialized = SerializedFile::from_reader(&mut data)?;
        // iterate over objects
        for info in serialized.objects() {
            let object = serialized.get_object::<serde_json::Value>(info.m_PathID, &tpk)?;

            let name = format!("{}", info.m_PathID);

            // ensure that the parent directory exists
            let dst_path = export_cab_dir.join(name);
            DirBuilder::new()
                .recursive(true)
                .create(dst_path.parent().unwrap())?;

            let json = object.read(&mut data)?;
            File::create(format!("{}.json", dst_path.to_string_lossy()))?
                .write_all(serde_json::to_string_pretty(&json)?.as_bytes())?;
        }
    }
    Ok(())
}
