use std::fs::File;

use anyhow::Result;
use rabex::{objects::ClassId, tpk::TpkFile};

fn main() -> Result<()> {
    let mut args = std::env::args().skip(1);
    let version = args.next();

    let mut tpk_file = File::open("lz4.tpk").map_err(|_| {
        anyhow::anyhow!("missing lz4.tpk file, download from https://github.com/AssetRipper/Tpk")
    })?;
    let tpk_file = TpkFile::from_reader(&mut tpk_file)?;
    let tpk = tpk_file.as_type_tree()?.unwrap();

    let version = version
        .map(|v| v.parse())
        .transpose()?
        .unwrap_or(*tpk.versions.last().unwrap());
    let ty = tpk.get_typetree_node(ClassId::GameObject, version).unwrap();
    println!("{}", ty.dump());

    Ok(())
}
