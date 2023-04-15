use std::io::{Read, Seek, Error};

pub trait UnityFile {
    fn from_reader<T: Read+Seek>(reader: &mut T) -> Result<Self, Error>
    where
        Self: Sized;
    // fn get_objects(&self) -> &Vec<Object>;
    // fn to_writer(&self, writer: &mut impl Write+Seek) -> Result<(), Error>;
}