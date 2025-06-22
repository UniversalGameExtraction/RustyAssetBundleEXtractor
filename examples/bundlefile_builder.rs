use std::fs::File;
use std::io::{BufReader, BufWriter};
use std::path::Path;

use anyhow::Result;
use rabex::files::bundlefile::CompressionType;
use rabex::files::bundlefile::builder::BundleFileBuilder;

fn main() -> Result<()> {
    let mut builder = BundleFileBuilder::unityfs(7, "2020.2.2f1".parse().unwrap());

    let base = Path::new("tests/files/hk_serialized/");
    let files = [
        "BuildPlayer-bundle_Fungus1_12.sharedAssets",
        "BuildPlayer-bundle_Fungus1_12",
        "BuildPlayer-bundle_Fungus1_19.sharedAssets",
        "BuildPlayer-bundle_Fungus1_19",
        "BuildPlayer-bundle_White_Palace_01.sharedAssets",
        "BuildPlayer-bundle_White_Palace_01",
        "BuildPlayer-bundle_GG_Workshop.sharedAssets",
        "BuildPlayer-bundle_GG_Workshop",
        "BuildPlayer-bundle_Dream_Final_Boss.sharedAssets",
        "BuildPlayer-bundle_Dream_Final_Boss",
        "BuildPlayer-bundle_Mines_31.sharedAssets",
        "BuildPlayer-bundle_Mines_31",
        "BuildPlayer-bundle_White_Palace_03_hub.sharedAssets",
        "BuildPlayer-bundle_White_Palace_03_hub",
        "BuildPlayer-bundle_Dream_Room_Believer_Shrine.sharedAssets",
        "BuildPlayer-bundle_Dream_Room_Believer_Shrine",
        "BuildPlayer-bundle_GG_Radiance.sharedAssets",
        "BuildPlayer-bundle_GG_Radiance",
        "BuildPlayer-bundle_GG_Atrium.sharedAssets",
        "BuildPlayer-bundle_GG_Atrium",
        "BuildPlayer-bundle_Tutorial_01.sharedAssets",
        "BuildPlayer-bundle_Tutorial_01",
        "BuildPlayer-bundle_Abyss_10.sharedAssets",
        "BuildPlayer-bundle_Abyss_10",
        "BuildPlayer-bundle_Waterways_05.sharedAssets",
        "BuildPlayer-bundle_Waterways_05",
        "BuildPlayer-bundle_Mines_28.sharedAssets",
        "BuildPlayer-bundle_Mines_28",
    ];
    for path in files {
        let p = base.join(path);
        builder.add_file(path, BufReader::new(File::open(p)?))?;
    }

    let mut out = BufWriter::new(File::create("out.unity3d")?);
    builder.write(&mut out, CompressionType::None)?;

    Ok(())
}
