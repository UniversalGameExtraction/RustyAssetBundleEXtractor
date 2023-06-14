use crate::config::ExtractionConfig;
use std::io::{Error, Read, Seek};

pub trait UnityFile {
    fn from_reader<T: Read + Seek>(
        reader: &mut T,
        config: &ExtractionConfig,
    ) -> Result<Self, Error>
    where
        Self: Sized;
    // fn get_objects(&self) -> &Vec<Object>;
    // fn to_writer(&self, writer: &mut impl Write+Seek) -> Result<(), Error>;
}

pub struct FileEntry {
    pub offset: i64,
    pub size: i64,
    pub flags: u32,
    pub path: String,
}

impl FileEntry {
    // pub fn new(offset: i64, size: i64, flags: u32, path: String) -> Self {
    //     Self {
    //         offset,
    //         size,
    //         flags,
    //         path,
    //     }
    // }

    pub fn get_offset(&self) -> i64 {
        self.offset
    }

    pub fn get_size(&self) -> i64 {
        self.size
    }

    pub fn get_flags(&self) -> u32 {
        self.flags
    }

    pub fn get_path(&self) -> &String {
        &self.path
    }
}
