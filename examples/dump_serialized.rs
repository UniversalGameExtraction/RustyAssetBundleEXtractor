use anyhow::Result;
use rabex::{files::SerializedFile, tpk::TpkFile};
use std::fs::File;

fn main() -> Result<()> {
    let path = std::env::args()
        .nth(1)
        .ok_or_else(|| anyhow::anyhow!("Expected path to unity bundle argument"))?;
    let mut data = File::open(path)?;

    let mut tpk_file = File::open("lz4.tpk").map_err(|_| {
        anyhow::anyhow!("missing lz4.tpk file, download from https://github.com/AssetRipper/Tpk")
    })?;
    let tpk_file = TpkFile::from_reader(&mut tpk_file)?;
    let tpk = tpk_file.as_type_tree()?.unwrap();

    let file = SerializedFile::from_reader(&mut data)?;
    for obj in file.objects() {
        println!("{} {:?}", obj.m_PathID, obj.m_ClassID);
        let value = file.read::<serde_json::Value>(obj, &tpk, &mut data)?;

        println!("{value:#?}");
    }

    Ok(())
}
