use std::io::{self, Read};

pub trait ReadLittleEndian {
    fn read_u8 (&mut self) -> io::Result<u8>;
    fn read_u16 (&mut self) -> io::Result<u16>;
    fn read_u32 (&mut self) -> io::Result<u32>;
}
impl<T: Read> ReadLittleEndian for T {
    fn read_u8 (&mut self) -> io::Result<u8> {
        let mut buf = [0; 1];
        self.read(&mut buf)?;

        Ok(buf[0])
    }
    fn read_u16 (&mut self) -> io::Result<u16>{
        let mut buf = [0; 2];
        self.read(&mut buf)?;

        let b0 = buf[0] as u16;
        let b1 = buf[1] as u16;

        Ok((b0 << 0) | (b1 << 8))
    }
    fn read_u32 (&mut self) -> io::Result<u32> {
        let mut buf = [0; 4];
        self.read(&mut buf)?;

        let b0 = buf[0] as u32;
        let b1 = buf[1] as u32;
        let b2 = buf[2] as u32;
        let b3 = buf[3] as u32;

        Ok(
            (b0 << 0) |
            (b1 << 8) |
            (b2 << 16) |
            (b3 << 24) 
        )
    }

}

pub fn add_u8_clamp (x: u8, y: u8) -> u8 {
    let sum: u16 = (x as u16) + (y as u16);
    if sum > 255 {
        255
    } else {
        sum as u8
    }
}
