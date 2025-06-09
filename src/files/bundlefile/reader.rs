use crate::archive_storage_manager::ArchiveStorageDecryptor;
use crate::config::ExtractionConfig;
use crate::files::bundlefile::{
    BundleFileHeader, BundleSignature, StorageBlock, read_unityfs_info,
};
use crate::files::unityfile::FileEntry;
use std::io::{Read, Seek};

// The chunks of the bundle file are compressed in blocks, but those blocks don't necessarily
// align with the stored `FileEntry`s inside.
pub struct BundleFileReader<R> {
    pub(crate) header: BundleFileHeader,
    pub(crate) decryptor: Option<ArchiveStorageDecryptor>,
    pub(crate) reader: R,

    pub(crate) blocks: std::iter::Enumerate<std::vec::IntoIter<StorageBlock>>,
    pub(crate) files: std::vec::IntoIter<FileEntry>,
    /// Amount of bytes in `scratch` that were returned by `read`, and need to be clear before the next file
    pub(crate) scratch_pending_clear: usize,
    /// Scratch buffer which `read` returns a slice to
    pub(crate) scratch: Vec<u8>,
    /// The current (uncompressed) position for which the scratch buffer stores data
    /// Mainly used when files aren't continuosly next to each other, but have some
    /// space in between them.
    pub(crate) scratch_offset: usize,
}

impl<'r, R: Read + Seek> BundleFileReader<R> {
    pub fn header(&self) -> &BundleFileHeader {
        &self.header
    }

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

        Ok(BundleFileReader {
            header,
            decryptor,
            reader,
            blocks: blocks.into_iter().enumerate(),
            files: files.into_iter(),
            scratch_pending_clear: 0,
            scratch: Vec::new(),
            scratch_offset: 0,
        })
    }

    pub fn next<'s>(&'s mut self) -> Option<BundleFileRef<'s, R>> {
        let file = self.files.next()?;

        Some(BundleFileRef {
            iter: self,
            path: file.path,
            offset: file.offset,
            size: file.size as usize,
        })
    }

    pub fn remaining(&self) -> &[FileEntry] {
        self.files.as_slice()
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
            } else {
                skip_read -= self.iter.scratch.len();
                self.iter.scratch_offset += self.iter.scratch.len();
                self.iter.scratch.clear();
            }
        }

        while let Some((block_index, block)) = self.iter.blocks.next() {
            if skip_read >= block.uncompressed_size as usize {
                skip_read -= block.uncompressed_size as usize;
                self.iter
                    .reader
                    .seek_relative(block.compressed_size as i64)?;
                self.iter.scratch_offset += block.compressed_size as usize;
                continue;
            }

            super::decompress_block(
                self.iter.reader.by_ref(),
                &mut self.iter.scratch,
                &block,
                block_index,
                self.iter.decryptor.as_ref(),
            )?;

            if skip_read > 0 {
                debug_assert!(skip_read <= self.iter.scratch.len());
                discard_front(&mut self.iter.scratch, skip_read);
                skip_read = 0;
            }

            if self.iter.scratch.len() >= self.size {
                break;
            }
        }

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
