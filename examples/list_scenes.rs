use std::{fs::File, io::Cursor, path::Path};

use anyhow::Result;
use rabex::files::SerializedFile;
use rabex::objects::{ClassId, ClassIdType};
use rabex::tpk::TpkFile;

fn main() -> Result<()> {
    let mut tpk_file = File::open("lz4.tpk").map_err(|_| {
        anyhow::anyhow!("missing lz4.tpk file, download from https://github.com/AssetRipper/Tpk")
    })?;
    let tpk_file = TpkFile::from_reader(&mut tpk_file)?;
    let tpk = tpk_file.as_type_tree()?.unwrap();

    let path = Path::new(
        "/home/jakob/.local/share/Steam/steamapps/common/Nine Sols-Speedrunpatch/NineSols_Data/globalgamemanagers",
    );
    let reader = &mut Cursor::new(std::fs::read(path)?);

    let ggm = SerializedFile::from_reader(reader)?;

    let build_settings = ggm
        .find_object_of::<BuildSettings>(&tpk)?
        .unwrap()
        .read(reader)?;

    for (i, scene) in build_settings.scenes.iter().enumerate() {
        let name = Path::new(&scene).file_stem().unwrap().display();
        println!("level{i} - {name}");
    }

    Ok(())
}

#[derive(Debug, serde_derive::Deserialize)]
pub struct BuildSettings {
    pub scenes: Vec<String>,
}
impl ClassIdType for BuildSettings {
    const CLASS_ID: ClassId = ClassId::BuildSettings;
}
