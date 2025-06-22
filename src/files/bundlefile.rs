pub mod builder;

use crate::unity_version::UnityVersion;
use crate::write_ext::{WriteExt, WriteSeekExt};
use crate::{
    archive_storage_manager::ArchiveStorageDecryptor,
    config::ExtractionConfig,
    files::unityfile::{FileEntry, UnityFile},
    read_ext::{ReadSeekUrexExt, ReadUrexExt},
};
use bitflags::bitflags;
use byteorder::WriteBytesExt;
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

#[derive(Debug, Eq, PartialEq, TryFromPrimitive, Clone, Copy)]
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
impl std::fmt::Display for BundleSignature {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            BundleSignature::UnityArchive => f.write_str("UnityArchive"),
            BundleSignature::UnityWeb => f.write_str("UnityWeb"),
            BundleSignature::UnityRaw => f.write_str("UnityRaw"),
            BundleSignature::UnityFS => f.write_str("UnityFS"),
        }
    }
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
    pub fn write<T: Write>(&self, mut writer: T) -> Result<(), Error> {
        writer.write_cstr(&self.signature.to_string())?;
        writer.write_u32::<BigEndian>(self.version)?;
        writer.write_cstr(&self.unity_version)?;
        writer.write_cstr(
            self.unity_revision
                .map(|v| v.to_string())
                .as_deref()
                .unwrap_or("0.0.0"),
        )?;
        Ok(())
    }
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

fn write_infoblock(block_infos: &[StorageBlock], files: &[FileEntry]) -> Result<Vec<u8>, Error> {
    let mut info_block_writer = Vec::<u8>::new();
    info_block_writer.write_u128::<BigEndian>(0)?;

    info_block_writer.write_i32::<BigEndian>(block_infos.len() as i32)?;
    for block in block_infos {
        info_block_writer.write_u32::<BigEndian>(block.uncompressed_size)?;
        info_block_writer.write_u32::<BigEndian>(block.compressed_size)?;
        info_block_writer.write_u16::<BigEndian>(block.flags as u16)?;
    }

    info_block_writer.write_i32::<BigEndian>(files.len() as i32)?;
    for file in files {
        info_block_writer.write_i64::<BigEndian>(file.offset)?;
        info_block_writer.write_i64::<BigEndian>(file.size)?;
        info_block_writer.write_u32::<BigEndian>(file.flags)?;
        info_block_writer.write_cstr(&file.path)?;
    }
    Ok(info_block_writer)
}

pub fn write_bundle<W: Write + Seek>(
    header: &BundleFileHeader,
    mut writer: W,
    header_compression: CompressionType,
    compression: CompressionType,
    files: &[FileEntry],
    uncompressed_data: &[u8],
) -> Result<(), Error> {
    header.write(&mut writer)?;
    if let BundleSignature::UnityFS = header.signature {
    } else {
        todo!();
    }
    write_fs(
        header,
        writer,
        header_compression,
        compression,
        files,
        uncompressed_data,
    )
}

pub fn write_bundle_iter<W: Write + Seek, R: Read>(
    header: &BundleFileHeader,
    mut writer: W,
    header_compression: CompressionType,
    compression: CompressionType,
    files: impl Iterator<Item = Result<(String, R), std::io::Error>>,
) -> Result<(), Error> {
    header.write(&mut writer)?;
    if let BundleSignature::UnityFS = header.signature {
    } else {
        todo!();
    }
    write_fs_iter(header, writer, header_compression, compression, files)
}

fn write_fs_iter<W: Write + Seek, R: Read>(
    header: &BundleFileHeader,
    writer: W,
    header_compression: CompressionType,
    compression: CompressionType,
    files: impl Iterator<Item = Result<(String, R), std::io::Error>>,
) -> Result<(), Error> {
    let mut uncompressed_data = Vec::new();
    let mut dir_info = Vec::new();
    for item in files {
        let (path, mut data) = item?;
        let offset = uncompressed_data.len();
        let len = std::io::copy(&mut data, &mut uncompressed_data)?;
        dir_info.push(FileEntry {
            offset: offset as i64,
            size: len as i64,
            flags: 4,
            path,
        });
    }
    write_fs(
        header,
        writer,
        header_compression,
        compression,
        &dir_info,
        &uncompressed_data,
    )
}

pub fn write_fs<W: Write + Seek>(
    header: &BundleFileHeader,
    mut writer: W,
    header_compression: CompressionType,
    compression: CompressionType,
    files: &[FileEntry],
    uncompressed_data: &[u8],
) -> Result<(), Error> {
    let use_new_archive_flags = use_new_archive_flags(header, &ExtractionConfig::default());
    let info_block_flags = match use_new_archive_flags {
        true => (ArchiveFlags::BLOCKS_AND_DIRECTORY_INFO_COMBINED
            | ArchiveFlags::BLOCK_INFO_NEED_PADDING_AT_START)
            .bits(),
        false => ArchiveFlagsOld::BLOCKS_AND_DIRECTORY_INFO_COMBINED.bits(),
    } | header_compression as u32;

    let (compressed_block_data, block_infos) = {
        let mut compressed_block_data = Vec::new();
        let mut block_infos = Vec::new();

        let flags = match compression {
            CompressionType::Lz4hc => ArchiveFlags::empty().bits(),
            _ => ArchiveFlags::BLOCKS_AND_DIRECTORY_INFO_COMBINED.bits(),
        } | compression as u32;

        match compression {
            CompressionType::None => {
                compressed_block_data.extend_from_slice(uncompressed_data);
                block_infos.push(StorageBlock {
                    compressed_size: uncompressed_data.len() as u32,
                    uncompressed_size: uncompressed_data.len() as u32,
                    flags,
                });
            }
            _ => {
                const CHUNK_SIZE: usize = 0x20000;

                let mut reader = uncompressed_data;
                loop {
                    let chunk = read_chunk_slice::<CHUNK_SIZE>(&mut reader);

                    let start_len = compressed_block_data.len();
                    write_block(&mut compressed_block_data, chunk, flags)?;
                    let end_len = compressed_block_data.len();
                    let compressed_size = end_len - start_len;
                    block_infos.push(StorageBlock {
                        compressed_size: compressed_size as u32,
                        uncompressed_size: chunk.len() as u32,
                        flags,
                    });

                    if chunk.len() < CHUNK_SIZE {
                        break;
                    }
                }
            }
        };

        (compressed_block_data, block_infos)
    };

    let (info_block_compressed, info_storage_block) = {
        let mut info_block = Vec::new();
        info_block.write_u128::<BigEndian>(0)?;

        info_block.write_i32::<BigEndian>(block_infos.len() as i32)?;
        for block in block_infos {
            info_block.write_u32::<BigEndian>(block.uncompressed_size)?;
            info_block.write_u32::<BigEndian>(block.compressed_size)?;
            info_block.write_u16::<BigEndian>(block.flags as u16)?;
        }

        info_block.write_i32::<BigEndian>(files.len() as i32)?;
        for file in files {
            info_block.write_i64::<BigEndian>(file.offset)?;
            info_block.write_i64::<BigEndian>(file.size)?;
            info_block.write_u32::<BigEndian>(file.flags)?;
            info_block.write_cstr(&file.path)?;
        }

        let mut info_block_compressed = Vec::new();
        write_block(&mut info_block_compressed, &info_block, info_block_flags)?;
        let info_storage_block = StorageBlock {
            compressed_size: info_block_compressed.len() as u32,
            uncompressed_size: info_block.len() as u32,
            flags: info_block_flags,
        };
        (info_block_compressed, info_storage_block)
    };

    // begin writing

    let start_position = writer.stream_position()?;
    writer.write_i64::<BigEndian>(i64::from_be_bytes([255, 255, 255, 255, 255, 255, 255, 255]))?; // total size

    writer.write_u32::<BigEndian>(info_storage_block.compressed_size)?;
    writer.write_u32::<BigEndian>(info_storage_block.uncompressed_size)?;
    writer.write_u32::<BigEndian>(info_storage_block.flags)?;

    if header.version >= 7 {
        // todo >= (2019,4)
        writer.align::<16>()?;
    }

    if (info_block_flags & ArchiveFlags::BLOCKS_INFO_AT_THE_END.bits()) != 0 {
        todo!()
    }

    let do_encryption = match use_new_archive_flags {
        true => info_block_flags & ArchiveFlags::USES_ASSET_BUNDLE_ENCRYPTION.bits() > 0,
        false => info_block_flags & ArchiveFlagsOld::USES_ASSET_BUNDLE_ENCRYPTION.bits() > 0,
    };
    if do_encryption {
        todo!();
    }

    writer.write_all(&info_block_compressed)?;

    if use_new_archive_flags
        & (info_storage_block.flags & ArchiveFlags::BLOCK_INFO_NEED_PADDING_AT_START.bits() > 0)
    {
        writer.align::<16>()?;
    }

    writer.write_all(&compressed_block_data)?;

    let length = writer.stream_position().unwrap();

    writer.seek(std::io::SeekFrom::Start(start_position))?;
    writer.write_i64::<BigEndian>(length as i64)?;
    writer.seek(std::io::SeekFrom::Start(length))?;

    Ok(())
}

fn write_block<W: Write>(
    writer: &mut W,
    block_data: &[u8],
    block_flags: u32,
) -> Result<(), std::io::Error> {
    if (block_flags & 0x100) > 0 {
        todo!();
    }
    match CompressionType::try_from(block_flags & 0x3F).unwrap() {
        CompressionType::None => {
            writer.write_all(block_data)?;
        }
        CompressionType::Lzma => {
            let mut block_data_reader = block_data;
            lzma_rs::lzma_compress_with_options(
                &mut block_data_reader,
                writer,
                &lzma_rs::compress::Options {
                    unpacked_size: lzma_rs::compress::UnpackedSize::SkipWritingToHeader,
                },
            )?;
        }
        CompressionType::Lz4 => {
            let out = lz4_flex::block::compress(block_data);
            writer.write_all(&out)?;
        }
        CompressionType::Lz4hc => {
            let out = lz4::block::compress(
                block_data,
                Some(lz4::block::CompressionMode::HIGHCOMPRESSION(12)), // TODO 6-12 works so far
                false,
            )?;
            writer.write_all(&out)?;
        }

        CompressionType::Lzham => todo!(),
    }
    Ok(())
}

#[derive(Debug, PartialEq, Eq)]
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

        self.m_BlocksInfo = block_infos;

        for (i, block) in self.m_BlocksInfo.iter().enumerate() {
            decompress_block(reader, writer, block, i, self._decryptor.as_ref())?;
        }

        Ok(directory_infos)
    }
}

fn use_new_archive_flags(m_Header: &BundleFileHeader, config: &ExtractionConfig) -> bool {
    let version = m_Header.get_revision_tuple(config);
    let old = (version < (2020, 0, 0))
        | ((version.0 == 2020) & (version < (2020, 3, 34)))
        | ((version.0 == 2021) & (version < (2021, 3, 2)))
        | ((version.0 == 2022) & (version < (2022, 1, 1)));
    !old
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
    let use_new_archive_flags = use_new_archive_flags(m_Header, config);

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

fn read_chunk<'a, R: Read, const C: usize>(
    reader: &mut R,
    buf: &'a mut [u8; C],
) -> Result<&'a [u8], std::io::Error> {
    let mut total_read = 0;

    while total_read < C {
        match reader.read(&mut buf[total_read..])? {
            0 => break,
            n => total_read += n,
        }
    }

    Ok(&buf[..total_read])
}

fn read_chunk_slice<'a, const C: usize>(reader: &mut &'a [u8]) -> &'a [u8] {
    match reader.split_at_checked(C) {
        Some((start, rest)) => {
            *reader = rest;
            start
        }
        None => {
            let data = &**reader;
            *reader = &[];
            data
        }
    }
}
