#![allow(non_snake_case)]
use anyhow::Result;
use rabex::{
    files::SerializedFile,
    objects::{ClassId, ClassIdType, TypedPPtr},
    tpk::TpkTypeTreeBlob,
    typetree::{TypeTreeCache, TypeTreeProvider},
};
use rustc_hash::FxHashMap;
use serde_derive::{Deserialize, Serialize};
use std::{
    fs::File,
    io::{Read, Seek},
};

fn main() -> Result<()> {
    let path = std::env::args()
        .nth(1)
        .ok_or_else(|| anyhow::anyhow!("Expected path to unity bundle argument"))?;
    let data = &mut File::open(path)?;

    let tpk = &TypeTreeCache::new(TpkTypeTreeBlob::embedded());

    let file = SerializedFile::from_reader(data)?;
    let transforms = file
        .objects_of::<Transform>(tpk)?
        .map(|obj| Ok((obj.info.m_PathID, obj.read(data)?)))
        .collect::<Result<FxHashMap<_, _>>>()?;

    for root in transforms
        .values()
        .filter(|transform| transform.m_Father.is_null())
    {
        print_object_hierarchy(root, &file, tpk, data, 0)?;
    }

    Ok(())
}

fn print_object_hierarchy(
    root: &Transform,
    file: &SerializedFile,
    tpk: &impl TypeTreeProvider,
    reader: &mut (impl Read + Seek),
    indent: usize,
) -> Result<()> {
    let go = root.m_GameObject.deref_local(file, tpk)?.read(reader)?;
    println!("{}{}", "  ".repeat(indent), go.m_Name);

    for child in &root.m_Children {
        let child = child.deref_local(file, tpk)?.read(reader)?;
        print_object_hierarchy(&child, file, tpk, reader, indent + 1)?;
    }
    Ok(())
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Transform {
    pub m_GameObject: TypedPPtr<GameObject>,
    // pub m_LocalRotation: (f32, f32, f32, f32),
    // pub m_LocalPosition: (f32, f32, f32),
    // pub m_LocalScale: (f32, f32, f32),
    pub m_Children: Vec<TypedPPtr<Transform>>,
    pub m_Father: TypedPPtr<Transform>,
}
impl ClassIdType for Transform {
    const CLASS_ID: ClassId = ClassId::Transform;
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GameObject {
    // pub m_Component: Vec<ComponentPair>,
    // pub m_Layer: u32,
    pub m_Name: String,
    // pub m_Tag: u16,
    // pub m_IsActive: bool,
}
impl ClassIdType for GameObject {
    const CLASS_ID: ClassId = ClassId::GameObject;
}
