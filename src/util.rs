use std::io::{self, Read};

pub fn read_u16 (r: &mut impl Read) -> io::Result<u16> {
    let mut buf = [0; 2];
    r.read(&mut buf)?;

    let b0 = buf[0] as u16;
    let b1 = buf[1] as u16;

    Ok((b0 << 0) | (b1 << 8))
}

pub fn read_u32 (r: &mut impl Read) -> io::Result<u32> {
    let mut buf = [0; 4];
    r.read(&mut buf)?;

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

pub fn add_u8_clamp (x: u8, y: u8) -> u8 {
    let sum: u16 = (x as u16) + (y as u16);
    if sum > 255 {
        255
    } else {
        sum as u8
    }
}
