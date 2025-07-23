use std::{io::Cursor, path::Path};

use rabex::files::{SerializedFile, serializedfile};
use rabex::tpk::{TpkFile, TpkTypeTreeBlob};

#[test]
#[cfg(false)]
fn roundtrip_serialized_hk_gamefiles() -> Result<(), std::io::Error> {
    let mut file = std::fs::File::open("lz4.tpk")?;
    let tpk_file = TpkFile::from_reader(&mut file).unwrap();
    let tt = tpk_file.as_type_tree().unwrap().unwrap();

    let hk_dir = "/home/jakob/.local/share/Steam/steamapps/common/Hollow Knight/hollow_knight_Data";

    for entry in std::fs::read_dir(hk_dir)? {
        let entry = entry?;
        let filename = entry.file_name();
        let filename = filename.to_str().unwrap();
        if !filename.starts_with("level") {
            continue;
        }
        assert_roundtrip(&entry.path(), &tt)?;
    }

    Ok(())
}

#[test]
#[cfg(false)]
fn roundtrip_serialized_ninesols_gamefiles() -> Result<(), std::io::Error> {
    let mut file = std::fs::File::open("lz4.tpk")?;
    let tpk_file = TpkFile::from_reader(&mut file).unwrap();
    let tt = tpk_file.as_type_tree().unwrap().unwrap();

    let ninesols_dir = Path::new(
        "/home/jakob/.local/share/Steam/steamapps/common/Nine Sols-Speedrunpatch/NineSols_Data",
    );

    for entry in std::fs::read_dir(ninesols_dir)? {
        let entry = entry?;
        let filename = entry.file_name();
        let filename = filename.to_str().unwrap();
        if !filename.starts_with("level") {
            continue;
        }
        assert_roundtrip(&entry.path(), &tt)?;
    }

    Ok(())
}

#[test]
fn roundtrip_serialized_unitypy() -> Result<(), std::io::Error> {
    let mut file = std::fs::File::open("lz4.tpk")?;
    let tpk_file = TpkFile::from_reader(&mut file).unwrap();
    let tt = tpk_file.as_type_tree().unwrap().unwrap();

    for entry in std::fs::read_dir("tests/files/hk_serialized")? {
        let entry = entry?;
        assert_roundtrip(&entry.path(), &tt)?;
    }

    Ok(())
}

fn assert_roundtrip(path: &Path, tt: &TpkTypeTreeBlob) -> Result<(), std::io::Error> {
    println!("{}", path.display());
    let data = std::fs::read(path)?;

    let reader = &mut Cursor::new(data.as_slice());
    let serialized = SerializedFile::from_reader(reader)?;

    let mut out = Cursor::new(Vec::new());

    let common_offset_map =
        serializedfile::build_common_offset_map(tt, serialized.m_UnityVersion.unwrap());
    serializedfile::write_serialized(&mut out, &serialized, reader.get_ref(), &common_offset_map)?;

    let start = 0;
    let up_to = out.get_ref().len().min(data.len());
    pretty_assertions::assert_eq!(&data[start..up_to], &out.get_ref()[start..up_to]);

    // unity aligns at begin, unitypy at end of loop
    // assert_eq!(data.len(), out.get_ref().len());

    Ok(())
}
