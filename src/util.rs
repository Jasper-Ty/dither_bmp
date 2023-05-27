use std::io::{self, Read};

pub trait ReadLittleEndian: Read {
    fn read_u8 (&mut self) -> io::Result<u8>;
    fn read_u16 (&mut self) -> io::Result<u16>;
    fn read_u32 (&mut self) -> io::Result<u32>;
}
impl<T: Read> ReadLittleEndian for T {
    fn read_u8 (&mut self) -> io::Result<u8> {
        let mut buf = [0; 1];
        self.read_exact(&mut buf)?;
        Ok(u8::from_le_bytes(buf))
    }
    fn read_u16 (&mut self) -> io::Result<u16>{
        let mut buf = [0; 2];
        self.read_exact(&mut buf)?;
        Ok(u16::from_le_bytes(buf))
    }
    fn read_u32 (&mut self) -> io::Result<u32> {
        let mut buf = [0; 4];
        self.read_exact(&mut buf)?;
        Ok(u32::from_le_bytes(buf))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::cmp;

    struct FakeReader(Vec<u8>);
    impl Read for FakeReader {
        fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
            let min = cmp::min(buf.len(), self.0.len());
            for i in 0..min {
                buf[i] = self.0[i];
            }
            Ok(min)
        }
    }

    #[test]
    fn test_read_u8() {
        let mut r = FakeReader(vec![73]);
        let x: u8 = r.read_u8().unwrap();
        assert_eq!(x, 73);
    }

    #[test]
    fn test_read_u16() {
        let mut r = FakeReader(vec![0xCD, 0xAB]);
        let x: u16 = r.read_u16().unwrap();
        assert_eq!(x, 0xABCD)
    }

    #[test]
    fn test_read_u32() {
        let mut r = FakeReader(vec![0x01, 0xEF, 0xCD, 0xAB]);
        let x: u32 = r.read_u32().unwrap();
        assert_eq!(x, 0xABCDEF01)
    }
}
