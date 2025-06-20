use std::{io::Cursor, path::Path};

use anyhow::{Context, Result};
use byteorder::LittleEndian;
use rabex::files::SerializedFile;
use rabex::files::serialzedfile::Endianness;
use rabex::objects::ClassId;
use rabex::read_ext::ReadUrexExt;
use rabex::serde_typetree;
use rabex::tpk::TpkTypeTreeBlob;
use rayon::iter::{IntoParallelRefIterator, ParallelIterator};

#[test]
fn roundtrip_typetree() -> Result<()> {
    let tt = TpkTypeTreeBlob::embedded();

    // let dir = "/home/jakob/.local/share/Steam/steamapps/common/Hollow Knight/hollow_knight_Data";
    // let dir = "/home/jakob/.local/share/Steam/steamapps/common/Nine Sols-Speedrunpatch/NineSols_Data";
    let dir = "tests/files/hk_serialized";

    let paths = std::fs::read_dir(dir)?
        .filter_map(|e| {
            let entry = match e {
                Ok(entry) => entry,
                Err(e) => return Some(Err(e)),
            };
            let name = entry.file_name();
            let name = name.to_str().unwrap();
            name.starts_with("level").then_some(entry.path()).map(Ok)
        })
        .collect::<Result<Vec<_>, std::io::Error>>()?;

    let error = paths
        .par_iter()
        .map(|level| {
            if let Err(e) = assert_roundtrip_all(level, &tt).context(level.display().to_string()) {
                eprintln!("{e:?}");
                true
            } else {
                false
            }
        })
        .collect::<Vec<_>>()
        .into_iter()
        .any(|x| x);

    assert!(!error);

    Ok(())
}

fn assert_roundtrip_all(path: &Path, tts: &TpkTypeTreeBlob) -> Result<()> {
    println!("{}", path.display());
    let data = std::fs::read(path)?;

    let reader = &mut Cursor::new(data.as_slice());
    let serialized = SerializedFile::from_reader(reader)?;
    assert_eq!(serialized.m_Header.m_Endianess, Endianness::Little);

    for object in serialized.m_Objects.values() {
        let tt = tts
            .get_typetree_node(object.m_ClassID, serialized.m_UnityVersion.unwrap())
            .unwrap();

        if object.m_ClassID == ClassId::MonoBehaviour {
            continue; // TODO
        }

        reader.set_position(object.m_Offset as u64);
        let raw = reader.read_bytes_sized(object.m_Size as usize)?;
        reader.set_position(object.m_Offset as u64);
        let value = tt
            .read::<serde_json::Value, _, LittleEndian>(reader)
            .with_context(|| format!("deserializing {:?} {}", object.m_ClassID, object.m_PathID))?;
        // println!("[Try] {} {:?}", object.m_PathID, object.m_ClassID);
        // println!("[TT]  {}", tt.dump());
        // println!("[Val] {:#?}", value);

        let out = serde_typetree::to_vec::<_, LittleEndian>(&value, &tt)
            .with_context(|| format!("serializing {:?} {}", object.m_ClassID, object.m_PathID))?;
        let up_to = raw.len().min(out.len());
        pretty_assertions::assert_eq!(raw[..up_to], out[..up_to]);
        assert_eq!(raw.len(), out.len());
        // println!("[OK]  {:?}", object.m_ClassID);
    }

    Ok(())
}
