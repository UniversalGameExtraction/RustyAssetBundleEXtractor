// credits to: https://github.com/Razmoth/CNStudio/blob/master/AssetStudio/CNUnity.cs
use crate::read_ext::ReadUrexExt;
use aes::cipher::{block_padding::NoPadding, BlockEncryptMut, KeyIvInit};
use byteorder::{BigEndian, ReadBytesExt};
use std::io::{Read, Result, Seek, SeekFrom};
type Aes128CbcEnc = cbc::Encryptor<aes::Aes128>;

// #$unity3dchina!@
const UNITY3D_SIGNATURE: [u8; 16] = [
    35, 36, 117, 110, 105, 116, 121, 51, 100, 99, 104, 105, 110, 97, 33, 64,
];

fn decrypt_key(key: [u8; 16], data: [u8; 16], archive_key: [u8; 16]) -> [u8; 16] {
    let mut _archive_key: [u8; 16] = [0; 16];
    _archive_key.copy_from_slice(&archive_key);
    let mut _key: [u8; 16] = [0; 16];
    _key.copy_from_slice(&key);
    let _iv: [u8; 16] = [0; 16];

    let encryptor = Aes128CbcEnc::new(&_archive_key.into(), &_iv.into());
    encryptor
        .encrypt_padded_mut::<NoPadding>(&mut _key, 16)
        .unwrap();

    for i in 0..16 {
        _key[i] ^= data[i];
    }
    _key
}

fn to_uint4_array(source: &[u8; 16], offset: usize) -> [u8; 32] {
    let mut result = [0; 32];
    for i in 0..16 {
        result[i * 2] = source[offset + i] >> 4;
        result[i * 2 + 1] = source[offset + i] & 15;
    }
    result
}

pub struct ArchiveStorageDecryptor {
    index: [u8; 16],
    sub: [u8; 16],
}

impl ArchiveStorageDecryptor {
    pub fn from_reader<T: Read + Seek>(reader: &mut T, archive_key: [u8; 16]) -> Result<Self> {
        let unknown = reader.read_u32::<BigEndian>().unwrap();

        let info_bytes: [u8; 16] = reader.read_bytes_sized(16)?.try_into().unwrap();
        let info_key: [u8; 16] = reader.read_bytes_sized(16)?.try_into().unwrap();
        reader.seek(SeekFrom::Current(1)).unwrap();

        let signature_bytes: [u8; 16] = reader.read_bytes_sized(16)?.try_into().unwrap();
        let signature_key: [u8; 16] = reader.read_bytes_sized(16)?.try_into().unwrap();
        reader.seek(SeekFrom::Current(1)).unwrap();

        let signature: [u8; 16] = decrypt_key(signature_key, signature_bytes, archive_key);
        assert_eq!(signature, UNITY3D_SIGNATURE);

        let data = decrypt_key(info_key, info_bytes, archive_key);
        let data = to_uint4_array(&data, 0);
        let index: [u8; 16] = data[0..16].try_into().unwrap();
        let mut sub: [u8; 16] = [0; 16];
        for j in 0..4 {
            for i in 0..4 {
                sub[i + j * 4] = data[16 + i * 4 + j];
            }
        }
        Ok(ArchiveStorageDecryptor { index, sub })
    }

    // Decrypts a StorageBlock in place
    pub fn decrypt_block(&self, bytes: &mut [u8], size: usize, index: usize) -> Result<()> {
        let mut index = index;
        let mut offset = 0;

        let data_sum: usize = bytes.iter().fold(0, |acc, x| acc + *x as usize);

        while offset < size {
            offset = self.decrypt(bytes, offset, index, size)?;
            index += 1;
        }
        Ok(())
    }

    fn decrypt_byte(&self, bytes: &mut [u8], offset: usize, index: usize) -> (u8, usize, usize) {
        let b: [u8; 4] = [
            self.sub[((index >> 2) & 3) + 4],
            self.sub[index & 3],
            self.sub[((index >> 4) & 3) + 8],
            self.sub[((index % 256) >> 6) + 12],
        ];
        let b: u8 = b.iter().fold(0u8, |acc: u8, x| acc.overflowing_add(*x).0);

        let byte_low = self.index[(bytes[offset] & 0xF) as usize]
            .overflowing_sub(b)
            .0;
        let byte_high = self.index[(bytes[offset] >> 4) as usize]
            .overflowing_sub(b)
            .0;
        bytes[offset] = (byte_low & 0xF) | (byte_high << 4);
        (bytes[offset], offset + 1, index + 1)
    }

    fn decrypt(&self, bytes: &mut [u8], offset: usize, index: usize, end: usize) -> Result<usize> {
        let (cur_byte, mut offset, mut index) = self.decrypt_byte(bytes, offset, index);
        // u24, split into u16 and u8 here for easier handling
        let mut byte_high: u16 = (cur_byte >> 4) as u16;
        let byte_low = cur_byte & 0xF;

        if byte_high == 0xF {
            let mut b: u8 = 0xFF;
            while b == 0xFF {
                (b, offset, index) = self.decrypt_byte(bytes, offset, index);
                byte_high += b as u16;
            }
        }

        offset += byte_high as usize;

        if offset < end {
            (_, offset, index) = self.decrypt_byte(bytes, offset, index);
            (_, offset, index) = self.decrypt_byte(bytes, offset, index);
            if byte_low == 0xF {
                let mut b: u8 = 0xFF;
                while b == 0xFF {
                    (b, offset, index) = self.decrypt_byte(bytes, offset, index);
                }
            }
        }

        Ok(offset)
    }
}
