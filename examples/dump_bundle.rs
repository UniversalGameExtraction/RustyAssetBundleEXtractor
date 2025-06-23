use anyhow::Result;
use rabex::files::SerializedFile;
use rabex::files::bundlefile::BundleFileReader;
use rabex::files::bundlefile::ExtractionConfig;
use std::fs::File;
use std::io::Cursor;

fn main() -> Result<()> {
    let path = std::env::args()
        .nth(1)
        .ok_or_else(|| anyhow::anyhow!("Expected path to unity bundle argument"))?;
    let config = ExtractionConfig::default();

    let file = File::open(path)?;
    let mut reader = Cursor::new(unsafe { memmap2::Mmap::map(&file)? });

    let mut bundle = BundleFileReader::from_reader(&mut reader, &config)?;

    let header = bundle.header();
    println!(
        "{:?} {} {}",
        header.signature,
        header.version,
        header.unity_revision.unwrap(),
    );

    let mut size_all = 0;
    let mut size_main = 0;

    while let Some(mut file) = bundle.next() {
        println!("- {}", file.path);
        println!("  Size: {}", friendly_size(file.size));

        size_all += file.size;
        if !file.path.ends_with("resS") {
            size_main += file.size;
            let mut data = Cursor::new(file.read()?);
            let serialized = SerializedFile::from_reader(&mut data)?;
            println!("  Object Count: {}", serialized.objects().len());
        }
    }

    println!("Size (total): {}", friendly_size(size_all));
    println!("Size (excluding resources): {}", friendly_size(size_main));

    Ok(())
}

fn friendly_size(size: usize) -> String {
    const UNITS: [&str; 5] = ["B", "KiB", "MiB", "GiB", "TiB"];
    let mut size = size as f64;
    let mut unit = 0;

    while size >= 1024.0 && unit < UNITS.len() - 1 {
        size /= 1024.0;
        unit += 1;
    }

    if unit == 0 {
        format!("{}{}", size as usize, UNITS[unit])
    } else {
        format!("{:.2}{}", size, UNITS[unit])
    }
}
