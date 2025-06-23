use std::{io::Cursor, path::Path};

use anyhow::Result;
use rabex::files::SerializedFile;
use rabex::objects::{ClassId, ClassIdType};
use rabex::tpk::TpkTypeTreeBlob;
use rabex::typetree::TypeTreeCache;

fn main() -> Result<()> {
    let tpk = TypeTreeCache::new(TpkTypeTreeBlob::embedded());

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
