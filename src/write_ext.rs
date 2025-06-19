use std::io::{Result, Seek, Write};
pub trait WriteExt: Write {
    fn write_cstr(&mut self, str: &str) -> Result<()> {
        self.write_all(str.as_bytes())?;
        self.write_all(b"\0")?;
        Ok(())
    }
}

impl<T: Write> WriteExt for T {}

pub trait WriteSeekExt: Write + Seek {
    fn align<const ALIGN: usize>(&mut self) -> Result<()> {
        let pos = self.stream_position()?;
        let new_pos = (pos + ALIGN as u64 - 1) & !(ALIGN as u64 - 1);
        let diff = new_pos - pos;
        if diff > 0 {
            self.write_all(&[0; ALIGN][..diff as usize])?;
        }

        Ok(())
    }
}

impl<T: Write + Seek> WriteSeekExt for T {}

pub fn align_vec<const ALIGN: usize>(vec: &mut Vec<u8>) {
    let new_pos = (vec.len() + ALIGN as usize - 1) & !(ALIGN as usize - 1);
    vec.resize_with(new_pos, || 0);
}
