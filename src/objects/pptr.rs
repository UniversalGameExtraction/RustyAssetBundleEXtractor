use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct PPtr {
    pub m_FileID: i64,
    pub m_PathID: i64,
}

impl PPtr {
    fn get_object_handler<'a, R: std::io::Read + std::io::Seek>(
        &'a self,
        asset: &'a crate::files::SerializedFile,
        reader: &'a mut R,
    ) -> std::io::Result<crate::files::ObjectHandler<R>> {
        match asset.m_Objects.iter().find(|x| x.m_PathID == self.m_PathID) {
            Some(objectinfo) => Ok(asset.get_object_handler(objectinfo, reader)),
            None => Err(std::io::Error::new(
                std::io::ErrorKind::NotFound,
                "Object not found",
            )),
        }
    }
}
