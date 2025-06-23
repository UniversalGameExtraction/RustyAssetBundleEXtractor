//! The main unity data file.
//!
//! - To read the file, use [`SerializedFile::from_reader`].
//! - To create a new serialized file from scratch, use [`builder::SerializedFileBuilder`]
//!
//! # Example
//!
//! ```rust,no_run
//! # #![allow(non_snake_case)]
//! use anyhow::Result;
//! use rabex::files::SerializedFile;
//! use rabex::objects::{ClassId, ClassIdType, PPtr, TypedPPtr};
//! use rabex::{tpk::TpkTypeTreeBlob, typetree::TypeTreeCache};
//! use serde_derive::{Deserialize, Serialize};
//! use std::{fs::File, io::BufReader};
//!
//! fn main() -> Result<()> {
//!     let path = std::env::args()
//!         .nth(1)
//!         .ok_or_else(|| anyhow::anyhow!("Expected path to unity bundle argument"))?;
//!     let reader = &mut BufReader::new(File::open(path)?);
//!     let tpk = &TypeTreeCache::new(TpkTypeTreeBlob::embedded());
//!     let file = SerializedFile::from_reader(reader)?;
//!
//!     for transform_obj in file.objects_of::<Transform>(tpk)? {
//!         let transform = transform_obj.read(reader)?;
//!
//!         if transform.m_Father.is_null() {
//!             let go = transform
//!                 .m_GameObject
//!                 .deref_local(&file, tpk)?
//!                 .read(reader)?;
//!             println!("Root object: {}", go.m_Name);
//!         }
//!     }
//!
//!     Ok(())
//! }
//!
//! #[derive(Deserialize)]
//! pub struct Transform {
//!     pub m_GameObject: TypedPPtr<GameObject>,
//!     pub m_LocalRotation: (f32, f32, f32, f32),
//!     pub m_LocalPosition: (f32, f32, f32),
//!     pub m_LocalScale: (f32, f32, f32),
//!     pub m_Children: Vec<TypedPPtr<Transform>>,
//!     pub m_Father: TypedPPtr<Transform>,
//! }
//! impl ClassIdType for Transform {
//!     const CLASS_ID: ClassId = ClassId::Transform;
//! }
//!
//! #[derive(Deserialize)]
//! pub struct GameObject {
//!     pub m_Component: Vec<ComponentPair>,
//!     pub m_Layer: u32,
//!     pub m_Name: String,
//!     pub m_Tag: u16,
//!     pub m_IsActive: bool,
//! }
//!
//! #[derive(Deserialize)]
//! pub struct ComponentPair {
//!     pub component: PPtr,
//! }
//! ```
pub mod builder;

use std::borrow::Cow;
use std::collections::HashMap;
use std::io::{Read, Seek, Write};
use std::marker::PhantomData;

use super::UnityFile;
use super::bundlefile::ExtractionConfig;
use crate::objects::pptr::PathId;
use crate::objects::{ClassId, ClassIdType, PPtr};
use crate::read_ext::{ReadSeekUrexExt, ReadUrexExt};
use crate::serde_typetree;
use crate::tpk::TpkTypeTreeBlob;
use crate::typetree::{TypeTreeNode, TypeTreeProvider};
use crate::unity_version::UnityVersion;
use crate::write_ext::WriteExt;
use crate::write_ext::WriteSeekExt;
use bitflags::bitflags;
use byteorder::WriteBytesExt;
use byteorder::{BigEndian, ByteOrder, LittleEndian, ReadBytesExt};

use num_enum::TryFromPrimitive;
use rustc_hash::FxHashMap;
use serde::Deserialize;

pub type Result<T, E = Error> = std::result::Result<T, E>;

#[derive(Clone, Copy, Debug, TryFromPrimitive, PartialEq, Eq)]
#[repr(u8)]
pub enum Endianness {
    Little = 0,
    Big = 1,
}

#[derive(Debug, Copy, Clone)]
pub struct SerializedFileHeader {
    pub m_MetadataSize: u32,
    pub m_FileSize: i64,
    pub m_Version: u32,
    pub m_DataOffset: i64,
    pub m_Endianess: Endianness,
    pub m_Reserved: [u8; 3],
    pub unknown: i64,
}
impl SerializedFileHeader {
    fn from_reader<T: std::io::Read + std::io::Seek>(
        reader: &mut T,
    ) -> Result<SerializedFileHeader, std::io::Error> {
        let mut header = SerializedFileHeader {
            m_MetadataSize: reader.read_u32::<BigEndian>()?,
            m_FileSize: reader.read_u32::<BigEndian>()? as i64,
            m_Version: reader.read_u32::<BigEndian>()?,
            m_DataOffset: reader.read_u32::<BigEndian>()? as i64,
            m_Endianess: Endianness::Little,
            m_Reserved: [0, 0, 0],
            unknown: 0,
        };

        let endianness = if header.m_Version >= 9 {
            let endianness = reader.read_u8()?;
            header.m_Reserved = reader.read_bytes_array::<3>()?;
            endianness
        } else {
            reader.seek(std::io::SeekFrom::Current(
                header.m_FileSize - header.m_MetadataSize as i64,
            ))?;
            reader.read_u8()?
        };
        header.m_Endianess = endianness.try_into().map_err(|_| {
            std::io::Error::new(std::io::ErrorKind::InvalidData, "Invalid Endianness")
        })?;

        if header.m_Version >= 22 {
            header.m_MetadataSize = reader.read_u32::<BigEndian>()?;
            header.m_FileSize = reader.read_i64::<BigEndian>()?;
            header.m_DataOffset = reader.read_i64::<BigEndian>()?;
            header.unknown = reader.read_i64::<BigEndian>()?; // unknown
        };
        Ok(header)
    }

    fn write<W: Write>(&self, writer: &mut W) -> Result<(), std::io::Error> {
        if self.m_Version < 9 {
            todo!()
        }

        if self.m_Version >= SerializedFileFormatVersion::LARGE_FILES_SUPPORT.bits() {
            writer.write_u32::<BigEndian>(0)?;
            writer.write_u32::<BigEndian>(0)?;
            writer.write_u32::<BigEndian>(self.m_Version)?;
            writer.write_u32::<BigEndian>(0)?;
            writer.write_u8(self.m_Endianess as u8)?;
            writer.write_u8(0)?; // reserved
            writer.write_u8(0)?;
            writer.write_u8(0)?;
            writer.write_u32::<BigEndian>(self.m_MetadataSize)?;
            writer.write_i64::<BigEndian>(self.m_FileSize)?;
            writer.write_i64::<BigEndian>(self.m_DataOffset)?;
            writer.write_i64::<BigEndian>(self.unknown)?;
        } else {
            writer.write_u32::<BigEndian>(self.m_MetadataSize)?;
            writer.write_u32::<BigEndian>(self.m_FileSize as u32)?;
            writer.write_u32::<BigEndian>(self.m_Version)?;
            writer.write_u32::<BigEndian>(self.m_DataOffset as u32)?;
            todo!();
        }
        Ok(())
    }
}

#[derive(Debug, Clone)]
pub struct SerializedType {
    pub m_ClassID: ClassId,
    pub m_IsStrippedType: bool,
    pub m_ScriptTypeIndex: i16,
    pub m_ScriptID: [u8; 16],
    pub m_OldTypeHash: [u8; 16],
    /// Only set if [`SerializedFile::m_EnableTypeTree`] is `true`.
    pub m_Type: Option<TypeTreeNode>,
    // for reftypes
    pub m_ClassName: Option<String>,
    pub m_NameSpace: Option<String>,
    pub m_AsmName: Option<String>,
    // for non ref-types
    pub m_TypeDependencies: Vec<i32>,
}

impl SerializedType {
    pub fn simple(class_id: ClassId, typetree: Option<TypeTreeNode>) -> SerializedType {
        SerializedType {
            m_ClassID: class_id,
            m_IsStrippedType: false,
            m_ScriptTypeIndex: -1,
            m_ScriptID: [0; 16],
            m_OldTypeHash: [0; 16],
            m_Type: typetree,
            m_ClassName: None,
            m_NameSpace: None,
            m_AsmName: None,
            m_TypeDependencies: Vec::new(),
        }
    }
    pub fn from_reader<T: std::io::Read + std::io::Seek, B: ByteOrder>(
        reader: &mut T,
        header: &SerializedFileHeader,
        m_EnableTypeTree: bool,
        isRefType: bool,
    ) -> Result<SerializedType, std::io::Error> {
        let mut typ = SerializedType {
            m_ClassID: ClassId::UnknownType,
            m_IsStrippedType: false,
            m_ScriptTypeIndex: -1,
            m_ScriptID: [0; 16],
            m_OldTypeHash: [0; 16],
            m_Type: None,
            m_ClassName: None,
            m_NameSpace: None,
            m_AsmName: None,
            m_TypeDependencies: Vec::new(),
        };
        typ.m_ClassID = ClassId(reader.read_i32::<B>()?);

        if header.m_Version >= SerializedFileFormatVersion::REFACTORED_CLASS_ID.bits() {
            typ.m_IsStrippedType = reader.read_bool()?;
        }

        if header.m_Version >= SerializedFileFormatVersion::REFACTOR_TYPE_DATA.bits() {
            typ.m_ScriptTypeIndex = reader.read_i16::<B>()?;
        }

        if header.m_Version >= SerializedFileFormatVersion::HAS_TYPE_TREE_HASHES.bits() {
            if (isRefType && typ.m_ScriptTypeIndex >= 0)
                || ((header.m_Version < SerializedFileFormatVersion::REFACTORED_CLASS_ID.bits()
                    && typ.m_ClassID.0 < 0)
                    || (header.m_Version
                        >= SerializedFileFormatVersion::REFACTORED_CLASS_ID.bits()
                        && typ.m_ClassID == ClassId::MonoBehaviour))
            {
                typ.m_ScriptID = reader.read_bytes_array::<16>()?;
            }
            typ.m_OldTypeHash = reader.read_bytes_array::<16>()?;
        }

        if m_EnableTypeTree {
            if header.m_Version >= SerializedFileFormatVersion::UNKNOWN_12.bits()
                || header.m_Version == SerializedFileFormatVersion::UNKNOWN_10.bits()
            {
                typ.m_Type = Some(TypeTreeNode::blob_from_reader::<T, B>(
                    reader,
                    header.m_Version,
                )?);
            } else {
                typ.m_Type = Some(TypeTreeNode::from_reader::<T, B>(reader, header.m_Version)?);
            }
            if header.m_Version >= SerializedFileFormatVersion::STORES_TYPE_DEPENDENCIES.bits() {
                if isRefType {
                    typ.m_ClassName = Some(reader.read_cstr()?);
                    typ.m_NameSpace = Some(reader.read_cstr()?);
                    typ.m_AsmName = Some(reader.read_cstr()?);
                } else {
                    typ.m_TypeDependencies = reader.read_i32_array::<B>(None)?;
                }
            }
        }

        Ok(typ)
    }

    fn write<W: Write, B: ByteOrder>(
        &self,
        mut writer: W,
        version: u32,
        is_ref_type: bool,
        enable_type_tree: bool,
        common_offset_map: &HashMap<&str, u32>,
    ) -> Result<(), std::io::Error> {
        writer.write_i32::<B>(self.m_ClassID.0)?;

        if version >= 16 {
            writer.write_u8(self.m_IsStrippedType as u8)?;
        }
        if version >= 17 {
            writer.write_i16::<B>(self.m_ScriptTypeIndex)?;
        }
        if version >= 13 {
            if is_ref_type && self.m_ScriptTypeIndex >= 0
                || version < 16 && self.m_ClassID.0 < 0
                || version >= 16 && self.m_ClassID == ClassId::MonoBehaviour
            {
                writer.write_all(&self.m_ScriptID)?;
            }
            writer.write_all(&self.m_OldTypeHash)?;
        }

        if enable_type_tree {
            let ty = self.m_Type.as_ref().unwrap();
            if version >= 12 || version == 10 {
                ty.write_blob::<_, B>(writer.by_ref(), version, common_offset_map)?;
            } else {
                todo!()
            }

            if version >= 21 {
                if is_ref_type {
                    writer.write_cstr(self.m_ClassName.as_ref().unwrap())?;
                    writer.write_cstr(self.m_NameSpace.as_ref().unwrap())?;
                    writer.write_cstr(self.m_AsmName.as_ref().unwrap())?;
                } else {
                    writer.write_i32::<B>(self.m_TypeDependencies.len() as i32)?;
                    for &type_dependency in &self.m_TypeDependencies {
                        writer.write_i32::<B>(type_dependency)?;
                    }
                }
            }
        }

        Ok(())
    }
}

#[derive(Debug, Clone)]
pub struct LocalSerializedObjectIdentifier {
    pub m_LocalSerializedFileIndex: i32,
    pub m_LocalIdentifierInFile: i64,
}

impl LocalSerializedObjectIdentifier {
    pub fn from_reader<T: std::io::Read + std::io::Seek, B: ByteOrder>(
        reader: &mut T,
        header: &SerializedFileHeader,
    ) -> Result<LocalSerializedObjectIdentifier, std::io::Error> {
        Ok(LocalSerializedObjectIdentifier {
            m_LocalSerializedFileIndex: reader.read_i32::<B>()?,
            m_LocalIdentifierInFile: if header.m_Version
                < SerializedFileFormatVersion::UNKNOWN_14.bits()
            {
                reader.read_i32::<B>()? as i64
            } else {
                reader.read_i64::<B>()?
            },
        })
    }
}

#[derive(Debug, Clone, Default)]
pub struct ObjectInfo {
    pub m_PathID: i64,
    pub m_Offset: i64,
    pub m_Size: u32,
    pub m_TypeID: i32,
    pub m_ClassID: ClassId,
    pub m_IsDestroyed: Option<u16>,
    pub m_ScriptTypeIndex: Option<i16>,
    pub m_Stripped: Option<u8>,
}
impl ObjectInfo {
    pub fn as_local_pptr(&self) -> PPtr {
        PPtr {
            m_FileID: 0,
            m_PathID: self.m_PathID,
        }
    }

    fn from_reader<T: std::io::Read + std::io::Seek, B: ByteOrder>(
        reader: &mut T,
        header: &SerializedFileHeader,
        bigIDEnabled: Option<i32>,
        types: &[SerializedType],
    ) -> Result<ObjectInfo, std::io::Error> {
        let mut objectInfo = ObjectInfo {
            m_PathID: 0,
            m_Offset: 0,
            m_Size: 0,
            m_TypeID: 0,
            m_ClassID: ClassId(0),
            m_IsDestroyed: None,
            m_ScriptTypeIndex: None,
            m_Stripped: None,
        };
        if bigIDEnabled.is_some_and(|enabled| enabled > 0) {
            objectInfo.m_PathID = reader.read_i64::<B>()?;
        } else if header.m_Version < 14 {
            objectInfo.m_PathID = reader.read_i32::<B>()? as i64;
        } else {
            reader.align(4)?;
            objectInfo.m_PathID = reader.read_i64::<B>()?;
        }

        if header.m_Version >= SerializedFileFormatVersion::LARGE_FILES_SUPPORT.bits() {
            objectInfo.m_Offset = reader.read_i64::<B>()?;
        } else {
            objectInfo.m_Offset = reader.read_u32::<B>()? as i64;
        }
        objectInfo.m_Offset += header.m_DataOffset;
        objectInfo.m_Size = reader.read_u32::<B>()?;
        objectInfo.m_TypeID = reader.read_i32::<B>()?;
        if header.m_Version < SerializedFileFormatVersion::REFACTORED_CLASS_ID.bits() {
            objectInfo.m_ClassID = ClassId(reader.read_u16::<B>()? as i32);
        } else {
            objectInfo.m_ClassID = types
                .get(objectInfo.m_TypeID as usize)
                .ok_or_else(|| {
                    std::io::Error::new(
                        std::io::ErrorKind::InvalidData,
                        "reference to undefined type",
                    )
                })?
                .m_ClassID;
        }
        if header.m_Version < SerializedFileFormatVersion::HAS_SCRIPT_TYPE_INDEX.bits() {
            objectInfo.m_IsDestroyed = Some(reader.read_u16::<B>()?);
        }
        if header.m_Version >= SerializedFileFormatVersion::HAS_SCRIPT_TYPE_INDEX.bits()
            && header.m_Version < SerializedFileFormatVersion::REFACTOR_TYPE_DATA.bits()
        {
            objectInfo.m_ScriptTypeIndex = Some(reader.read_i16::<B>()?);
            // if objectInfo.serializedType != null
            //     objectInfo.serializedType.m_ScriptTypeIndex = m_ScriptTypeIndex;
        }
        if header.m_Version == SerializedFileFormatVersion::SUPPORTS_STRIPPED_OBJECT.bits()
            || header.m_Version == SerializedFileFormatVersion::REFACTORED_CLASS_ID.bits()
        {
            objectInfo.m_Stripped = Some(reader.read_u8()?);
        }

        Ok(objectInfo)
    }
}

#[derive(Debug, Clone)]
pub struct ScriptType {
    pub localSerializedFileIndex: i32,
    pub localIdentifierInFile: i64,
}

impl ScriptType {
    pub fn from_reader<T: std::io::Read + std::io::Seek, B: ByteOrder>(
        reader: &mut T,
        header: &SerializedFileHeader,
    ) -> Result<ScriptType, std::io::Error> {
        Ok(ScriptType {
            localSerializedFileIndex: reader.read_i32::<B>()?,
            localIdentifierInFile: if header.m_Version
                < SerializedFileFormatVersion::UNKNOWN_14.bits()
            {
                reader.read_i32::<B>()? as i64
            } else {
                reader.read_i64::<B>()?
            },
        })
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FileIdentifier {
    pub tempEmpty: Option<String>,
    pub guid: Option<Guid>,
    pub typeId: Option<i32>,
    pub pathName: String,
}

#[derive(Clone, PartialEq, Eq)]
pub struct Guid(pub [u8; 16]);

impl Guid {
    pub fn is_zero(&self) -> bool {
        self.0.iter().all(|&x| x == 0)
    }
}

impl std::fmt::Debug for Guid {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("Guid(")?;
        for byte in &self.0 {
            write!(f, "{byte:02x}")?;
        }
        f.write_str(")")
    }
}

impl FileIdentifier {
    pub fn from_reader<T: std::io::Read + std::io::Seek, B: ByteOrder>(
        reader: &mut T,
        header: &SerializedFileHeader,
    ) -> Result<FileIdentifier, std::io::Error> {
        Ok(FileIdentifier {
            tempEmpty: if header.m_Version >= SerializedFileFormatVersion::UNKNOWN_6.bits() {
                Some(reader.read_cstr()?)
            } else {
                None
            },
            guid: if header.m_Version >= SerializedFileFormatVersion::UNKNOWN_5.bits() {
                Some(Guid(reader.read_bytes_array::<16>()?))
            } else {
                None
            },
            typeId: if header.m_Version >= SerializedFileFormatVersion::UNKNOWN_5.bits() {
                Some(reader.read_i32::<B>()?)
            } else {
                None
            },
            pathName: reader.read_cstr()?,
        })
    }
}

/// A handle to an object, equipped with a [`TypeTreeNode`] for (de)serialization.
///
/// `T` can either a specific type for e.g. `Transform`s, or something like `serde_json::Value` for
/// dynamic deserialization.
pub struct ObjectRef<'a, T> {
    pub file: &'a SerializedFile,
    pub info: &'a ObjectInfo,
    pub tt: Cow<'a, TypeTreeNode>,

    marker: PhantomData<T>,
}

impl<'a, T> ObjectRef<'a, T> {
    pub fn new(file: &'a SerializedFile, info: &'a ObjectInfo, tt: Cow<'a, TypeTreeNode>) -> Self {
        ObjectRef {
            file,
            info,
            tt,
            marker: PhantomData,
        }
    }

    /// Read and deserialize the object.
    pub fn read<'de>(&self, mut reader: &mut (impl Read + Seek)) -> Result<T>
    where
        T: Deserialize<'de>,
    {
        reader.seek(std::io::SeekFrom::Start(self.info.m_Offset as u64))?;
        serde_typetree::from_reader_endianed(&mut reader, &self.tt, self.file.m_Header.m_Endianess)
            .map_err(Error::Deserialize)
    }

    pub fn get_raw_data(&self, reader: &mut (impl Read + Seek)) -> Result<Vec<u8>, std::io::Error> {
        reader.seek(std::io::SeekFrom::Start(self.info.m_Offset as u64))?;
        reader.read_bytes_sized(self.info.m_Size as usize)
    }

    /// Specify another typetree to be used for the handle. This is useful to replace a `ObjectRef<MonoBehaviour>`
    /// with a specific typetree for a certain monoscript.
    pub fn with_typetree<U>(&self, typetree: &'a TypeTreeNode) -> ObjectRef<'a, U> {
        ObjectRef {
            file: self.file,
            info: self.info,
            tt: Cow::Borrowed(typetree),
            marker: PhantomData,
        }
    }
    pub fn cast<U>(&'a self) -> ObjectRef<'a, U> {
        ObjectRef {
            file: self.file,
            info: self.info,
            tt: Cow::Borrowed(&self.tt),
            marker: PhantomData,
        }
    }
}

#[derive(Debug)]
pub struct ObjectHandler<'a, R: std::io::Read + std::io::Seek> {
    pub info: &'a ObjectInfo,
    pub typ: Option<&'a SerializedType>,
    pub file: &'a SerializedFile,
    pub reader: &'a mut R,
}

impl<'a, R: std::io::Read + std::io::Seek> ObjectHandler<'a, R> {
    pub fn new(
        info: &'a ObjectInfo,
        typ: Option<&'a SerializedType>,
        file: &'a SerializedFile,
        reader: &'a mut R,
    ) -> Self {
        ObjectHandler {
            info,
            typ,
            file,
            reader,
        }
    }

    pub fn get_raw_data(&mut self) -> Result<Vec<u8>, std::io::Error> {
        self.reader
            .seek(std::io::SeekFrom::Start(self.info.m_Offset as u64))?;
        self.reader.read_bytes_sized(self.info.m_Size as usize)
    }

    fn get_typetree(&self) -> Option<&TypeTreeNode> {
        match self.typ {
            Some(typ) => typ.m_Type.as_ref(),
            _ => None,
        }
    }

    pub fn peek_name(&mut self) -> Result<String, std::io::Error> {
        self.reader
            .seek(std::io::SeekFrom::Start(self.info.m_Offset as u64))?;

        todo!("not implemented yet");
        // todo - check against typeid
        /*match self.file.m_Header.m_Endianess {
            Endianness::Little => self.reader.read_string::<LittleEndian>(),
            Endianness::Big => self.reader.read_string::<BigEndian>(),
        }*/
    }

    pub fn parse<'de, T: serde::de::Deserialize<'de>>(
        &mut self,
    ) -> Result<T, serde_typetree::Error> {
        match self.get_typetree().cloned() {
            Some(node) => {
                self.reader
                    .seek(std::io::SeekFrom::Start(self.info.m_Offset as u64))?;
                match self.file.m_Header.m_Endianess {
                    Endianness::Little => node.read::<T, R, LittleEndian>(self.reader),
                    Endianness::Big => node.read::<T, R, BigEndian>(self.reader),
                }
            }
            _ => Err(serde_typetree::Error::custom("Couldn't find typetree")),
        }
    }

    #[cfg(feature = "formats")]
    pub fn parse_as_json(&mut self) -> Result<serde_json::Value, serde_typetree::Error> {
        self.parse::<serde_json::Value>()
    }

    #[cfg(feature = "formats")]
    pub fn parse_as_yaml(&mut self) -> Result<serde_yaml::Value, serde_typetree::Error> {
        self.parse::<serde_yaml::Value>()
    }

    /// Parses the object as msgpack
    #[cfg(feature = "formats")]
    pub fn parse_as_msgpack(&mut self) -> Result<Vec<u8>, serde_typetree::Error> {
        match self.get_typetree().cloned() {
            Some(node) => {
                self.reader
                    .seek(std::io::SeekFrom::Start(self.info.m_Offset as u64))?;
                match self.file.m_Header.m_Endianess {
                    Endianness::Little => node.read_as_msgpack::<R, LittleEndian>(self.reader),
                    Endianness::Big => node.read_as_msgpack::<R, BigEndian>(self.reader),
                }
            }
            _ => Err(serde_typetree::Error::custom("No Typetree foun")),
        }
    }
}

#[derive(Debug, Clone)]
pub struct SerializedFile {
    pub m_Header: SerializedFileHeader,
    pub m_UnityVersion: Option<UnityVersion>,
    pub m_TargetPlatform: Option<i32>,
    pub m_EnableTypeTree: bool,
    pub m_bigIDEnabled: Option<i32>,
    pub m_Types: Vec<SerializedType>,
    m_Objects: Vec<ObjectInfo>,
    // PERF: 20% faster in end-to-end benchmark compared to BTreeMap<i64, ObjectInfo>
    m_Objects_lookup: FxHashMap<i64, usize>,
    pub m_ScriptTypes: Option<Vec<LocalSerializedObjectIdentifier>>,
    /// Determines what external files [`PPtr`]s in the file can refer to.
    pub m_Externals: Vec<FileIdentifier>,
    pub m_RefTypes: Option<Vec<SerializedType>>,
    pub m_UserInformation: Option<String>,
}

impl SerializedFile {
    pub fn from_reader<T: std::io::Read + std::io::Seek>(
        reader: &mut T,
    ) -> Result<SerializedFile, std::io::Error> {
        let header = SerializedFileHeader::from_reader::<T>(reader)?;

        match header.m_Endianess {
            Endianness::Little => {
                SerializedFile::from_reader_endianed::<T, LittleEndian>(reader, header)
            }
            Endianness::Big => SerializedFile::from_reader_endianed::<T, BigEndian>(reader, header),
        }
    }

    fn from_reader_endianed<T, B>(
        reader: &mut T,
        header: SerializedFileHeader,
    ) -> Result<SerializedFile, std::io::Error>
    where
        T: std::io::Read + std::io::Seek,
        B: ByteOrder,
    {
        // Read Metadata
        let mut m_UnityVersion = None;
        if header.m_Version >= 9 {
            m_UnityVersion = Some(reader.read_cstr()?.parse().unwrap());
            // SetVersion(unity_version);
        }

        let mut m_TargetPlatform = None;
        if header.m_Version >= 10 {
            m_TargetPlatform = Some(reader.read_i32::<B>()?);
            // if !Enum.IsDefined(typeof(BuildTarget, m_TargetPlatform))
            // {
            //     m_TargetPlatform = BuildTarget.UnknownPlatform;
            // }
        }

        let mut m_EnableTypeTree = false;
        if header.m_Version >= 11 {
            m_EnableTypeTree = reader.read_bool()?;
        }

        // Read Types
        let typeCount = reader.read_i32::<B>()?;
        let m_Types: Vec<SerializedType> = (0..typeCount)
            .map(|_| SerializedType::from_reader::<T, B>(reader, &header, m_EnableTypeTree, false))
            .collect::<Result<Vec<_>, _>>()?;

        let m_bigIDEnabled = None;
        if header.m_Version >= SerializedFileFormatVersion::UNKNOWN_7.bits()
            && header.m_Version < SerializedFileFormatVersion::UNKNOWN_14.bits()
        {
            let _bigIDEnabled = Some(reader.read_i32::<B>()?);
        }

        // Read Objects
        let objectCount = reader.read_i32::<B>()?;
        let m_Objects: Vec<ObjectInfo> = (0..objectCount)
            .map(|_| ObjectInfo::from_reader::<T, B>(reader, &header, m_bigIDEnabled, &m_Types))
            .collect::<Result<_, _>>()?;

        let mut m_ScriptTypes = None;
        if header.m_Version >= SerializedFileFormatVersion::HAS_SCRIPT_TYPE_INDEX.bits() {
            let scriptCount = reader.read_i32::<B>()?;
            m_ScriptTypes = Some(
                (0..scriptCount)
                    .map(|_| LocalSerializedObjectIdentifier::from_reader::<T, B>(reader, &header))
                    .collect::<Result<_, _>>()?,
            );
        }

        let externalsCount = reader.read_i32::<B>()?;
        let m_Externals = (0..externalsCount)
            .map(|_| FileIdentifier::from_reader::<T, B>(reader, &header))
            .collect::<Result<_, _>>()?;

        let mut m_RefTypes = None;
        if header.m_Version >= SerializedFileFormatVersion::SUPPORTS_REF_OBJECT.bits() {
            let refTypesCount = reader.read_i32::<B>()?;
            m_RefTypes = Some(
                (0..refTypesCount)
                    .map(|_| {
                        SerializedType::from_reader::<T, B>(reader, &header, m_EnableTypeTree, true)
                    })
                    .collect::<Result<_, _>>()?,
            );
        }

        let mut m_UserInformation = None;
        if header.m_Version >= SerializedFileFormatVersion::UNKNOWN_5.bits() {
            m_UserInformation = Some(reader.read_cstr()?);
        }

        //reader.AlignStream(16);
        let mut file = SerializedFile {
            m_Header: header,
            m_UnityVersion,
            m_TargetPlatform,
            m_EnableTypeTree,
            m_bigIDEnabled,
            m_Types,
            m_Objects_lookup: FxHashMap::default(),
            m_Objects,
            m_ScriptTypes,
            m_Externals,
            m_RefTypes,
            m_UserInformation,
        };
        file.recompute_lookup();
        Ok(file)
    }
}

impl SerializedFile {
    /// List all object infos in the file
    pub fn objects(&self) -> impl ExactSizeIterator<Item = &ObjectInfo> {
        self.m_Objects.iter()
    }

    /// Mutate the object infos in the file.
    ///
    /// We don't expose unconstrained mutable access to maintain an index accelerating the lookup
    /// of path ids internally.
    pub fn modify_objects(&mut self, f: impl FnOnce(&mut Vec<ObjectInfo>)) {
        f(&mut self.m_Objects);
        self.recompute_lookup();
    }

    pub fn take_objects(&mut self) -> Vec<ObjectInfo> {
        self.m_Objects_lookup.clear();
        std::mem::take(&mut self.m_Objects)
    }

    fn recompute_lookup(&mut self) {
        self.m_Objects_lookup = self
            .m_Objects
            .iter()
            .enumerate()
            .map(|(i, obj)| (obj.m_PathID, i))
            .collect();
    }

    pub fn get_object_info(&self, path_id: i64) -> Option<&ObjectInfo> {
        self.m_Objects_lookup
            .get(&path_id)
            .map(|&i| &self.m_Objects[i])
    }

    pub fn get_object_handler<'a, R: std::io::Read + std::io::Seek>(
        &'a self,
        objectinfo: &'a ObjectInfo,
        reader: &'a mut R,
    ) -> ObjectHandler<'a, R> {
        let mut typ = None;
        if self.m_Header.m_Version >= SerializedFileFormatVersion::REFACTORED_CLASS_ID.bits() {
            typ = Some(&self.m_Types[objectinfo.m_TypeID as usize]);
        }

        ObjectHandler::new(objectinfo, typ, self, reader)
    }
}

impl SerializedFile {
    /// Iterate over all [`ObjectInfo`]s of type `T`
    pub fn object_infos_of<T>(&self) -> impl Iterator<Item = &ObjectInfo>
    where
        T: ClassIdType,
    {
        self.m_Objects
            .iter()
            .filter(move |object| object.m_ClassID == T::CLASS_ID)
    }

    /// Find the first [`ObjectInfo`] of type `T`
    pub fn find_object_info_of<T>(&self) -> Option<&ObjectInfo>
    where
        T: ClassIdType,
    {
        self.m_Objects
            .iter()
            .find(move |object| object.m_ClassID == T::CLASS_ID)
    }

    /// Get a reference to the object at `path_id`. The [`TypeTreeProvider`] is used for
    /// [`SerializedFile`]s without serialized type tree information.
    pub fn get_object<'a, T>(
        &'a self,
        path_id: PathId,
        tpk: &'a impl TypeTreeProvider,
    ) -> Result<ObjectRef<'a, T>> {
        let info = self
            .get_object_info(path_id)
            .ok_or(Error::NoObject(path_id))?;
        let tt = self.get_typetree_for(info, tpk)?;
        // breaks for Transform/RectTransform?
        // assert_eq!(info.m_ClassID, T::CLASS_ID, "get_object<{:?}> was actually {:?}", T::CLASS_ID, info.m_ClassID);
        Ok(ObjectRef::new(self, info, tt))
    }

    /// Iterate over all objects of type `T`
    pub fn objects_of<'a, T>(
        &'a self,
        tpk: &'a impl TypeTreeProvider,
    ) -> Result<impl Iterator<Item = ObjectRef<'a, T>>>
    where
        T: ClassIdType,
    {
        let ty = tpk
            .get_typetree_node(T::CLASS_ID, self.m_UnityVersion.unwrap())
            .ok_or(Error::NoTypetree(T::CLASS_ID))?;

        Ok(self.object_infos_of::<T>().map(move |info| {
            let tt = self
                .get_serialized_objectinfo_type(info)
                .map(Cow::Borrowed)
                .unwrap_or(ty.clone());
            ObjectRef::new(self, info, tt)
        }))
    }

    /// Find the first object of type `T`
    pub fn find_object_of<'a, T>(
        &'a self,
        tpk: &'a impl TypeTreeProvider,
    ) -> Result<Option<ObjectRef<'a, T>>>
    where
        T: ClassIdType,
    {
        self.find_object_info_of::<T>()
            .map(move |info| {
                let tt = self.get_typetree_for(info, tpk)?;
                Ok(ObjectRef::new(self, info, tt))
            })
            .transpose()
    }

    fn get_serialized_objectinfo_type(&self, object: &ObjectInfo) -> Option<&TypeTreeNode> {
        if self.m_Header.m_Version >= SerializedFileFormatVersion::REFACTORED_CLASS_ID.bits()
            && let Some(ty) = &self.m_Types[object.m_TypeID as usize].m_Type
        {
            return Some(ty);
        }
        None
    }

    /// Deserialize the object data as type `T`.
    ///
    /// Equivalent to
    /// ```rust,no_run
    /// # type T = ();
    /// # fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// # let file: rabex::files::SerializedFile = todo!();
    /// # let tpk: &rabex::tpk::TpkTypeTreeBlob = todo!();
    /// # let mut reader: &mut std::io::Cursor<Vec<u8>> = &mut Default::default();
    /// file.get_object::<T>(1234, tpk)?.read(reader)
    /// # ;Ok(())
    /// # }
    /// ```
    pub fn read<'de, T: Deserialize<'de>>(
        &self,
        object: &ObjectInfo,
        tpk: impl TypeTreeProvider,
        reader: &mut (impl Read + Seek),
    ) -> Result<T> {
        let tt = self.get_typetree_for(object, &tpk)?;
        reader.seek(std::io::SeekFrom::Start(object.m_Offset as u64))?;
        serde_typetree::from_reader_endianed(reader, &tt, self.m_Header.m_Endianess)
            .map_err(Error::Deserialize)
    }

    /// Read the raw bytes of the object.
    pub fn read_raw(
        &self,
        object: &ObjectInfo,
        reader: &mut (impl Read + Seek),
    ) -> Result<Vec<u8>> {
        reader.seek(std::io::SeekFrom::Start(object.m_Offset as u64))?;
        let bytes = reader.read_bytes_sized(object.m_Size as usize)?;
        Ok(bytes)
    }

    pub fn get_typetree_for_class<'a>(
        &'a self,
        class_id: ClassId,
        tpk: &'a impl TypeTreeProvider,
    ) -> Result<Cow<'a, TypeTreeNode>> {
        tpk.get_typetree_node(class_id, self.m_UnityVersion.unwrap())
            .ok_or(Error::NoTypetree(class_id))
    }

    pub fn get_typetree_for<'a>(
        &'a self,
        info: &ObjectInfo,
        tpk: &'a impl TypeTreeProvider,
    ) -> Result<Cow<'a, TypeTreeNode>> {
        self.get_serialized_objectinfo_type(info)
            .map(Cow::Borrowed)
            .or_else(|| tpk.get_typetree_node(info.m_ClassID, self.m_UnityVersion.unwrap()))
            .ok_or(Error::NoTypetree(info.m_ClassID))
    }
}

#[derive(Debug)]
pub enum Error {
    NoTypetree(ClassId),
    Deserialize(serde_typetree::Error),
    Serialize(serde_typetree::Error),
    NoObject(PathId),
    IO(std::io::Error),
}
impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::NoTypetree(class_id) => {
                write!(f, "No typetree found for class {class_id:?}")
            }
            Error::Deserialize(error) => write!(f, "Error trying to deserialize: {error}"),
            Error::Serialize(error) => write!(f, "Error trying to serialize: {error}"),
            Error::IO(error) => write!(f, "IO error: {error}"),
            Error::NoObject(path_id) => {
                write!(f, "No object with path id {path_id} exists",)
            }
        }
    }
}
impl std::error::Error for Error {}

impl From<std::io::Error> for Error {
    fn from(error: std::io::Error) -> Self {
        Error::IO(error)
    }
}

impl UnityFile for SerializedFile {
    fn from_reader<T: std::io::Read + std::io::Seek>(
        reader: &mut T,
        _: &ExtractionConfig,
    ) -> Result<Self, std::io::Error>
    where
        Self: Sized,
    {
        SerializedFile::from_reader(reader)
    }
}

bitflags! {
    struct SerializedFileFormatVersion: u32 {
        const UNSUPPORTED = 1;
        const UNKNOWN_2 = 2;
        const UNKNOWN_3 = 3;
        /// 1.2.0 to 2.0.0
        const UNKNOWN_5 = 5;
        /// 2.1.0 to 2.6.1
        const UNKNOWN_6 = 6;
        /// 3.0.0b
        const UNKNOWN_7 = 7;
        /// 3.0.0 to 3.4.2
        const UNKNOWN_8 = 8;
        /// 3.5.0 to 4.7.2
        const UNKNOWN_9 = 9;
        /// 5.0.0aunk1
        const UNKNOWN_10 = 10;
        /// 5.0.0aunk2
        const HAS_SCRIPT_TYPE_INDEX = 11;
        /// 5.0.0aunk3
        const UNKNOWN_12 = 12;
        /// 5.0.0aunk4
        const HAS_TYPE_TREE_HASHES = 13;
        /// 5.0.0unk
        const UNKNOWN_14 = 14;
        /// 5.0.1 to 5.4.0
        const SUPPORTS_STRIPPED_OBJECT = 15;
        /// 5.5.0a
        const REFACTORED_CLASS_ID = 16;
        /// 5.5.0unk to 2018.4
        const REFACTOR_TYPE_DATA = 17;
        /// 2019.1a
        const REFACTOR_SHAREABLE_TYPE_TREE_DATA = 18;
        /// 2019.1unk
        const TYPE_TREE_NODE_WITH_TYPE_FLAGS = 19;
        /// 2019.2
        const SUPPORTS_REF_OBJECT = 20;
        /// 2019.3 to 2019.4
        const STORES_TYPE_DEPENDENCIES = 21;
        /// 2020.1 to x
        const LARGE_FILES_SUPPORT = 22;
    }
}

/// Write the file into the I/O stream.
///
/// Only the version from the header is respected,
/// the rest of the metadata is recomputed from the file's contents.
///
/// `file_data` is the place where the object's data is read from, according to
/// [`ObjectInfo::m_Offset`] and [`ObjectInfo::m_Size`].
///
/// If you want more flexibility with the objects, use [`write_serialized_with_objects`].
pub fn write_serialized<W: Write + Seek>(
    writer: W,
    serialized: &SerializedFile,
    file_data: &[u8],
    common_offset_map: &HashMap<&str, u32>,
) -> Result<(), std::io::Error> {
    write_serialized_with_objects(
        writer,
        serialized,
        common_offset_map,
        serialized.objects().map(|obj| {
            let data =
                &file_data[obj.m_Offset as usize..obj.m_Offset as usize + obj.m_Size as usize];
            (obj.clone(), Cow::Borrowed(data))
        }),
    )
}

/// Write the file with the given objects into the I/O stream.
///
/// Only the version from the header is respected,
/// the rest of the metadata is recomputed from the file's contents.
pub fn write_serialized_with_objects<'a, W: Write + Seek>(
    writer: W,
    serialized: &SerializedFile,
    common_offset_map: &HashMap<&str, u32>,
    objects: impl Iterator<Item = (ObjectInfo, Cow<'a, [u8]>)>,
) -> Result<(), std::io::Error> {
    match serialized.m_Header.m_Endianess {
        Endianness::Little => write_serialized_endianed::<W, LittleEndian>(
            writer,
            serialized,
            common_offset_map,
            objects,
        ),
        Endianness::Big => write_serialized_endianed::<W, BigEndian>(
            writer,
            serialized,
            common_offset_map,
            objects,
        ),
    }
}

fn write_serialized_endianed<'a, W: Write + Seek, B: ByteOrder>(
    mut writer: W,
    serialized: &SerializedFile,
    common_offset_map: &HashMap<&str, u32>,
    objects: impl Iterator<Item = (ObjectInfo, Cow<'a, [u8]>)>,
) -> Result<(), std::io::Error> {
    let start = writer.stream_position()?;

    serialized.m_Header.write(writer.by_ref())?;

    let metadata_start = writer.stream_position()?;

    let version = serialized.m_Header.m_Version;
    // in the reader this is 9 10 11, in UnityPy 7 8 13
    if version >= 7 {
        writer.write_cstr(&serialized.m_UnityVersion.unwrap().to_string())?;
    }
    if version >= 8 {
        writer.write_i32::<B>(serialized.m_TargetPlatform.unwrap())?;
    }

    if version >= 13 {
        writer.write_u8(serialized.m_EnableTypeTree as u8)?;
    }

    writer.write_i32::<B>(serialized.m_Types.len() as i32)?;
    for ty in &serialized.m_Types {
        ty.write::<_, B>(
            writer.by_ref(),
            version,
            false,
            serialized.m_EnableTypeTree,
            common_offset_map,
        )?;
    }

    if (7..14).contains(&version) {
        writer.write_i32::<B>(serialized.m_bigIDEnabled.unwrap())?;
    }

    let mut data_writer = Vec::<u8>::new();

    // writer.write_i32::<B>(serialized.m_Objects.len() as i32)?;
    let object_length_pos = writer.stream_position()?;
    writer.write_i32::<B>(0x0FFFFFFF)?; // written later

    let mut object_count = 0;
    for (mut obj, obj_data) in objects {
        crate::write_ext::align_vec::<8>(&mut data_writer);

        obj.m_Size = obj_data.len() as u32;
        let offset = data_writer.len() as i64;

        if serialized.m_bigIDEnabled.unwrap_or(0) > 0 {
            writer.write_i64::<B>(obj.m_PathID)?;
        } else if version < 14 {
            writer.write_i32::<B>(obj.m_PathID as i32)?;
        } else {
            writer.align::<4>()?;
            writer.write_i64::<B>(obj.m_PathID)?;
        }

        if version >= 22 {
            writer.write_i64::<B>(offset)?;
        } else {
            writer.write_i32::<B>(offset as i32)?;
        }

        data_writer.extend_from_slice(obj_data.as_ref());

        writer.write_u32::<B>(obj.m_Size)?;
        writer.write_i32::<B>(obj.m_TypeID)?;

        if version < 16 {
            writer.write_i16::<B>(obj.m_ClassID.0.try_into().unwrap())?;
        }
        if version < 11 {
            // write self.is_destroyed
            todo!()
        }

        if (11..17).contains(&version) {
            writer.write_i16::<B>(obj.m_ScriptTypeIndex.unwrap())?;
        }

        object_count += 1;
    }

    if version >= 11 {
        let script_types = serialized.m_ScriptTypes.as_deref().unwrap_or_default();
        writer.write_i32::<B>(script_types.len() as i32)?;
        for ty in script_types {
            writer.write_i32::<B>(ty.m_LocalSerializedFileIndex)?;
            if version < 14 {
                writer.write_i32::<B>(ty.m_LocalIdentifierInFile as i32)?;
            } else {
                writer.align::<4>()?;
                writer.write_i64::<B>(ty.m_LocalIdentifierInFile)?;
            }
        }
    }

    writer.write_i32::<B>(serialized.m_Externals.len() as i32)?;
    for external in &serialized.m_Externals {
        if version >= 6 {
            let temp_empty = external.tempEmpty.as_deref().unwrap();
            writer.write_cstr(temp_empty)?;
        }
        if version >= 5 {
            let guid = external.guid.as_ref().unwrap();
            writer.write_all(&guid.0)?;
            writer.write_i32::<B>(external.typeId.unwrap())?;
        }
        writer.write_cstr(&external.pathName)?;
    }

    if version >= 20 {
        let ref_types = serialized.m_RefTypes.as_deref().unwrap();
        writer.write_i32::<B>(ref_types.len() as i32)?;
        for _ in ref_types {
            todo!();
        }
    }

    if version >= 5 {
        let user_info = serialized.m_UserInformation.as_deref().unwrap();
        writer.write_cstr(user_info)?;
    }

    let computed_metadata_size = writer.stream_position()? as usize - metadata_start as usize;

    writer.align::<16>()?;
    if serialized.m_Header.m_DataOffset == 4096 {
        // unity behaviour
        if writer.stream_position()? < 4096 {
            writer.seek(std::io::SeekFrom::Start(4096))?;
        }
    }

    let computed_dataoffset = writer.stream_position()? as usize;
    writer.write_all(&data_writer)?;
    let computed_filesize = writer.stream_position()? as usize;

    let mut modified_header = serialized.m_Header;
    modified_header.m_MetadataSize = computed_metadata_size as u32;
    modified_header.m_DataOffset = computed_dataoffset as i64;
    modified_header.m_FileSize = computed_filesize as i64;

    // Write early data depending on unknown sizes

    writer.seek(std::io::SeekFrom::Start(start))?;
    modified_header.write(writer.by_ref())?;

    writer.seek(std::io::SeekFrom::Start(object_length_pos))?;
    writer.write_i32::<B>(object_count)?;

    /*
    // only if the objects weren't modified
    assert_eq!(
        serialized.m_Header.m_MetadataSize as usize,
        computed_metadata_size
    );
    assert_eq!(
        serialized.m_Header.m_DataOffset as usize,
        _computed_dataoffset
    );
    assert_eq!(serialized.m_Header.m_FileSize as usize, computed_filesize);
    */

    Ok(())
}

/// Required for serializing typetrees in [`write_serialized`].
pub fn build_common_offset_map(
    tpk: &TpkTypeTreeBlob,
    unity_version: UnityVersion,
) -> HashMap<&str, u32> {
    let strings = tpk
        .common_string
        .string_buffer_indices
        .iter()
        .map(|&i| &tpk.string_buffer[i as usize])
        .collect::<Vec<_>>();
    let count = tpk.common_string.get_count(unity_version).unwrap();
    let strings = &strings[..count as usize];

    let mut map = HashMap::new();
    let mut offset = 0;
    for str in strings {
        map.insert(str.as_str(), offset as u32);
        offset += str.len() + 1
    }
    map
}

fn write_n_zeroes<W: Write>(writer: &mut W, n: usize) -> std::io::Result<()> {
    const ZERO_BUF_SIZE: usize = 1024;
    let zero_buf = [0u8; ZERO_BUF_SIZE];

    let mut remaining = n;
    while remaining > 0 {
        let to_write = remaining.min(ZERO_BUF_SIZE);
        writer.write_all(&zero_buf[..to_write])?;
        remaining -= to_write;
    }
    Ok(())
}
