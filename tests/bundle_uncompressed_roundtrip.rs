use std::io::Cursor;

use rabex::{
    files::bundlefile::ExtractionConfig,
    files::{
        BundleFile,
        bundlefile::{self, CompressionType},
    },
};

#[test]
fn check_uncompressed_2020() -> Result<(), std::io::Error> {
    assert_roundtrip(
        "tests/files/uncompressed-2020.2.2f1.bundle",
        CompressionType::None,
    )
}

#[test]
fn check_uncompressed_2022() -> Result<(), std::io::Error> {
    assert_roundtrip(
        "tests/files/uncompressed-2022.3.18f1.bundle",
        CompressionType::None,
    )
}

#[test]
fn check_chunkbased_2020() -> Result<(), std::io::Error> {
    assert_roundtrip(
        "tests/files/chunkbased-2020.2.2f1.bundle",
        CompressionType::Lz4hc,
    )
}

#[test]
fn check_chunkbased_2022() -> Result<(), std::io::Error> {
    assert_roundtrip(
        "tests/files/chunkbased-2022.3.18f1.bundle",
        CompressionType::Lz4hc,
    )
}

#[test]
fn check_default_2020() -> Result<(), std::io::Error> {
    assert_roundtrip(
        "tests/files/default-2020.2.2f1.bundle",
        CompressionType::Lzma,
    )
}

#[test]
fn check_default_2022() -> Result<(), std::io::Error> {
    assert_roundtrip(
        "tests/files/default-2022.3.18f1.bundle",
        CompressionType::Lzma,
    )
}

#[test]
fn check_compression() -> Result<(), std::io::Error> {
    assert_roundtrip(
        "tests/files/default-2022.3.18f1.bundle",
        CompressionType::None,
    )?;
    assert_roundtrip(
        "tests/files/default-2022.3.18f1.bundle",
        CompressionType::Lzma,
    )?;
    assert_roundtrip(
        "tests/files/default-2022.3.18f1.bundle",
        CompressionType::Lz4,
    )?;
    assert_roundtrip(
        "tests/files/default-2022.3.18f1.bundle",
        CompressionType::Lz4hc,
    )?;
    Ok(())
}

fn assert_roundtrip(path: &str, compression: CompressionType) -> Result<(), std::io::Error> {
    println!("{path}");
    let data = std::fs::read(path)?;

    let config = ExtractionConfig::default();
    let bundle = BundleFile::from_reader(&mut Cursor::new(data), &config)?;

    let mut out = Cursor::new(Vec::new());
    bundlefile::write_bundle(
        &bundle.m_Header,
        &mut out,
        CompressionType::Lz4hc,
        compression,
        &bundle.m_DirectoryInfo,
        &bundle.m_BlockData,
    )?;

    out.set_position(0);
    let new = BundleFile::from_reader(&mut out, &config)?;
    assert_eq!(bundle.m_BlockData, new.m_BlockData);

    Ok(())
}
