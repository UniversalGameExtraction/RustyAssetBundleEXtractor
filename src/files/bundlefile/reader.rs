use crate::archive_storage_manager::ArchiveStorageDecryptor;
use crate::files::bundlefile::{
    BundleFileHeader, BundleSignature, StorageBlock, read_unityfs_info,
};
use crate::files::unityfile::FileEntry;
use std::io::{Read, Seek, SeekFrom};

use super::ExtractionConfig;

// The chunks of the bundle file are compressed in blocks, but those blocks don't necessarily
// align with the stored `FileEntry`s inside.
pub struct BundleFileReader<R> {
    pub(crate) header: BundleFileHeader,
    pub(crate) decryptor: Option<ArchiveStorageDecryptor>,
    pub(crate) reader: R,
    /// The position in the reader after the header where the file data starts
    pub(crate) data_offset: u64,

    pub(crate) blocks: Vec<StorageBlock>,
    pub(crate) files: Vec<FileEntry>,
    pub(crate) block_index: usize,
    pub(crate) file_index: usize,
    /// Amount of bytes in `scratch` that were returned by `read`, and need to be clear before the next file
    pub(crate) scratch_pending_clear: usize,
    /// Scratch buffer which `read` returns a slice to
    pub(crate) scratch: Vec<u8>,
    /// The current (uncompressed) position for which the scratch buffer stores data
    /// Mainly used when files aren't continuosly next to each other, but have some
    /// space in between them.
    pub(crate) scratch_offset: usize,
}

impl<R: Read + Seek> BundleFileReader<R> {
    /// Read the bundlefile from an I/O stream.
    pub fn from_reader(
        mut reader: R,
        config: &ExtractionConfig,
    ) -> Result<BundleFileReader<R>, std::io::Error> {
        let mut header = BundleFileHeader::from_reader(&mut reader)?;
        match header.signature {
            BundleSignature::UnityFS => {}
            _ => unimplemented!(),
        };
        let (blocks, files, decryptor) = read_unityfs_info(&mut header, reader.by_ref(), config)?;

        let data_offset = reader.stream_position()?;
        Ok(BundleFileReader {
            header,
            decryptor,
            reader,
            data_offset,
            blocks,
            files,
            block_index: 0,
            file_index: 0,
            scratch_pending_clear: 0,
            scratch: Vec::new(),
            scratch_offset: 0,
        })
    }

    /// Advance the reader to the next file entry. Call [`BundleFileRef::read`] to get the actual data.
    pub fn next(&mut self) -> Option<BundleFileRef<'_, R>> {
        let file = self.files.get(self.file_index)?.clone();
        self.file_index += 1;

        Some(BundleFileRef {
            iter: self,
            path: file.path,
            offset: file.offset,
            size: file.size as usize,
        })
    }

    /// Go back to the start of the file, so that [`BundleFileReader::next`] will yield the first file.
    pub fn reset(&mut self) -> Result<(), std::io::Error> {
        self.block_index = 0;
        self.file_index = 0;
        self.scratch_pending_clear = 0;
        self.scratch.clear();
        self.scratch_offset = 0;
        self.reader.seek(SeekFrom::Start(self.data_offset))?;
        Ok(())
    }

    /// Get a [`BundleFileRef`] to an arbitrary file in the bundle by path.
    pub fn seek_file(
        &mut self,
        file_path: &str,
    ) -> Option<Result<BundleFileRef<'_, R>, std::io::Error>> {
        self.files()
            .iter()
            .enumerate()
            .find_map(|(file_index, file)| {
                (file.path == file_path).then(|| (file_index, file.clone()))
            })
            .map(|(file_index, file_entry)| self.seek_file_inner(file_index, file_entry.clone()))
    }

    fn seek_file_inner(
        &mut self,
        file_index: usize,
        file: FileEntry,
    ) -> Result<BundleFileRef<'_, R>, std::io::Error> {
        let mut block_offset_uncompressed = 0;
        let mut block_offset_compressed = 0;
        let mut block_index = 0;

        // PERF: this linear scan on every seek could be avoided if the file block offsets
        // were computed and saved once at creation
        for block in &self.blocks {
            if block_offset_uncompressed + block.uncompressed_size as u64 > file.offset as u64 {
                break;
            }

            block_offset_uncompressed += block.uncompressed_size as u64;
            block_offset_compressed += block.compressed_size as u64;
            block_index += 1;
        }

        self.block_index = block_index;
        self.file_index = file_index;
        self.scratch_pending_clear = 0;
        self.scratch.clear();
        self.scratch_offset = block_offset_uncompressed as usize;
        self.reader
            .seek(SeekFrom::Start(self.data_offset + block_offset_compressed))?;

        Ok(BundleFileRef {
            path: file.path,
            size: file.size as usize,
            iter: self,
            offset: file.offset,
        })
    }

    pub fn header(&self) -> &BundleFileHeader {
        &self.header
    }

    pub fn files(&self) -> &[FileEntry] {
        &self.files
    }

    pub fn blocks(&self) -> &[StorageBlock] {
        &self.blocks
    }

    pub fn remaining(&self) -> &[FileEntry] {
        &self.files[self.file_index..]
    }
}

pub struct BundleFileRef<'s, R> {
    pub path: String,
    pub size: usize,
    iter: &'s mut BundleFileReader<R>,
    offset: i64,
}

impl<'s, R: std::fmt::Debug> std::fmt::Debug for BundleFileRef<'s, R> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("BundleFileRef")
            .field("path", &self.path)
            .field("size", &self.size)
            .field("offset", &self.offset)
            .finish()
    }
}

impl<R: Read + Seek> BundleFileRef<'_, R> {
    pub fn read(&mut self) -> Result<&[u8], std::io::Error> {
        // clear part of scratch that was already read
        discard_front(&mut self.iter.scratch, self.iter.scratch_pending_clear);
        self.iter.scratch_offset += self.iter.scratch_pending_clear;
        self.iter.scratch_pending_clear = 0;

        // This is nonzero if you call `BundleIterator::next` without reading the file,
        // OR if two files in the asset bundle aren't actually contiguous and there's a gap.
        let mut skip_read = self.offset as usize - self.iter.scratch_offset;

        if skip_read > 0 {
            if self.iter.scratch.len() > skip_read {
                discard_front(&mut self.iter.scratch, skip_read);
                self.iter.scratch_offset += skip_read;
                skip_read = 0;
            } else if !self.iter.scratch.is_empty() {
                skip_read -= self.iter.scratch.len();
                self.iter.scratch_offset += self.iter.scratch.len();
                self.iter.scratch.clear();
            }
        }

        if self.iter.scratch.len() < self.size {
            while let Some(block) = self.iter.blocks.get(self.iter.block_index) {
                let block_index = self.iter.block_index;
                self.iter.block_index += 1;

                if skip_read >= block.uncompressed_size as usize {
                    skip_read -= block.uncompressed_size as usize;
                    self.iter
                        .reader
                        .seek_relative(block.compressed_size as i64)?;
                    self.iter.scratch_offset += block.uncompressed_size as usize;
                    continue;
                }

                super::decompress_block(
                    self.iter.reader.by_ref(),
                    &mut self.iter.scratch,
                    block,
                    block_index,
                    self.iter.decryptor.as_ref(),
                )?;

                if skip_read > 0 {
                    debug_assert!(skip_read <= self.iter.scratch.len());
                    discard_front(&mut self.iter.scratch, skip_read);
                    self.iter.scratch_offset += skip_read;
                    skip_read = 0;
                }

                if self.iter.scratch.len() >= self.size {
                    break;
                }
            }
        }

        assert_eq!(self.iter.scratch_offset, self.offset as usize);
        self.iter.scratch_pending_clear = self.size;
        Ok(&self.iter.scratch[..self.size])
    }
}

/// Removes the first <begin> bytes of the vector
fn discard_front(vec: &mut Vec<u8>, begin: usize) {
    if begin == 0 {
        return;
    }

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
