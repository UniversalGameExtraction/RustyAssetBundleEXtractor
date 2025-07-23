use std::io::{Read, Seek};
use std::marker::PhantomData;

use crate::files::SerializedFile;
use crate::files::serializedfile::{self, ObjectInfo, TypeTreeProvider};
use serde_derive::{Deserialize, Serialize};

pub type PathId = i64;
pub type FileId = i32;

#[derive(Debug, Serialize, Deserialize, Default, Copy, Clone, PartialEq, Eq)]
pub struct PPtr {
    pub m_FileID: FileId,
    pub m_PathID: PathId,
}

impl PPtr {
    pub fn null() -> PPtr {
        PPtr::default()
    }

    pub fn typed<T>(self) -> TypedPPtr<T> {
        TypedPPtr {
            m_FileID: self.m_FileID,
            m_PathID: self.m_PathID,
            marker: PhantomData,
        }
    }

    pub fn optional(self) -> Option<PPtr> {
        (self.m_PathID != 0).then_some(self)
    }

    pub fn local(path_id: PathId) -> PPtr {
        PPtr {
            m_FileID: 0,
            m_PathID: path_id,
        }
    }

    pub fn into_local(self) -> PPtr {
        PPtr {
            m_FileID: 0,
            m_PathID: self.m_PathID,
        }
    }

    pub fn is_local(self) -> bool {
        self.m_FileID == 0
    }

    pub fn try_deref_local(self, serialized: &SerializedFile) -> Option<&ObjectInfo> {
        if self.m_PathID == 0 {
            return None;
        }
        serialized.get_object(self.m_PathID)
    }

    pub fn deref_local(self, serialized: &SerializedFile) -> &ObjectInfo {
        self.try_deref_local(serialized).unwrap()
    }

    pub fn try_deref_read<'de, T>(
        self,
        serialized: &SerializedFile,
        tpk: impl TypeTreeProvider,
        reader: &mut (impl Read + Seek),
    ) -> Option<T>
    where
        T: serde::Deserialize<'de>,
    {
        let info = self.try_deref_local(serialized)?;
        Some(serialized.read(info, tpk, reader).unwrap())
    }

    #[track_caller]
    pub fn deref_read_local<'de, T>(
        self,
        serialized: &SerializedFile,
        tpk: impl TypeTreeProvider,
        reader: &mut (impl Read + Seek),
    ) -> Result<T, serializedfile::Error>
    where
        T: serde::Deserialize<'de>,
    {
        if self.m_FileID != 0 {
            panic!("Non-local pptr in deref_read_local");
        }
        let info = serialized
            .get_object(self.m_PathID)
            .expect("Object did not exist in serialized file");

        serialized.read(info, tpk, reader)
    }

    pub fn get_object_handler<'a, R: std::io::Read + std::io::Seek>(
        &'a self,
        asset: &'a crate::files::SerializedFile,
        reader: &'a mut R,
    ) -> Result<crate::files::ObjectHandler<'a, R>, std::io::Error> {
        match asset.get_object(self.m_PathID) {
            Some(objectinfo) => Ok(asset.get_object_handler(objectinfo, reader)),
            None => Err(std::io::Error::new(
                std::io::ErrorKind::NotFound,
                "Object not found",
            )),
        }
    }
}

#[derive(Serialize, Deserialize)]
pub struct TypedPPtr<T> {
    pub m_FileID: i32,
    pub m_PathID: i64,
    #[serde(skip)]
    marker: PhantomData<T>,
}

impl<T> Default for TypedPPtr<T> {
    fn default() -> Self {
        Self {
            m_FileID: 0,
            m_PathID: 0,
            marker: PhantomData,
        }
    }
}

impl<T: std::fmt::Debug + 'static> std::fmt::Debug for TypedPPtr<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct(&format!("TypedPPtr<{}>", std::any::type_name::<T>()))
            .field("m_FileID", &self.m_FileID)
            .field("m_PathID", &self.m_PathID)
            .finish()
    }
}

impl<T> Copy for TypedPPtr<T> {}
#[allow(clippy::non_canonical_clone_impl)]
impl<T> Clone for TypedPPtr<T> {
    fn clone(&self) -> Self {
        Self {
            m_FileID: self.m_FileID,
            m_PathID: self.m_PathID,
            marker: self.marker,
        }
    }
}

impl<T> TypedPPtr<T> {
    pub fn null() -> TypedPPtr<T> {
        TypedPPtr::default()
    }

    pub fn untyped(self) -> PPtr {
        PPtr {
            m_FileID: self.m_FileID,
            m_PathID: self.m_PathID,
        }
    }

    pub fn into_local(self) -> TypedPPtr<T> {
        TypedPPtr {
            m_FileID: 0,
            m_PathID: self.m_PathID,
            marker: self.marker,
        }
    }

    pub fn try_deref(self, serialized: &SerializedFile) -> Option<&ObjectInfo> {
        self.untyped().try_deref_local(serialized)
    }

    pub fn deref_local(self, serialized: &SerializedFile) -> &ObjectInfo {
        self.untyped().deref_local(serialized)
    }

    pub fn try_deref_read<'de>(
        self,
        serialized: &SerializedFile,
        tpk: impl TypeTreeProvider,
        reader: &mut (impl Read + Seek),
    ) -> Option<T>
    where
        T: serde::Deserialize<'de>,
    {
        self.untyped().try_deref_read::<T>(serialized, tpk, reader)
    }

    pub fn deref_read_local<'de>(
        self,
        serialized: &SerializedFile,
        tpk: impl TypeTreeProvider,
        reader: &mut (impl Read + Seek),
    ) -> Result<T, serializedfile::Error>
    where
        T: serde::Deserialize<'de>,
    {
        self.untyped().deref_read_local(serialized, tpk, reader)
    }
}
