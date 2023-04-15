use std::io::Seek;

use byteorder::{ByteOrder, ReadBytesExt};

pub trait ReadUrexExt: ReadBytesExt {
    fn read_array_len<T: ByteOrder>(&mut self) -> Result<usize, std::io::Error> {
        let len = self.read_u32::<T>()?;
        Ok(len as usize)
    }

    fn read_cstr(&mut self) -> Result<String, std::io::Error> {
        let mut bytes = Vec::new();
        loop {
            let byte = self.read_u8()?;
            if byte == 0 {
                break;
            }
            bytes.push(byte);
        }
        Ok(String::from_utf8(bytes).unwrap())
    }

    fn read_bytes_sized(&mut self, len: usize) -> Result<Vec<u8>, std::io::Error> {
        let mut buf = vec![0; len];
        self.read_exact(&mut buf)?;
        Ok(buf)
    }

    fn read_string<T: ByteOrder>(&mut self) -> Result<String, std::io::Error> {
        let len = self.read_array_len::<T>()?;
        let mut buf = vec![0; len];
        self.read_exact(&mut buf)?;
        let s = String::from_utf8(buf).unwrap();
        Ok(s)
    }

    fn read_string_sized(&mut self, len: usize) -> Result<String, std::io::Error> {
        let mut buf = vec![0; len];
        self.read_exact(&mut buf)?;
        let s = String::from_utf8(buf).unwrap();
        Ok(s)
    }

    fn read_bool(&mut self) -> Result<bool, std::io::Error> {
        let b = self.read_u8()?;
        Ok(b != 0)
    }
}

pub trait ReadSeekUrexExt: ReadUrexExt + Seek{
    fn align(&mut self, align: usize) -> Result<(), std::io::Error> {
        let pos = self.stream_position()?;
        let new_pos = (pos + align as u64 - 1) & !(align as u64 - 1);
        let diff = new_pos - pos;
        if diff > 0 {
            self.seek(std::io::SeekFrom::Current(diff as i64))?;
        }
        Ok(())
    }
}

impl<R: std::io::Read + ?Sized> ReadUrexExt for R {}
impl<R: std::io::Read + Seek + ?Sized> ReadSeekUrexExt for R {}
