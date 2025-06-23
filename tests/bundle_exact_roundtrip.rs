use std::io::Cursor;

use rabex::files::BundleFile;
use rabex::files::bundlefile::ExtractionConfig;
use rabex::files::bundlefile::{self, CompressionType};

#[test]
fn roundtrip_uncompressed_2020() -> Result<(), std::io::Error> {
    assert_roundtrip(
        "tests/files/uncompressed-2020.2.2f1.bundle",
        CompressionType::Lz4hc,
        CompressionType::None,
    )
}

#[test]
fn roundtrip_uncompressed_2022() -> Result<(), std::io::Error> {
    assert_roundtrip(
        "tests/files/uncompressed-2022.3.18f1.bundle",
        CompressionType::Lz4hc,
        CompressionType::None,
    )
}

#[test]
fn roundtrip_chunkbased_2020() -> Result<(), std::io::Error> {
    assert_roundtrip(
        "tests/files/chunkbased-2020.2.2f1.bundle",
        CompressionType::Lz4hc,
        CompressionType::Lz4hc,
    )
}

#[test]
fn roundtrip_chunkbased_2022() -> Result<(), std::io::Error> {
    assert_roundtrip(
        "tests/files/chunkbased-2022.3.18f1.bundle",
        CompressionType::Lz4hc,
        CompressionType::Lz4hc,
    )
}

#[test]
fn roundtrip_uncompressed_2scenes() -> Result<(), std::io::Error> {
    assert_roundtrip(
        "tests/files/uncompressed-2scenes-2022.3.18f1.bundle",
        CompressionType::Lz4hc,
        CompressionType::None,
    )
}
#[test]
fn roundtrip_chunkbased_2scenes() -> Result<(), std::io::Error> {
    assert_roundtrip(
        "tests/files/chunkbased-2scenes-2022.3.18f1.bundle",
        CompressionType::Lz4hc,
        CompressionType::Lz4hc,
    )
}

fn assert_roundtrip(
    path: &str,
    header_compression: CompressionType,
    compression: CompressionType,
) -> Result<(), std::io::Error> {
    println!("{path}");
    let data = std::fs::read(path)?;

    let config = ExtractionConfig::default();
    let bundle = BundleFile::from_reader(&mut Cursor::new(data.as_slice()), &config)?;

    let mut out = Cursor::new(Vec::new());
    bundlefile::write_bundle(
        &bundle.m_Header,
        &mut out,
        header_compression,
        compression,
        &bundle.m_DirectoryInfo,
        &bundle.m_BlockData,
    )?;

    let out = out.into_inner();

    // let start_at = out.iter().zip(&data).take_while(|&(x, y)| x == y).count();
    // eprintln!("Skipped {start_at} common bytes");
    // let start_at = start_at.max(38); // header with size
    let start_at = 0;
    let up_to = out.len().min(data.len());
    pretty_assertions::assert_eq!(&data[start_at..up_to], &out[start_at..up_to]);
    assert_eq!(data.len(), out.len());

    Ok(())
}
