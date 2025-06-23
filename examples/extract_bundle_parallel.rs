use anyhow::Result;
use rabex::files::bundlefile::BundleFileReader;
use rabex::files::bundlefile::ExtractionConfig;
use rayon::iter::IntoParallelIterator;
use rayon::iter::ParallelIterator;
use std::fs::File;
use std::io::Cursor;
use std::path::PathBuf;
use std::sync::atomic::AtomicU32;
use std::sync::atomic::AtomicUsize;
use std::sync::atomic::Ordering;
use std::time::Instant;

fn main() -> Result<()> {
    let mut args = std::env::args().skip(1);
    let out_dir = args
        .next()
        .map(PathBuf::from)
        .ok_or_else(|| anyhow::anyhow!("Expected path for out directory"))?;
    let path = args
        .next()
        .map(PathBuf::from)
        .ok_or_else(|| anyhow::anyhow!("Expected path to unity bundle argument"))?;

    let file = File::open(path)?;
    let mmap = unsafe { memmap2::Mmap::map(&file)? };

    let config = ExtractionConfig::default();
    let bundle = BundleFileReader::from_reader(Cursor::new(&*mmap), &config)?;

    let start = Instant::now();

    let mut n_files = AtomicU32::new(0);
    let mut total_size = AtomicUsize::new(0);

    bundle
        .files()
        .into_par_iter()
        .try_for_each(|file| -> Result<_> {
            if file.path.ends_with("resS") {
                return Ok(());
            }
            println!(
                "Extracting {} ({})",
                file.path,
                friendly_size(file.size as usize)
            );

            let data = bundle.read_at_entry(file)?;

            n_files.fetch_add(1, Ordering::Relaxed);
            total_size.fetch_add(data.len(), Ordering::Relaxed);

            let path = out_dir.join(&file.path);
            std::fs::DirBuilder::new()
                .recursive(true)
                .create(path.parent().unwrap())?;
            std::fs::write(&path, data)?;

            Ok(())
        })?;

    println!(
        "Extracted {} files ({}) in {:.2}s",
        n_files.get_mut(),
        friendly_size(*total_size.get_mut()),
        start.elapsed().as_secs_f32()
    );

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
