use crate::{
    files::UnityFile,
    read_ext::{ReadSeekUrexExt, ReadUrexExt},
};
use bitflags::bitflags;
use byteorder::{BigEndian, ReadBytesExt};
use num_enum::TryFromPrimitive;
use std::io::{Error, Read, Seek, Cursor};

bitflags! {
    struct ArchiveFlags: u32 {
        const COMPRESSION_TYPE_MASK = 0x3f;
        const BLOCKS_AND_DIRECTORY_INFO_COMBINED = 0x40;
        const BLOCKS_INFO_AT_THE_END = 0x80;
        const OLD_WEB_PLUGIN_COMPATIBILITY = 0x100;
        const BLOCK_INFO_NEED_PADDING_AT_START = 0x200;
    }
}

// bitflags! {
//     struct StorageBlockFlags: u32 {
//         const CompressionTypeMask = 0x3f;
//         const Streamed = 0x40;
//     }
// }

#[derive(Debug, Eq, PartialEq, TryFromPrimitive)]
#[repr(u32)]
enum CompressionType {
    None = 0,
    Lzma = 1,
    Lz4 = 2,
    Lz4hc = 3,
    Lzham = 4,
}

struct BundleFileHeader {
    signature: String,
    version: u32,
    unity_version: String,
    unity_revision: String,
    size: u32,
}

struct StorageBlock {
    compressed_size: u32,
    uncompressed_size: u32,
    flags: u32,
}

struct Node {
    offset: u32,
    size: u32,
    flags: u32,
    path: String,
}

pub struct BundleFile {
    m_Header: BundleFileHeader,
    m_BlocksInfo: Vec<StorageBlock>,
    m_DirectoryInfo: Vec<Node>,
    //filelist: Vec<StreamFile>
}

impl BundleFile {
    fn from_reader<T: Read + Seek>(reader: &mut T) -> Result<Self, Error> {
        let mut bundle = Self {
            m_Header: BundleFileHeader {
                signature: reader.read_cstr().unwrap(),
                version: reader.read_u32::<BigEndian>().unwrap(),
                unity_version: reader.read_cstr().unwrap(),
                unity_revision: reader.read_cstr().unwrap(),
                size: 0,
            },
            m_BlocksInfo: Vec::new(),
            m_DirectoryInfo: Vec::new(),
        };

        match bundle.m_Header.signature.as_str() {
            "UnityArchive" => {
                panic!("UnityArchive is not supported");
            }
            "UnityWeb" | "UnityRaw" => {
                if bundle.m_Header.version == 6 {
                    bundle.read_unityfs(reader)?;
                }
                bundle.read_unity_raw(reader)?;
            }
            "UnityFS" => {
                bundle.read_unityfs(reader)?;
            }
            _ => {
                panic!("Unknown signature");
            }
        };

        Ok(bundle)
    }

    fn read_unity_raw<T: Read + Seek>(&mut self, reader: &mut T) -> Result<(), Error> {
        if self.m_Header.version >= 4 {
            let hash = reader.read_u128::<BigEndian>().unwrap();
            let crc = reader.read_u32::<BigEndian>().unwrap();
        }
        let minimum_streamed_bytes = reader.read_u32::<BigEndian>().unwrap();

        self.m_Header.size = reader.read_u32::<BigEndian>().unwrap();

        let number_of_levels_to_download_before_streaming = reader.read_u32::<BigEndian>().unwrap();
        let level_count = reader.read_u32::<BigEndian>().unwrap();

        // jump to last level
        reader
            .seek(std::io::SeekFrom::Current(((level_count - 1) * 8) as i64))
            .unwrap_err();

        let mut m_BlocksInfo = StorageBlock {
            compressed_size: reader.read_u32::<BigEndian>().unwrap(),
            uncompressed_size: reader.read_u32::<BigEndian>().unwrap(),
            flags: 0,
        };

        if self.m_Header.version >= 2 {
            let complete_file_size = reader.read_u32::<BigEndian>().unwrap();
        }
        if self.m_Header.version >= 3 {
            let file_info_header_size = reader.read_u128::<BigEndian>().unwrap();
        }
        reader
            .seek(std::io::SeekFrom::Start(self.m_Header.size as u64))
            .unwrap_err();

        //ReadBlocksAndDirectory
        // is compressed -> lzma compression -> can be passed to decompress_block
        m_BlocksInfo.flags = (self.m_Header.signature == "UnityWeb") as u32;

        // let mut block_reader =
        //     BinaryReader::from_vec(&self.decompress_block(reader, &m_BlocksInfo));
        // block_reader.set_endian(Endian::Little);

        // let nodes_count = block_reader.read_i32().unwrap();
        // let m_DirectoryInfo: Vec<Node> = (0..nodes_count)
        //     .map(|_| Node {
        //         offset: block_reader.read_u32::<BigEndian>().unwrap(),
        //         size: block_reader.read_u32::<BigEndian>().unwrap(),
        //         flags: block_reader.read_u32::<BigEndian>().unwrap(),
        //         path: block_reader.read_cstr().unwrap(),
        //     })
        //     .collect();

        // self.read_files(&mut block_reader);
        Ok(())
    }

    fn read_unityfs<T: Read + Seek>(&mut self, reader: &mut T) -> Result<(), Error> {
        //ReadHeader
        self.m_Header.size = reader.read_i64::<BigEndian>().unwrap() as u32;

        let compressed_blocks_info_size = reader.read_u32::<BigEndian>().unwrap();
        let uncompressed_blocks_info_size = reader.read_u32::<BigEndian>().unwrap();
        let blocks_info_flags =
            ArchiveFlags::from_bits(reader.read_u32::<BigEndian>().unwrap()).unwrap();

        if self.m_Header.signature != "UnityFS" {
            reader.read_bool().unwrap();
        }

        //ReadBlocksInfoAndDirectory
        // TODO - check for 2019.4, which is version 6
        if self.m_Header.version >= 7 {
            reader.align(16)?;
        }

        let blocks_info_bytes: Vec<u8>;
        if blocks_info_flags.contains(ArchiveFlags::BLOCKS_INFO_AT_THE_END) {
            //0x80 BlocksInfoAtTheEnd
            let position = reader.stream_position().unwrap();
            // originally reader.length
            reader
                .seek(std::io::SeekFrom::End(compressed_blocks_info_size as i64))
                .unwrap_err();
            blocks_info_bytes = self.decompress_block(reader, {
                &StorageBlock {
                    compressed_size: compressed_blocks_info_size,
                    uncompressed_size: uncompressed_blocks_info_size,
                    flags: blocks_info_flags.bits(),
                }
            })?;
            reader.seek(std::io::SeekFrom::Start(position))?;
        } else {
            //0x40 BlocksAndDirectoryInfoCombined
            blocks_info_bytes = self.decompress_block(reader, {
                &StorageBlock {
                    compressed_size: compressed_blocks_info_size,
                    uncompressed_size: uncompressed_blocks_info_size,
                    flags: blocks_info_flags.bits(),
                }
            })?;
        }

        let block_info_reader = Cursor::new(&blocks_info_bytes);
        

        Ok(())
    }

    fn read_files<T: Read + Seek>(&mut self, reader: &T) {
        panic!("Not implemented");
    }

    fn decompress_block<T: Read + Seek>(
        &mut self,
        reader: &mut T,
        block: &StorageBlock,
    ) -> Result<Vec<u8>, Error> {
        let compressed = reader
            .read_bytes_sized(block.compressed_size as usize)
            .unwrap();
        match CompressionType::try_from(block.flags & 0x3F).unwrap() {
            CompressionType::Lzma => {
                let mut compressed_reader = std::io::Cursor::new(&compressed);
                let mut uncompressed = vec![0; block.uncompressed_size as usize];
                lzma_rs::lzma_decompress(&mut compressed_reader, &mut uncompressed).unwrap();
                Ok(uncompressed)
            }
            CompressionType::Lz4 | CompressionType::Lz4hc => {
                Ok(lz4_flex::block::decompress(&compressed, block.uncompressed_size as usize).unwrap())

            }
            CompressionType::Lzham => {
                panic!("LZHAM is not supported");
            }
            CompressionType::None => Ok(compressed),
        }
    }
}

impl UnityFile for BundleFile {
    fn from_reader<T: Read + Seek>(reader: &mut T) -> Result<Self, Error>
    where
        Self: Sized,
    {
        BundleFile::from_reader(reader)
    }
}

