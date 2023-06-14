use crate::{config::ExtractionConfig, files::unityfile::UnityFile, read_ext::ReadUrexExt};
use byteorder::{BigEndian, ReadBytesExt};
use std::io::{Error, Read, Seek};
enum WebCompressionType {
    None,
    GZip,
    Brotli,
}

struct WebFile {
    compression: WebCompressionType,
    signature: String,
}

const GZIP_MAGIC: u16 = 0x1f8b;
const BROTLI_MAGIC: &str = "brotli";

impl UnityFile for WebFile {
    fn from_reader<T: Read + Seek>(reader: &mut T, config: &ExtractionConfig) -> Result<Self, Error>
    where
        Self: Sized,
    {
        let start_pos = reader.stream_position()?;
        let mut compression: WebCompressionType = WebCompressionType::None;
        // GZIP
        if reader.read_u16::<BigEndian>()? == GZIP_MAGIC {
            reader.seek(std::io::SeekFrom::Current(-2))?;
            compression = WebCompressionType::GZip;
        } else {
            // Brotli
            reader.seek(std::io::SeekFrom::Current(0x20 - 2))?;
            let magic = reader.read_string_sized(6)?;
            reader.seek(std::io::SeekFrom::Current(-0x26))?;
            if magic == BROTLI_MAGIC {
                compression = WebCompressionType::Brotli;
            }
        }
        let signature = reader.read_cstr()?;

        Ok(WebFile {
            compression,
            signature,
        })
    }
}
