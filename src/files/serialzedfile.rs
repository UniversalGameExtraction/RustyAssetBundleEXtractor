use super::UnityFile;
use crate::typetree::TypeTreeNode;
use crate::{
    config::ExtractionConfig,
    read_ext::{ReadSeekUrexExt, ReadUrexExt},
};
use bitflags::bitflags;
use byteorder::{BigEndian, ByteOrder, LittleEndian, ReadBytesExt};

#[derive(Debug, Copy, Clone)]
pub struct SerializedFileHeader {
    m_MetadataSize: u32,
    m_FileSize: i64,
    m_Version: u32,
    m_DataOffset: i64,
    m_Endianess: u8,
    m_Reserved: [u8; 3],
    unknown: i64,
}
impl SerializedFileHeader {
    fn from_reader<T: std::io::Read + std::io::Seek, B: ByteOrder>(
        reader: &mut T,
        config: &crate::config::ExtractionConfig,
    ) -> Result<SerializedFileHeader, std::io::Error> {
        let mut header = SerializedFileHeader {
            m_MetadataSize: reader.read_u32::<B>()?,
            m_FileSize: reader.read_u32::<B>()? as i64,
            m_Version: reader.read_u32::<B>()?,
            m_DataOffset: reader.read_u32::<B>()? as i64,
            m_Endianess: 0,
            m_Reserved: [0, 0, 0],
            unknown: 0,
        };

        if header.m_Version >= 9 {
            header.m_Endianess = reader.read_u8()?;
            header.m_Reserved = reader.read_bytes_sized(3)?.as_slice().try_into().unwrap();
        } else {
            reader.seek(std::io::SeekFrom::Current(
                header.m_FileSize - header.m_MetadataSize as i64,
            ))?;
            header.m_Endianess = reader.read_u8()?;
        }

        if header.m_Version >= 22 {
            match header.m_Endianess {
                0 => header.read_large_file_header::<T, LittleEndian>(reader, config)?,
                1 => header.read_large_file_header::<T, BigEndian>(reader, config)?,
                _ => panic!("Invalid endianess"),
            };
        };
        Ok(header)
    }

    fn read_large_file_header<T: std::io::Read + std::io::Seek, B: ByteOrder>(
        &mut self,
        reader: &mut T,
        config: &crate::config::ExtractionConfig,
    ) -> Result<(), std::io::Error> {
        self.m_MetadataSize = reader.read_u32::<B>()?;
        self.m_FileSize = reader.read_i64::<B>()?;
        self.m_DataOffset = reader.read_i64::<B>()?;
        self.unknown = reader.read_i64::<B>()?; // unknown
        Ok(())
    }
}

#[derive(Debug, Clone)]
pub struct SerializedType {
    pub m_ClassID: i32,
    pub m_IsStrippedType: bool,
    pub m_ScriptTypeIndex: i16,
    pub m_ScriptID: [u8; 16],
    pub m_OldTypeHash: [u8; 16],
    pub m_Type: Option<TypeTreeNode>,
    // for reftypes
    pub m_ClassName: Option<String>,
    pub m_NameSpace: Option<String>,
    pub m_AsmName: Option<String>,
    // for non ref-types
    pub m_TypeDependencies: Vec<i32>,
}
impl SerializedType {
    pub fn from_reader<T: std::io::Read + std::io::Seek, B: ByteOrder>(
        reader: &mut T,
        header: &SerializedFileHeader,
        m_EnableTypeTree: bool,
        isRefType: bool,
    ) -> Result<SerializedType, std::io::Error> {
        let mut typ = SerializedType {
            m_ClassID: -1,
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
        typ.m_ClassID = reader.read_i32::<B>()?;

        if header.m_Version >= SerializedFileFormatVersion::REFACTORED_CLASS_ID.bits() {
            typ.m_IsStrippedType = reader.read_bool()?;
        }

        if header.m_Version >= SerializedFileFormatVersion::REFACTOR_TYPE_DATA.bits() {
            typ.m_ScriptTypeIndex = reader.read_i16::<B>()?;
        }

        if header.m_Version >= SerializedFileFormatVersion::HAS_TYPE_TREE_HASHES.bits() {
            if (isRefType && typ.m_ScriptTypeIndex >= 0)
                || ((header.m_Version < SerializedFileFormatVersion::REFACTORED_CLASS_ID.bits()
                    && typ.m_ClassID < 0)
                    || (header.m_Version
                        >= SerializedFileFormatVersion::REFACTORED_CLASS_ID.bits()
                        && typ.m_ClassID == 114))
            {
                typ.m_ScriptID = reader.read_bytes_sized(16)?.as_slice().try_into().unwrap();
            }
            typ.m_OldTypeHash = reader.read_bytes_sized(16)?.as_slice().try_into().unwrap();
        }

        if m_EnableTypeTree {
            if header.m_Version >= SerializedFileFormatVersion::UNKNOWN_12.bits()
                || header.m_Version == SerializedFileFormatVersion::UNKNOWN_10.bits()
            {
                typ.m_Type =
                    Some(TypeTreeNode::blob_from_reader::<T, B>(reader, header.m_Version).unwrap());
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
}

#[derive(Debug, Clone)]
pub struct LocalSerializedObjectIdentifier {
    m_LocalSerializedFileIndex: i32,
    m_LocalIdentifierInFile: i64,
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

#[derive(Debug, Clone)]
pub struct ObjectInfo {
    pub m_PathID: i64,
    pub m_Offset: i64,
    pub m_Size: u32,
    pub m_TypeID: i32,
    pub m_ClassID: i32,
    pub m_IsDestroyed: Option<u16>,
    pub m_ScriptTypeIndex: Option<i16>,
    pub m_Stripped: Option<u8>,
}
impl ObjectInfo {
    pub fn from_reader<T: std::io::Read + std::io::Seek, B: ByteOrder>(
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
            m_ClassID: 0,
            m_IsDestroyed: None,
            m_ScriptTypeIndex: None,
            m_Stripped: None,
        };
        if bigIDEnabled.is_some() && bigIDEnabled.unwrap() > 0 {
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
            objectInfo.m_ClassID = reader.read_u16::<B>()? as i32;
        } else {
            objectInfo.m_ClassID = types[objectInfo.m_TypeID as usize].m_ClassID;
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
    localSerializedFileIndex: i32,
    localIdentifierInFile: i64,
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

#[derive(Debug, Clone)]
pub struct FileIdentifier {
    tempEmpty: Option<String>,
    guid: Option<Vec<u8>>,
    typeId: Option<i32>,
    pathName: String,
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
                Some(reader.read_bytes_sized(16)?)
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

#[derive(Debug)]
pub struct ObjectHandler<'a, R: std::io::Read + std::io::Seek> {
    pub info: &'a ObjectInfo,
    pub typ: Option<&'a SerializedType>,
    pub file: &'a SerializedFile,
    pub reader: &'a mut R,
}

macro_rules! parse_as {
    ($name:ident, $ret_typ: ty) => {
        paste::item! {
            #[doc = "Parses the object as" $name "."]
            pub fn [< parse_as_ $name >](&mut self) -> Result<$ret_typ, std::io::Error>{
                match self.get_typetree().cloned() {
                    Some(node) => {
                        self.reader
                            .seek(std::io::SeekFrom::Start(self.info.m_Offset as u64))?;
                        match self.file.m_Header.m_Endianess {
                            0 => node.[< read_as_ $name >]::<R, LittleEndian>(self.reader),
                            1 => node.[< read_as_ $name >]::<R, BigEndian>(self.reader),
                            _ => Err(std::io::Error::new(
                                std::io::ErrorKind::NotFound,
                                "Unknown endianess!",
                            )),
                        }
                    }
                    _ => Err(std::io::Error::new(
                        std::io::ErrorKind::NotFound,
                        "Couldn't find typetree!",
                    )),
                }
            }
        }
    };
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

    pub fn peak_name(&mut self) -> Result<String, std::io::Error> {
        self.reader
            .seek(std::io::SeekFrom::Start(self.info.m_Offset as u64))
            .unwrap();

        // todo - check against typeid
        match self.file.m_Header.m_Endianess {
            0 => self.reader.read_string::<LittleEndian>(),
            1 => self.reader.read_string::<BigEndian>(),
            _ => Err(std::io::Error::new(
                std::io::ErrorKind::NotFound,
                "Unknown endianess!",
            )),
        }
    }

    pub fn parse<T: serde::de::DeserializeOwned>(&mut self) -> std::io::Result<T> {
        let transmission = self.parse_as_msgpack().unwrap();
        Ok(rmp_serde::from_slice::<T>(&&transmission.as_slice()).unwrap())
    }

    parse_as!(json, serde_json::Value);
    parse_as!(yaml, Result<serde_yaml::Value, serde_yaml::Error>);
    parse_as!(msgpack, Vec<u8>);
}

#[derive(Debug, Clone)]
pub struct SerializedFile {
    pub m_Header: SerializedFileHeader,
    pub m_UnityVersion: Option<String>,
    pub m_TargetPlatform: Option<i32>,
    pub m_bigIDEnabled: Option<i32>,
    pub m_Types: Vec<SerializedType>,
    pub m_Objects: Vec<ObjectInfo>,
    pub m_ScriptTypes: Option<Vec<ScriptType>>,
    pub m_Externals: Vec<FileIdentifier>,
    pub m_RefTypes: Option<Vec<SerializedType>>,
    pub m_UserInformation: Option<String>,
}

impl SerializedFile {
    pub fn from_reader<T: std::io::Read + std::io::Seek>(
        reader: &mut T,
        config: &crate::config::ExtractionConfig,
    ) -> Result<SerializedFile, std::io::Error> {
        let header = SerializedFileHeader::from_reader::<T, BigEndian>(reader, config)?;

        match header.m_Endianess {
            0 => SerializedFile::from_reader_endianed::<T, LittleEndian>(reader, header, config),
            1 => SerializedFile::from_reader_endianed::<T, BigEndian>(reader, header, config),
            _ => panic!("Invalid endianess"),
        }
    }

    fn from_reader_endianed<T, B>(
        reader: &mut T,
        header: SerializedFileHeader,
        config: &crate::config::ExtractionConfig,
    ) -> Result<SerializedFile, std::io::Error>
    where
        T: std::io::Read + std::io::Seek,
        B: ByteOrder,
    {
        // Read Metadata
        let mut m_UnityVersion = None;
        if header.m_Version >= 9 {
            m_UnityVersion = Some(reader.read_cstr()?);
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

        let mut m_EnabledTypeTree = false;
        if header.m_Version >= 11 {
            m_EnabledTypeTree = reader.read_bool()?;
        }

        // Read Types
        let typeCount = reader.read_i32::<B>()?;
        let m_Types: Vec<SerializedType> = (0..typeCount)
            .map(|_| {
                let x =
                    SerializedType::from_reader::<T, B>(reader, &header, m_EnabledTypeTree, false);
                x.unwrap()
            })
            .collect();

        let m_bigIDEnabled = None;
        if header.m_Version >= SerializedFileFormatVersion::UNKNOWN_7.bits()
            && header.m_Version < SerializedFileFormatVersion::UNKNOWN_14.bits()
        {
            let bigIDEnabled = Some(reader.read_i32::<B>()?);
        }

        // Read Objects
        let objectCount = reader.read_i32::<B>()?;
        let m_Objects: Vec<ObjectInfo> = (0..objectCount)
            .map(|_| {
                ObjectInfo::from_reader::<T, B>(reader, &header, m_bigIDEnabled, &m_Types).unwrap()
            })
            .collect();

        let m_ScriptTypes = None;
        if header.m_Version >= SerializedFileFormatVersion::HAS_SCRIPT_TYPE_INDEX.bits() {
            let scriptCount = reader.read_i32::<B>()?;
            let m_ScriptTypes: Option<Vec<LocalSerializedObjectIdentifier>> = Some(
                (0..scriptCount)
                    .map(|_| {
                        LocalSerializedObjectIdentifier::from_reader::<T, B>(reader, &header)
                            .unwrap()
                    })
                    .collect(),
            );
        }

        let externalsCount = reader.read_i32::<B>()?;
        let m_Externals = (0..externalsCount)
            .map(|_| FileIdentifier::from_reader::<T, B>(reader, &header).unwrap())
            .collect();

        let m_RefTypes = None;
        if header.m_Version >= SerializedFileFormatVersion::SUPPORTS_REF_OBJECT.bits() {
            let refTypesCount = reader.read_i32::<B>()?;
            let m_RefTypes: Option<Vec<SerializedType>> = Some(
                (0..refTypesCount)
                    .map(|_| {
                        SerializedType::from_reader::<T, B>(
                            reader,
                            &header,
                            m_EnabledTypeTree,
                            true,
                        )
                        .unwrap()
                    })
                    .collect(),
            );
        }

        let m_UserInformation = None;
        if header.m_Version >= SerializedFileFormatVersion::UNKNOWN_5.bits() {
            let m_UserInformation = Some(reader.read_cstr()?);
        }

        //reader.AlignStream(16);
        Ok(SerializedFile {
            m_Header: header,
            m_UnityVersion,
            m_TargetPlatform,
            m_bigIDEnabled,
            m_Types,
            m_Objects,
            m_ScriptTypes,
            m_Externals,
            m_RefTypes,
            m_UserInformation,
        })
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

impl UnityFile for SerializedFile {
    fn from_reader<T: std::io::Read + std::io::Seek>(
        reader: &mut T,
        config: &ExtractionConfig,
    ) -> Result<Self, std::io::Error>
    where
        Self: Sized,
    {
        SerializedFile::from_reader(reader, config)
    }
}

bitflags! {
    pub struct SerializedFileFormatVersion: u32 {
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
