use std::borrow::Cow;
use std::collections::HashMap;
use std::io::{Seek, Write};

use super::Error;
use crate::files::serializedfile::{
    self, ObjectInfo, SerializedFile, SerializedFileHeader, SerializedType, TypeTreeProvider,
};
use crate::objects::pptr::PathId;
use crate::objects::{ClassId, ClassIdType};
use crate::serde_typetree;
use crate::unity_version::UnityVersion;
use byteorder::LittleEndian;

use super::Endianness;

pub struct SerializedFileBuilder<'a, P> {
    unity_version: UnityVersion,
    common_offset_map: &'a HashMap<&'a str, u32>,
    pub typetree_provider: &'a P,
    pub _next_path_id: i64,
    pub objects: Vec<(ObjectInfo, Cow<'a, [u8]>)>,

    pub serialized: SerializedFile,

    types_cache: HashMap<ClassId, i32>,
}

impl<'a, P: TypeTreeProvider> SerializedFileBuilder<'a, P> {
    pub fn new(
        version: UnityVersion,
        typetree_provider: &'a P,
        common_offset_map: &'a HashMap<&'a str, u32>,
        enable_type_tree: bool,
    ) -> Self {
        Self {
            unity_version: version,
            typetree_provider,
            common_offset_map,
            _next_path_id: 0,
            objects: Vec::new(),
            types_cache: HashMap::default(),

            serialized: SerializedFile {
                m_Header: SerializedFileHeader {
                    m_MetadataSize: 0,
                    m_FileSize: 0,
                    m_Version: 22,
                    m_DataOffset: 0,
                    m_Endianess: Endianness::Little,
                    m_Reserved: [0, 0, 0],
                    unknown: 0,
                },
                m_UnityVersion: Some(version),
                m_TargetPlatform: Some(24),
                m_EnableTypeTree: enable_type_tree,
                m_bigIDEnabled: None,
                m_Types: Vec::new(),
                m_Objects: Default::default(),        // unused
                m_Objects_lookup: Default::default(), // unused
                m_ScriptTypes: Some(Vec::new()),
                m_Externals: Vec::new(),
                /*m_Externals: vec![FileIdentifier {
                    tempEmpty: Some("".to_owned()),
                    guid: Some(Guid([0, 0, 0, 0, 0, 0, 0, 0, 14, 0, 0, 0, 0, 0, 0, 0])),
                    typeId: Some(0),
                    pathName: "Library/unity default resources".into(),
                }], TODO*/
                m_RefTypes: Some(Vec::new()),
                m_UserInformation: Some("".into()),
            },
        }
    }

    /// - All FileID's must reference the correct `m_External`
    ///     - scripts also match the file ID in `m_ScriptTypes`
    /// - `m_TypeID` must match the correct `m_Types` index
    ///
    /// Does not set
    /// - `m_IsDestroyed`:
    /// - `m_ScriptTypeIndex`
    /// - `m_Stripped`
    pub fn add_object_raw(
        &mut self,
        class_id: ClassId,
        type_id: i32,
        data: Cow<'a, [u8]>,
    ) -> Result<(), Error> {
        let path_id = self.get_next_path_id();
        self.objects.push((
            ObjectInfo {
                m_PathID: path_id,
                m_Offset: 0, // ignored
                m_Size: 0,   // ignoed
                m_TypeID: type_id,
                m_ClassID: class_id,
                m_IsDestroyed: None,
                m_ScriptTypeIndex: None, // TODO?
                m_Stripped: None,
            },
            data,
        ));

        Ok(())
    }

    // Does not validate that path ID isn't already in use
    pub fn add_object<T: serde::Serialize + ClassIdType>(
        &mut self,
        object: &T,
    ) -> Result<(), Error> {
        let path_id = self.get_next_path_id();
        let type_id = self.get_or_insert_type(T::CLASS_ID);
        self.add_object_inner(object, path_id, type_id)
    }

    fn get_or_insert_type(&mut self, class_id: ClassId) -> i32 {
        *self.types_cache.entry(class_id).or_insert_with(|| {
            if class_id == ClassId::MonoBehaviour {
                todo!();
            }

            let serialized_ty = if self.serialized.m_EnableTypeTree {
                let ty = self
                    .typetree_provider
                    .get_typetree_node(class_id, self.unity_version)
                    .unwrap();
                // TODO: OldTypeHash
                // TOOD: ScriptId for Monobehaviour
                SerializedType::simple(class_id, Some(ty.into_owned()))
            } else {
                SerializedType::simple(class_id, None)
            };

            let type_index = self.serialized.m_Types.len();
            self.serialized.m_Types.push(serialized_ty);
            type_index as i32
        })
    }

    pub fn add_object_inner<T: serde::Serialize + ClassIdType>(
        &mut self,
        object: &T,
        path_id: PathId,
        type_id: i32,
    ) -> Result<(), Error> {
        let tt = self
            .typetree_provider
            .get_typetree_node(T::CLASS_ID, self.unity_version)
            .unwrap();

        let data =
            serde_typetree::to_vec::<_, LittleEndian>(&object, &tt).map_err(Error::Serialize)?;

        self.objects.push((
            ObjectInfo {
                m_PathID: path_id,
                m_TypeID: type_id,
                m_ClassID: T::CLASS_ID,
                ..Default::default()
            },
            Cow::Owned(data),
        ));

        Ok(())
    }

    pub fn add_type_uncached(&mut self, ty: SerializedType) -> i32 {
        let new_index = self.serialized.m_Types.len();
        self.serialized.m_Types.push(ty);
        new_index as i32
    }

    pub fn get_next_path_id(&mut self) -> PathId {
        let id = self._next_path_id;
        self._next_path_id += 1;
        id
    }

    pub fn write<W: Write + Seek>(self, writer: W) -> Result<(), Error> {
        serializedfile::write_serialized_with_objects(
            writer,
            &self.serialized,
            self.common_offset_map,
            self.objects.into_iter(),
        )?;

        Ok(())
    }
}
