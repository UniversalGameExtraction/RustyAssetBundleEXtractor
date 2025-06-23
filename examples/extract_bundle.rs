use anyhow::Result;
use rabex::files::bundlefile::BundleFileReader;
use rabex::files::bundlefile::ExtractionConfig;
use std::fs::File;
use std::io::{Cursor, Read, Seek};
use std::path::Path;
use std::time::Instant;

fn main() -> Result<()> {
    let mut args = std::env::args().skip(1);
    let out_dir = args
        .next()
        .ok_or_else(|| anyhow::anyhow!("Expected path for out directory"))?;
    let path = args
        .next()
        .ok_or_else(|| anyhow::anyhow!("Expected path to unity bundle argument"))?;

    let file = File::open(path)?;
    let reader = Cursor::new(unsafe { memmap2::Mmap::map(&file)? });

    let start = Instant::now();
    let (n_files, size) = extract(reader, Path::new(&out_dir))?;

    println!(
        "Extracted {n_files} files ({}) in {:.2}s",
        friendly_size(size),
        start.elapsed().as_secs_f32()
    );

    Ok(())
}

fn extract(reader: impl Read + Seek, out_dir: &Path) -> Result<(usize, usize)> {
    let mut stats = (0, 0);

    let config = ExtractionConfig::default();
    let mut bundle = BundleFileReader::from_reader(reader, &config)?;

    while let Some(mut file) = bundle.next() {
        if file.path.ends_with(".resS") {
            continue;
        }
        println!("Extracting {} ({})", file.path, friendly_size(file.size));

        stats.0 += 1;
        stats.1 += file.size;

        let path = out_dir.join(&file.path);
        std::fs::DirBuilder::new()
            .recursive(true)
            .create(path.parent().unwrap())?;
        std::fs::write(&path, file.read()?)?;
    }
    Ok(stats)
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
