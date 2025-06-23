use anyhow::Result;
use rabex::{files::SerializedFile, tpk::TpkTypeTreeBlob, typetree::TypeTreeCache};
use std::fs::File;

fn main() -> Result<()> {
    let path = std::env::args()
        .nth(1)
        .ok_or_else(|| anyhow::anyhow!("Expected path to unity bundle argument"))?;
    let mut data = File::open(path)?;

    let tpk = TypeTreeCache::new(TpkTypeTreeBlob::embedded());

    let file = SerializedFile::from_reader(&mut data)?;
    for obj in file.objects() {
        println!("{} {:?}", obj.m_PathID, obj.m_ClassID);
        let value = file.read::<serde_json::Value>(obj, &tpk, &mut data)?;

        println!("{value:#?}");
    }

    Ok(())
}
