//! utility functions

/// Reads in a u16 from a byte array in little endian order
pub fn read_u16 (bytes: &[u8]) -> u16 { 
    let byte0 = bytes[0] as u16;
    let byte1 = bytes[1] as u16;

    let byte0 = byte0 << 0;
    let byte1 = byte1 << 8;

    byte0 + byte1
}

/// Reads in a u32 from a byte array in little endian order
pub fn read_u32 (bytes : &[u8]) -> u32 {
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

pub fn u8_sum_clamp (x: u8, y: u8) -> u8 {
    let sum: u16 = (x as u16) + (y as u16);
    if sum >= u8::MAX as u16 { 
        u8::MAX 
    } else { 
        sum as u8 
    }
}

pub fn u8_diff_clamp (x: u8, y: u8) -> u8 {
    let diff: i16 = (x as i16) - (y as i16);
    if diff < 0 { 
        0
    } else { 
        diff as u8 
    }
}

pub fn u8_scale256 (x: u8, numerator: u8, denominator: u8) -> u8 {
    if numerator > denominator {
        panic!("u8_scale_256 called with numerator > denominator!");
    }
    let res = x as u16;
    let res = res * (numerator as u16);
    let res = res / (denominator as u16);
    res as u8
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


    #[test]
    fn u8_sum_clamp_1() {
        let a: u8 = 109;
        let b: u8 = 32;

        let c: u8 = u8_sum_clamp (a, b);

        assert_eq! (c, 141);
    }

    #[test]
    fn u8_sum_clamp_2() {
        let a: u8 = 0;
        let b: u8 = 32;

        let c: u8 = u8_sum_clamp (a, b);

        assert_eq! (c, 32);
    }

    #[test]
    fn u8_sum_clamp_3() {
        let a: u8 = 109;
        let b: u8 = 0;

        let c: u8 = u8_sum_clamp (a, b);

        assert_eq! (c, 109);
    }

    #[test]
    fn u8_sum_clamp_4() {
        let a: u8 = 134;
        let b: u8 = 159;

        let c: u8 = u8_sum_clamp (a, b);

        assert_eq! (c, 255);
    }

    #[test]
    fn u8_sum_clamp_5() {
        let a: u8 = 255;
        let b: u8 = 255;

        let c: u8 = u8_sum_clamp (a, b);

        assert_eq! (c, 255);
    }

    #[test]
    fn u8_diff_clamp_1() {
        let a: u8 = 109;
        let b: u8 = 32;

        let c: u8 = u8_diff_clamp (a, b);

        assert_eq!(c, 77);
    }

    #[test]
    fn u8_diff_clamp_2() {
        let a: u8 = 0; 
        let b: u8 = 255;

        let c: u8 = u8_diff_clamp (a, b);

        assert_eq!(c, 0);
    }

    #[test]
    fn u8_diff_clamp_3() {
        let a: u8 = 0;
        let b: u8 = 129;

        let c: u8 = u8_diff_clamp (a, b);

        assert_eq!(c, 0);
    }

    #[test]
    fn u8_diff_clamp_4() {
        let a: u8 = 0;
        let b: u8 = 0;

        let c: u8 = u8_diff_clamp (a, b);

        assert_eq!(c, 0);
    }

    #[test] fn u8_diff_clamp_5() {
        let a: u8 = 255;
        let b: u8 = 0;

        let c: u8 = u8_diff_clamp (a, b);

        assert_eq!(c, 255);
    }

    #[test]
    fn u8_scale256_1() {
        let a: u8 = 255;
        let p: u8 = 7;
        let q: u8 = 16;

        let c: u8 = u8_scale256 (a, p, q);

        assert_eq!(c, 111);
    }
    
    #[test]
    fn u8_scale256_2() {
        let a: u8 = 255;
        let p: u8 = 34; 
        let q: u8 = 169;

        let c: u8 = u8_scale256 (a, p, q);

        assert_eq!(c, 51);
    }

    #[test]
    fn u8_scale256_3() {
        let a: u8 = 158;
        let p: u8 = 4;
        let q: u8 = 19;

        let c: u8 = u8_scale256 (a, p, q);

        assert_eq!(c, 33);
    }
    
    #[test]
    fn u8_scale256_4() {
        let a: u8 = 2;
        let p: u8 = 1;
        let q: u8 = 2;

        let c: u8 = u8_scale256 (a, p, q);

        assert_eq!(c, 1);
    }

    #[test]
    fn u8_scale256_5() {
        let a: u8 = 10;
        let p: u8 = 5;
        let q: u8 = 5;

        let c: u8 = u8_scale256 (a, p, q);

        assert_eq!(c, 10);
    }

    #[test]
    #[should_panic]
    fn u8_scale256_fail() {
        let a: u8 = 10;
        let p: u8 = 9;
        let q: u8 = 5;

        let c: u8 = u8_scale256 (a, p, q);
    }
}
