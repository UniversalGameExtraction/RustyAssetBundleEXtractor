use std::{collections::HashMap, io::Cursor};

use anyhow::Result;
use rabex::files::serializedfile::{build_common_offset_map, builder::SerializedFileBuilder};
use rabex::objects::{ClassId, ClassIdType, PPtr};
use rabex::tpk::TpkTypeTreeBlob;
use rabex::typetree::typetree_cache::TypeTreeCache;
use serde_derive::Serialize;

fn main() -> Result<()> {
    let bundle_name = "new_bundle";
    let unity_version = "2022.2.2f1".parse().unwrap();
    let tpk = TypeTreeCache::new(TpkTypeTreeBlob::embedded());
    let common_offset_map = build_common_offset_map(&tpk.inner, unity_version);

    let mut sharedassets = SerializedFileBuilder::new(unity_version, &tpk, &common_offset_map);

    let container = ["A", "B"]
        .iter()
        .map(|scene| {
            let path = format!("unity-scene-repacker/{scene}.unity");
            (path, AssetInfo::default())
        })
        .collect();

    sharedassets.add_object(&PreloadData {
        m_Assets: vec![PPtr {
            m_FileID: 1,
            m_PathID: 10001,
        }],
        ..Default::default()
    })?;

    sharedassets.add_object(&AssetBundle {
        m_Name: bundle_name.to_owned(),
        m_Container: container,
        m_MainAsset: AssetInfo::default(),
        m_RuntimeCompatibility: 1,
        m_IsStreamedSceneAssetBundle: true,
        m_PathFlags: 7,
        ..Default::default()
    })?;

    let mut out = Cursor::new(Vec::new());
    sharedassets.write(&mut out)?;

    Ok(())
}

#[derive(Debug, Serialize, Default)]
#[allow(non_snake_case)]
pub struct PreloadData {
    pub m_Name: String,
    pub m_Assets: Vec<PPtr>,
    pub m_Dependencies: Vec<String>,
    pub m_ExplicitDataLayout: bool,
}
impl ClassIdType for PreloadData {
    const CLASS_ID: ClassId = ClassId::PreloadData;
}

#[derive(Debug, Serialize, Default)]
#[allow(non_snake_case)]
pub struct AssetBundle {
    pub m_Name: String,
    pub m_PreloadTable: Vec<PPtr>,
    pub m_Container: HashMap<String, AssetInfo>,
    pub m_MainAsset: AssetInfo,
    pub m_RuntimeCompatibility: u32,
    pub m_AssetBundleName: String,
    pub m_Dependencies: Vec<String>,
    pub m_IsStreamedSceneAssetBundle: bool,
    pub m_ExplicitDataLayout: i32,
    pub m_PathFlags: i32,
    pub m_SceneHashes: HashMap<String, String>,
}
impl ClassIdType for AssetBundle {
    const CLASS_ID: ClassId = ClassId::AssetBundle;
}

#[derive(Debug, Serialize, Default)]
#[allow(non_snake_case)]
pub struct AssetInfo {
    pub preloadIndex: i32,
    pub preloadSize: i32,
    pub asset: PPtr,
}
