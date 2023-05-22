//! little endian.rs
//!

use std::io::Write;


/// Reads in a u16 from a byte array in little endian order
pub fn read_u16 (bytes: &[u8]) -> u16 { 
    let byte0 = bytes[0] as u16;
    let byte1 = bytes[1] as u16;

    let byte0 = byte0 << 0;
    let byte1 = byte1 << 8;

    byte0 + byte1
}


/// Reads in a u32 from a byte array in little endian order
pub fn read_u32 (bytes: &[u8]) -> u32 {
    let byte0 = bytes[0] as u32;
    let byte1 = bytes[1] as u32;
    let byte2 = bytes[2] as u32;
    let byte3 = bytes[3] as u32;

    let byte0 = byte0 << 0;
    let byte1 = byte1 << 8;
    let byte2 = byte2 << 16;
    let byte3 = byte3 << 24;

    byte0 + byte1 + byte2 + byte3
}


/// Creates a byte array from a u16 in little endian order
pub fn write_u16 (x: u16) -> [u8; 2] {
    let byte0 = ((x >> 0) & 0x00FF) as u8;
    let byte1 = ((x >> 8) & 0x00FF) as u8;
    [byte0, byte1]
}


/// Creates a byte array from a u32 in little endian order
pub fn write_u32 (x: u32) -> [u8; 4] {
    let byte0 = ((x >> 0) & 0x00FF) as u8;
    let byte1 = ((x >> 8) & 0x00FF) as u8;
    let byte2 = ((x >> 16) & 0x00FF) as u8;
    let byte3 = ((x >> 24) & 0x00FF) as u8;
    [byte0, byte1, byte2, byte3]
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[should_panic]
    fn read_u16_fail() {
        let bytes = [
            0b00000001
        ];

        let read = read_u16(&bytes);
    }

    #[test]
    fn read_u16_1() {
        let bytes = [
            0b00000001,
            0b00000000
        ];

        let read = read_u16(&bytes);
        assert_eq!(read, 0b00000000_00000001);
    }

    #[test]
    fn read_u16_2() {
        let bytes = [
            0b00000000,
            0b00000001
        ];

        let read = read_u16(&bytes);
        assert_eq!(read, 0b00000001_00000000);
    }

    #[test]
    fn read_u16_3() {
        let bytes = [
            0b01010101,
            0b10101010
        ];

        let read = read_u16(&bytes);
        assert_eq!(read, 0b10101010_01010101);
    }

    #[test]
    fn read_u16_4() {
        let bytes = [
            0b11111111,
            0b11111111
        ];

        let read = read_u16(&bytes);
        assert_eq!(read, 0b11111111_11111111);
    }

    #[test]
    fn read_u16_5() {
        let bytes = [
            0b00000000,
            0b00000000,
        ];

        let read = read_u16(&bytes);
        assert_eq!(read, 0b00000000_00000000);
    }

}
