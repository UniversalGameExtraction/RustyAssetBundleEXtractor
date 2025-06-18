use crate::unity_version::UnityVersion;
use crate::{
    archive_storage_manager::ArchiveStorageDecryptor,
    config::ExtractionConfig,
    files::unityfile::{FileEntry, UnityFile},
    read_ext::{ReadSeekUrexExt, ReadUrexExt},
};
use bitflags::bitflags;
use byteorder::{BigEndian, ReadBytesExt};
use num_enum::TryFromPrimitive;
use std::io::{Error, Read, Seek, Write};
use std::str::FromStr;

mod reader;
pub use reader::BundleFileReader;

bitflags! {
    struct ArchiveFlags: u32 {
        const COMPRESSION_TYPE_MASK = 0x3f;
        const BLOCKS_AND_DIRECTORY_INFO_COMBINED = 0x40;
        const BLOCKS_INFO_AT_THE_END = 0x80;
        const OLD_WEB_PLUGIN_COMPATIBILITY = 0x100;
        const BLOCK_INFO_NEED_PADDING_AT_START = 0x200;
        const USES_ASSET_BUNDLE_ENCRYPTION = 0x400;
    }

    struct ArchiveFlagsOld: u32 {
        const COMPRESSION_TYPE_MASK = 0x3f;
        const BLOCKS_AND_DIRECTORY_INFO_COMBINED = 0x40;
        const BLOCKS_INFO_AT_THE_END = 0x80;
        const OLD_WEB_PLUGIN_COMPATIBILITY = 0x100;
        const USES_ASSET_BUNDLE_ENCRYPTION = 0x200;
    }
}

// bitflags! {
//     struct StorageBlockFlags: u32 {
//         const CompressionTypeMask = 0x3f;
//         const Streamed = 0x40;
//         const Encrypted = 0x100;
//     }
// }

#[derive(Debug, Eq, PartialEq, TryFromPrimitive)]
#[repr(u32)]
pub enum CompressionType {
    None = 0,
    Lzma = 1,
    Lz4 = 2,
    Lz4hc = 3,
    Lzham = 4,
}

#[derive(Debug)]
pub enum BundleSignature {
    UnityArchive,
    UnityWeb,
    UnityRaw,
    UnityFS,
}
impl FromStr for BundleSignature {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(match s {
            "UnityArchive" => BundleSignature::UnityArchive,
            "UnityWeb" => BundleSignature::UnityWeb,
            "UnityRaw" => BundleSignature::UnityRaw,
            "UnityFS" => BundleSignature::UnityFS,
            _ => return Err(()),
        })
    }
}

pub struct BundleFileHeader {
    pub signature: BundleSignature,
    pub version: u32,
    pub unity_version: String,
    pub unity_revision: Option<UnityVersion>,
    pub size: u32,
}

impl BundleFileHeader {
    pub fn from_reader<T: Read + Seek>(reader: &mut T) -> Result<Self, Error> {
        Ok(BundleFileHeader {
            signature: reader.read_cstr().unwrap().parse().unwrap(),
            version: reader.read_u32::<BigEndian>().unwrap(),
            unity_version: reader.read_cstr().unwrap(),
            unity_revision: {
                let str = reader.read_cstr().unwrap();
                (str != "0.0.0").then(|| str.parse::<UnityVersion>().unwrap())
            },
            size: 0,
        })
    }

    fn get_revision_tuple(&self, config: &ExtractionConfig) -> (u16, u16, u16) {
        self.unity_revision
            .unwrap_or(config.fallback_unity_version)
            .version_tuple()
    }
}

pub struct StorageBlock {
    pub compressed_size: u32,
    pub uncompressed_size: u32,
    pub flags: u32,
}

pub struct BundleFile {
    pub m_Header: BundleFileHeader,
    pub m_BlocksInfo: Vec<StorageBlock>,
    pub m_DirectoryInfo: Vec<FileEntry>,
    pub m_BlockData: Vec<u8>,
    _decryptor: Option<ArchiveStorageDecryptor>,
}

impl BundleFile {
    pub fn from_reader<T: Read + Seek>(
        reader: &mut T,
        config: &ExtractionConfig,
    ) -> Result<Self, Error> {
        let mut bundle = Self {
            m_Header: BundleFileHeader::from_reader(reader)?,
            m_BlocksInfo: Vec::new(),
            m_DirectoryInfo: Vec::new(),
            m_BlockData: Vec::new(),
            _decryptor: None,
        };

        (bundle.m_DirectoryInfo, bundle.m_BlockData) = match bundle.m_Header.signature {
            BundleSignature::UnityArchive => {
                panic!("UnityArchive is not supported");
            }
            BundleSignature::UnityWeb | BundleSignature::UnityRaw => {
                if bundle.m_Header.version == 6 {
                    let mut out = Vec::new();
                    let file_entries = bundle.read_unityfs(reader, &mut out, config)?;
                    (file_entries, out)
                } else {
                    bundle.read_unity_raw(reader, config)?
                }
            }
            BundleSignature::UnityFS => {
                let mut out = Vec::new();
                let file_entries = bundle.read_unityfs(reader, &mut out, config)?;
                (file_entries, out)
            }
        };
        Ok(bundle)
    }

    fn read_unity_raw<T: Read + Seek>(
        &mut self,
        reader: &mut T,
        _config: &ExtractionConfig,
    ) -> Result<(Vec<FileEntry>, Vec<u8>), Error> {
        if self.m_Header.version >= 4 {
            let _hash = reader.read_u128::<BigEndian>().unwrap();
            let _crc = reader.read_u32::<BigEndian>().unwrap();
        }
        let _minimum_streamed_bytes = reader.read_u32::<BigEndian>().unwrap();

        self.m_Header.size = reader.read_u32::<BigEndian>().unwrap();

        let _number_of_levels_to_download_before_streaming =
            reader.read_u32::<BigEndian>().unwrap();
        let level_count = reader.read_u32::<BigEndian>().unwrap();

        // jump to last level
        // TODO - keep the levels for use in low-memory block decompressor strategy
        reader
            .seek(std::io::SeekFrom::Current(((level_count - 1) * 8) as i64))
            .unwrap();

        let mut m_BlocksInfo = StorageBlock {
            compressed_size: reader.read_u32::<BigEndian>().unwrap(),
            uncompressed_size: reader.read_u32::<BigEndian>().unwrap(),
            flags: 0,
        };

        if self.m_Header.version >= 2 {
            let _complete_file_size = reader.read_u32::<BigEndian>().unwrap();
        }
        if self.m_Header.version >= 3 {
            let _file_info_header_size = reader.read_u128::<BigEndian>().unwrap();
        }
        reader
            .seek(std::io::SeekFrom::Start(self.m_Header.size as u64))
            .unwrap();

        // ReadBlocksAndDirectory
        // is compressed -> lzma compression -> can be passed to decompress_block
        if let BundleSignature::UnityWeb = self.m_Header.signature {
            m_BlocksInfo.flags |= CompressionType::Lzma as u32;
        }

        let mut blocks_info_bytes = Vec::with_capacity(m_BlocksInfo.uncompressed_size as usize);

        decompress_block(
            reader,
            &mut blocks_info_bytes,
            &m_BlocksInfo,
            0,
            self._decryptor.as_ref(),
        )?;
        let block_info_reader = &mut blocks_info_bytes.as_slice();

        let FileEntrys_count = block_info_reader.read_i32::<BigEndian>().unwrap();
        let m_DirectoryInfo: Vec<FileEntry> = (0..FileEntrys_count)
            .map(|_| FileEntry {
                path: block_info_reader.read_cstr().unwrap(),
                offset: block_info_reader.read_u32::<BigEndian>().unwrap() as i64,
                size: block_info_reader.read_u32::<BigEndian>().unwrap() as i64,
                flags: 0,
            })
            .collect();

        Ok((m_DirectoryInfo, blocks_info_bytes))
    }

    fn read_unityfs<T: Read + Seek, W: Write>(
        &mut self,
        reader: &mut T,
        writer: &mut W,
        config: &ExtractionConfig,
    ) -> Result<Vec<FileEntry>, Error> {
        let (block_infos, directory_infos, decryptor) =
            read_unityfs_info(&mut self.m_Header, reader, config)?;
        self._decryptor = decryptor;

        for (i, block) in block_infos.iter().enumerate() {
            decompress_block(reader, writer, block, i, self._decryptor.as_ref())?;
        }

        Ok(directory_infos)
    }
}

#[allow(clippy::type_complexity)]
pub fn read_unityfs_info<R: Read + Seek>(
    m_Header: &mut BundleFileHeader,
    reader: &mut R,
    config: &ExtractionConfig,
) -> Result<
    (
        Vec<StorageBlock>,
        Vec<FileEntry>,
        Option<ArchiveStorageDecryptor>,
    ),
    Error,
> {
    let use_new_archive_flags = !{
        let version = m_Header.get_revision_tuple(config);
        (version < (2020, 0, 0))
            | ((version.0 == 2020) & (version < (2020, 3, 34)))
            | ((version.0 == 2021) & (version < (2021, 3, 2)))
            | ((version.0 == 2022) & (version < (2022, 1, 1)))
    };

    //ReadHeader
    m_Header.size = reader.read_i64::<BigEndian>()? as u32;

    let block_info = StorageBlock {
        compressed_size: reader.read_u32::<BigEndian>()?,
        uncompressed_size: reader.read_u32::<BigEndian>()?,
        flags: reader.read_u32::<BigEndian>()?,
    };
    if let BundleSignature::UnityFS = m_Header.signature {
        reader.read_bool()?;
    }

    //ReadBlocksInfoAndDirectory
    // TODO - check for 2019.4, which is version 6
    if m_Header.version >= 7 {
        reader.align(16)?;
    } // TODO else if version >= (2019, 4)

    let mut decryptor = None;

    let mut blocks_info_bytes = Vec::with_capacity(block_info.uncompressed_size as usize);
    if block_info.flags & ArchiveFlags::BLOCKS_INFO_AT_THE_END.bits() != 0 {
        // 0x80 BlocksInfoAtTheEnd
        let position = reader.stream_position()?;
        // originally reader.length
        reader.seek(std::io::SeekFrom::End(-(block_info.compressed_size as i64)))?;
        decompress_block(reader, &mut blocks_info_bytes, &block_info, 0, None)?;
        reader.seek(std::io::SeekFrom::Start(position))?;
    } else {
        // 0x40 BlocksAndDirectoryInfoCombined
        if (use_new_archive_flags
            & (block_info.flags & ArchiveFlags::USES_ASSET_BUNDLE_ENCRYPTION.bits() > 0))
            | (!use_new_archive_flags
                & (block_info.flags & ArchiveFlagsOld::USES_ASSET_BUNDLE_ENCRYPTION.bits() > 0))
        {
            decryptor = Some(ArchiveStorageDecryptor::from_reader(
                reader,
                config.unitycn_key.unwrap(),
            )?);
        }
        decompress_block(
            reader,
            &mut blocks_info_bytes,
            &block_info,
            0,
            decryptor.as_ref(),
        )?;
    }

    let block_info_reader = &mut blocks_info_bytes.as_slice();
    let _uncompressed_data_hash = block_info_reader.read_u128::<BigEndian>()?;

    let block_info_count = block_info_reader.read_i32::<BigEndian>()?;
    let m_BlocksInfo: Vec<StorageBlock> = (0..block_info_count)
        .map(|_| StorageBlock {
            uncompressed_size: block_info_reader.read_u32::<BigEndian>().unwrap(),
            compressed_size: block_info_reader.read_u32::<BigEndian>().unwrap(),
            flags: block_info_reader.read_u16::<BigEndian>().unwrap() as u32,
        })
        .collect();

    let FileEntrys_count = block_info_reader.read_i32::<BigEndian>()?;
    let m_DirectoryInfo: Vec<FileEntry> = (0..FileEntrys_count)
        .map(|_| FileEntry {
            offset: block_info_reader.read_i64::<BigEndian>().unwrap(),
            size: block_info_reader.read_i64::<BigEndian>().unwrap(),
            flags: block_info_reader.read_u32::<BigEndian>().unwrap(),
            path: block_info_reader.read_cstr().unwrap(),
        })
        .collect();

    if use_new_archive_flags
        & (block_info.flags & ArchiveFlags::BLOCK_INFO_NEED_PADDING_AT_START.bits() > 0)
    {
        reader.align(16)?;
    }

    Ok((m_BlocksInfo, m_DirectoryInfo, decryptor))
}

pub fn decompress_block<R: Read + Seek, W: Write>(
    reader: &mut R,
    writer: &mut W,
    block: &StorageBlock,
    index: usize,
    decryptor: Option<&ArchiveStorageDecryptor>,
) -> Result<(), Error> {
    match CompressionType::try_from(block.flags & 0x3F).unwrap() {
        CompressionType::Lzma => {
            let compressed = reader.read_bytes_sized(block.compressed_size as usize)?;
            lzma_rs::lzma_decompress_with_options(
                &mut compressed.as_slice(),
                writer,
                &lzma_rs::decompress::Options {
                    unpacked_size: lzma_rs::decompress::UnpackedSize::UseProvided(Some(
                        block.uncompressed_size as u64,
                    )),
                    ..Default::default()
                },
            )
            .unwrap();

            Ok(())
        }
        CompressionType::Lz4 | CompressionType::Lz4hc => {
            let mut compressed = reader.read_bytes_sized(block.compressed_size as usize)?;

            if block.flags & 0x100 > 0 {
                // UnityCN encryption
                decryptor.unwrap().decrypt_block(
                    &mut compressed,
                    block.compressed_size as usize,
                    index,
                )?;
            }
            let data =
                lz4_flex::block::decompress(&compressed, block.uncompressed_size as usize).unwrap();
            writer.write_all(&data)?;
            Ok(())
        }
        CompressionType::Lzham => {
            panic!("LZHAM is not supported");
        }
        CompressionType::None => {
            std::io::copy(&mut reader.take(block.compressed_size as u64), writer)?;
            Ok(())
        }
    }
}

impl UnityFile for BundleFile {
    fn from_reader<T: Read + Seek>(reader: &mut T, config: &ExtractionConfig) -> Result<Self, Error>
    where
        Self: Sized,
    {
        BundleFile::from_reader(reader, config)
    }
}
