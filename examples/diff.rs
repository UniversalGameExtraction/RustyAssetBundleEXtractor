use std::{
    fs::File,
    io::{Cursor, Read, Seek},
};

use anyhow::Result;
use rabex::{
    files::{BundleFile, SerializedFile, bundlefile::ExtractionConfig},
    tpk::TpkTypeTreeBlob,
    typetree::{TypeTreeCache, TypeTreeProvider},
};

fn main() -> Result<()> {
    let tpk = TypeTreeCache::new(TpkTypeTreeBlob::embedded());

    let config = ExtractionConfig::default();

    let mut args = std::env::args().skip(1);
    let a = args.next().unwrap();
    let b = args.next().unwrap();

    let mut a = File::open(a)?;
    let mut b = File::open(b)?;
    let bundle_a = BundleFile::from_reader(&mut a, &config)?;
    let bundle_b = BundleFile::from_reader(&mut b, &config)?;

    assert_eq!(
        bundle_a
            .m_DirectoryInfo
            .iter()
            .map(|x| &x.path)
            .collect::<Vec<_>>(),
        bundle_b
            .m_DirectoryInfo
            .iter()
            .map(|x| &x.path)
            .collect::<Vec<_>>()
    );

    for (entry_a, entry_b) in bundle_a
        .m_DirectoryInfo
        .iter()
        .zip(bundle_b.m_DirectoryInfo.iter())
        .collect::<Vec<_>>()
    {
        println!("{}", entry_a.path);
        if entry_a.path.ends_with(".sharedAssets") {
            continue;
        }

        let a_reader = &mut Cursor::new(
            &bundle_a.m_BlockData
                [entry_a.offset as usize..entry_a.offset as usize + entry_a.size as usize],
        );
        let b_reader = &mut Cursor::new(
            &bundle_b.m_BlockData
                [entry_b.offset as usize..entry_b.offset as usize + entry_b.size as usize],
        );

        let serialized_a = SerializedFile::from_reader(a_reader)?;
        let serialized_b = SerializedFile::from_reader(b_reader)?;
        diff_serialized(&tpk, &serialized_a, a_reader, &serialized_b, b_reader)?;
    }

    Ok(())
}

fn diff_serialized(
    tpk: impl TypeTreeProvider,
    serialized_a: &SerializedFile,
    mut a_reader: impl Read + Seek,
    serialized_b: &SerializedFile,
    mut b_reader: impl Read + Seek,
) -> Result<()> {
    pretty_assertions::assert_eq!(
        format!("{:#?}", serialized_a),
        format!("{:#?}", serialized_b)
    );

    for (a, b) in serialized_a.objects().zip(serialized_b.objects()) {
        assert_eq!(a.m_PathID, b.m_PathID);

        let val_a = serialized_a.read::<serde_json::Value>(a, &tpk, &mut a_reader)?;
        let val_b = serialized_b.read::<serde_json::Value>(b, &tpk, &mut b_reader)?;

        pretty_assertions::assert_eq!(val_a, val_b, "in {:?} {}", a.m_ClassID, a.m_PathID);
    }

    Ok(())
}
