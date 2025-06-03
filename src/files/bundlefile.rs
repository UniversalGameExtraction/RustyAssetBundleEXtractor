use crate::{
    archive_storage_manager::ArchiveStorageDecryptor,
    config::ExtractionConfig,
    files::serialzedfile::SerializedFile,
    files::unityfile::{FileEntry, UnityFile},
    read_ext::{ReadSeekUrexExt, ReadUrexExt},
};
use bitflags::bitflags;
use byteorder::{BigEndian, ReadBytesExt};
use num_enum::TryFromPrimitive;
use std::io::{Error, Read, Seek, Write};
use std::str::FromStr;

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
    pub unity_revision: String,
    pub size: u32,
}

impl BundleFileHeader {
    pub fn from_reader<T: Read + Seek>(reader: &mut T) -> Result<Self, Error> {
        Ok(BundleFileHeader {
            signature: reader.read_cstr().unwrap().parse().unwrap(),
            version: reader.read_u32::<BigEndian>().unwrap(),
            unity_version: reader.read_cstr().unwrap(),
            unity_revision: reader.read_cstr().unwrap(),
            size: 0,
        })
    }

    fn get_revision_tuple(&self, config: &ExtractionConfig) -> (u32, u32, u32) {
        // could be done way better, but this works for now
        let mut revision = self.unity_revision.clone();
        if revision.is_empty() | (revision == "0.0.0") {
            revision = config.fallback_unity_version.clone();
        }
        let mut revision_split = revision.split('.');
        (
            revision_split.next().unwrap().parse().unwrap(),
            revision_split.next().unwrap().parse().unwrap(),
            {
                let mut val = 0;
                let last_split = revision_split.next().unwrap();
                for (i, c) in last_split.chars().enumerate() {
                    if !c.is_numeric() {
                        val = last_split[..i].parse::<u32>().unwrap();
                        break;
                    }
                }
                val
            },
        )
    }
}

pub struct StorageBlock {
    pub compressed_size: u32,
    pub uncompressed_size: u32,
    pub flags: u32,
}

pub struct BundleFileReader<R> {
    m_Header: BundleFileHeader,
    decryptor: Option<ArchiveStorageDecryptor>,
    reader: R,
}
impl<R: Read + Seek> BundleFileReader<R> {
    pub fn from_reader(mut reader: R, config: &ExtractionConfig) -> Result<Self, Error> {
        Ok(BundleFileReader {
            m_Header: BundleFileHeader::from_reader(&mut reader)?,
            decryptor: None,
            reader,
        })
    }
    pub fn iter<'r>(
        &mut self,
        config: &ExtractionConfig,
    ) -> Result<iter::BundleIterator<'_, R>, std::io::Error> {
        match self.m_Header.signature {
            BundleSignature::UnityFS => {}
            _ => unimplemented!(),
        };
        let (blocks, files) = self.read_unityfs_info(config)?;

        Ok(iter::BundleIterator {
            bundle_file: self,
            blocks: blocks.into_iter().enumerate(),
            files: files.into_iter(),
            skip_read: 0,
            scratch_pending_clear: 0,
            scratch: Vec::new(),
        })
    }

    pub fn read_unityfs_info(
        &mut self,
        config: &ExtractionConfig,
    ) -> Result<(Vec<StorageBlock>, Vec<FileEntry>), Error> {
        let use_new_archive_flags = !{
            let version = self.m_Header.get_revision_tuple(config);
            (version < (2020, 0, 0))
                | ((version.0 == 2020) & (version < (2020, 3, 34)))
                | ((version.0 == 2021) & (version < (2021, 3, 2)))
                | ((version.0 == 2022) & (version < (2022, 1, 1)))
        };

        //ReadHeader
        self.m_Header.size = self.reader.read_i64::<BigEndian>()? as u32;

        let block_info = StorageBlock {
            compressed_size: self.reader.read_u32::<BigEndian>()?,
            uncompressed_size: self.reader.read_u32::<BigEndian>()?,
            flags: self.reader.read_u32::<BigEndian>()?,
        };
        if let BundleSignature::UnityFS = self.m_Header.signature {
            self.reader.read_bool()?;
        }

        //ReadBlocksInfoAndDirectory
        // TODO - check for 2019.4, which is version 6
        if self.m_Header.version >= 7 {
            self.reader.align(16)?;
        }

        let mut blocks_info_bytes = Vec::with_capacity(block_info.uncompressed_size as usize);
        if block_info.flags & ArchiveFlags::BLOCKS_INFO_AT_THE_END.bits() != 0 {
            // 0x80 BlocksInfoAtTheEnd
            let position = self.reader.stream_position()?;
            // originally reader.length
            self.reader
                .seek(std::io::SeekFrom::End(block_info.compressed_size as i64))?;
            self.decompress_block(&mut blocks_info_bytes, &block_info, 0)?;
            self.reader.seek(std::io::SeekFrom::Start(position))?;
        } else {
            // 0x40 BlocksAndDirectoryInfoCombined
            if (use_new_archive_flags
                & (block_info.flags & ArchiveFlags::USES_ASSET_BUNDLE_ENCRYPTION.bits() > 0))
                | (!use_new_archive_flags
                    & (block_info.flags & ArchiveFlagsOld::USES_ASSET_BUNDLE_ENCRYPTION.bits() > 0))
            {
                self.decryptor = Some(ArchiveStorageDecryptor::from_reader(
                    &mut self.reader,
                    config.unitycn_key.unwrap(),
                )?);
            }
            self.decompress_block(&mut blocks_info_bytes, &block_info, 0)?;
        }

        let block_info_reader = &mut blocks_info_bytes.as_slice();
        let uncompressed_data_hash = block_info_reader.read_u128::<BigEndian>()?;

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
            self.reader.align(16)?;
        }

        Ok((m_BlocksInfo, m_DirectoryInfo))
    }

    pub fn decompress_block<W: Write>(
        &mut self,
        writer: &mut W,
        block: &StorageBlock,
        index: usize,
    ) -> Result<(), Error> {
        let mut compressed = self
            .reader
            .read_bytes_sized(block.compressed_size as usize)
            .unwrap();
        match CompressionType::try_from(block.flags & 0x3F).unwrap() {
            CompressionType::Lzma => {
                let mut compressed_reader = std::io::Cursor::new(&compressed);
                lzma_rs::lzma_decompress(&mut compressed_reader, writer).unwrap();
                Ok(())
            }
            CompressionType::Lz4 | CompressionType::Lz4hc => {
                if block.flags & 0x100 > 0 {
                    // UnityCN encryption
                    self.decryptor.as_ref().unwrap().decrypt_block(
                        &mut compressed,
                        block.compressed_size as usize,
                        index,
                    )?;
                }
                let data =
                    lz4_flex::block::decompress(&compressed, block.uncompressed_size as usize)
                        .unwrap();
                writer.write_all(&data)?;
                Ok(())
            }
            CompressionType::Lzham => {
                panic!("LZHAM is not supported");
            }
            CompressionType::None => {
                writer.write_all(&compressed)?;
                Ok(())
            }
        }
    }
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
        config: &ExtractionConfig,
    ) -> Result<(Vec<FileEntry>, Vec<u8>), Error> {
        if self.m_Header.version >= 4 {
            let hash = reader.read_u128::<BigEndian>().unwrap();
            let crc = reader.read_u32::<BigEndian>().unwrap();
        }
        let minimum_streamed_bytes = reader.read_u32::<BigEndian>().unwrap();

        self.m_Header.size = reader.read_u32::<BigEndian>().unwrap();

        let number_of_levels_to_download_before_streaming = reader.read_u32::<BigEndian>().unwrap();
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
            let complete_file_size = reader.read_u32::<BigEndian>().unwrap();
        }
        if self.m_Header.version >= 3 {
            let file_info_header_size = reader.read_u128::<BigEndian>().unwrap();
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

        self.decompress_block(reader, &mut blocks_info_bytes, &m_BlocksInfo, 0)?;
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
        // TODO: this is double reading the header
        let mut reader = BundleFileReader::from_reader(reader, config)?;
        let (block_infos, directory_infos) = reader.read_unityfs_info(config)?;

        for (i, block) in block_infos.iter().enumerate() {
            reader.decompress_block(writer, block, i)?;
        }

        Ok(directory_infos)
    }

    fn read_files<T: Read + Seek>(
        &mut self,
        FileEntrys: Vec<FileEntry>,
        reader: &mut T,
        config: &ExtractionConfig,
    ) -> Result<(), Error> {
        for fileentry in FileEntrys {
            reader
                .seek(std::io::SeekFrom::Start(fileentry.offset as u64))
                .unwrap();
            let serialized_res = SerializedFile::from_reader(reader, config);
            match serialized_res {
                Ok(serialized) => print!("{:?}", serialized),
                Err(error) => print!("{:?}", error),
            };
        }
        Ok(())
    }

    pub fn decompress_block<T: Read + Seek, W: Write>(
        &mut self,
        reader: &mut T,
        writer: &mut W,
        block: &StorageBlock,
        index: usize,
    ) -> Result<(), Error> {
        let mut compressed = reader
            .read_bytes_sized(block.compressed_size as usize)
            .unwrap();
        match CompressionType::try_from(block.flags & 0x3F).unwrap() {
            CompressionType::Lzma => {
                let mut compressed_reader = std::io::Cursor::new(&compressed);
                lzma_rs::lzma_decompress(&mut compressed_reader, writer).unwrap();
                Ok(())
            }
            CompressionType::Lz4 | CompressionType::Lz4hc => {
                if block.flags & 0x100 > 0 {
                    // UnityCN encryption
                    self._decryptor.as_ref().unwrap().decrypt_block(
                        &mut compressed,
                        block.compressed_size as usize,
                        index,
                    )?;
                }
                let data =
                    lz4_flex::block::decompress(&compressed, block.uncompressed_size as usize)
                        .unwrap();
                writer.write_all(&data)?;
                Ok(())
            }
            CompressionType::Lzham => {
                panic!("LZHAM is not supported");
            }
            CompressionType::None => {
                writer.write_all(&compressed)?;
                Ok(())
            }
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

mod iter {
    use crate::files::bundlefile::{BundleFileReader, StorageBlock};
    use crate::files::unityfile::FileEntry;
    use std::io::{Read, Seek};

    pub struct BundleFileRef<'s, 'a, R> {
        pub path: String,
        pub size: usize,
        iter: &'s mut BundleIterator<'a, R>,
        offset: i64,
    }

    impl<R: Read + Seek> BundleFileRef<'_, '_, R> {
        pub fn read(&mut self) -> Result<&[u8], std::io::Error> {
            self.iter.skip_read -= self.size;

            // clear part of scratch that was already read
            discard_front(&mut self.iter.scratch, self.iter.scratch_pending_clear);
            self.iter.scratch_pending_clear = 0;

            if self.iter.skip_read > 0 {
                self.iter.skip_read -= self.iter.scratch.len();
                self.iter.scratch.clear();
            }

            while let Some((block_index, block)) = self.iter.blocks.next() {
                if self.iter.skip_read >= block.uncompressed_size as usize {
                    self.iter.skip_read -= block.uncompressed_size as usize;
                    self.iter
                        .bundle_file
                        .reader
                        .seek_relative(block.compressed_size as i64)?;
                    continue;
                }

                self.iter.bundle_file.decompress_block(
                    &mut self.iter.scratch,
                    &block,
                    block_index,
                )?;

                if self.iter.skip_read > 0 {
                    debug_assert!(self.iter.skip_read <= self.iter.scratch.len());
                    discard_front(&mut self.iter.scratch, self.iter.skip_read);
                    self.iter.skip_read = 0;
                }

                if self.iter.scratch.len() >= self.size {
                    break;
                }
            }

            self.iter.scratch_pending_clear = self.size;
            Ok(&self.iter.scratch[..self.size])
        }
    }

    /// The chunks of the bundle file are compressed in blocks, but those blocks don't necessarily
    /// align with the stored `FileEntry`s inside.
    /// So when you decode a block, you may get more data than required for the current file.
    /// This is managed using te
    pub struct BundleIterator<'a, R> {
        pub(crate) bundle_file: &'a mut BundleFileReader<R>,
        pub(crate) blocks: std::iter::Enumerate<std::vec::IntoIter<StorageBlock>>,
        pub(crate) files: std::vec::IntoIter<FileEntry>,
        /// When you call `BundleIterator::next` without calling `BundleFileRef::read`,
        /// we can skip decoding some block, but need to advance the underlying reader.
        /// To track this, `skip_read` is incremented by the file size in `next`,
        /// and substracted when `read` is actually called.
        pub(crate) skip_read: usize,
        /// Amount of bytes in `scratch` that were returned by `read`, and need to be clear before the next file
        pub(crate) scratch_pending_clear: usize,
        /// Scratch buffer which `read` returns a slice to
        pub(crate) scratch: Vec<u8>,
    }

    impl<'a, 'r, R: Read + Seek> BundleIterator<'a, R> {
        pub fn next<'s>(&'s mut self) -> Option<BundleFileRef<'s, 'a, R>> {
            let file = self.files.next()?;
            self.skip_read += file.size as usize;

            Some(BundleFileRef {
                iter: self,
                path: file.path,
                offset: file.offset,
                size: file.size as usize,
            })
        }
    }

    /// Removes the first <begin> bytes of the vector
    fn discard_front(vec: &mut Vec<u8>, begin: usize) {
        let len = vec.len();
        if begin >= len {
            if len != 0 {
                panic!(
                    "Tried to discard_front {begin} bytes, length is only {}",
                    vec.len()
                )
            }
        } else {
            let rest_len = len - begin;
            vec.copy_within(begin.., 0);
            vec.truncate(rest_len);
        }
    }
}
