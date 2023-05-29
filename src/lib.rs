pub mod bmp;
pub mod util;
pub mod surface;
pub mod dither;
pub mod pixel;
pub mod quantize;

use std::{
    fs::File,
    io::{ 
        Write,
        Seek, 
        SeekFrom,
        Result,
    }
};

use surface::Surface;
use pixel::RGB;
use bmp::BmpInfo;
use util::ReadLittleEndian;

pub fn read_rgb(info: &BmpInfo, f: &mut File) -> Result<Surface<RGB<i32>>> {
    let BmpInfo { offset, width, height, bits_per_pixel:_ } = *info;
    let pack_len = info.width*3;
    let full_len = match (pack_len) % 4 {
        0 => pack_len,
        r => pack_len-r+4,
    } as u64;

    let mut data: Vec<RGB<i32>> = Vec::with_capacity((width*height) as usize);

    for y in 0..height {
        let offset = offset + (y as u64)*full_len;
        f.seek(SeekFrom::Start(offset))?;
        for _ in 0..width {
            data.push(RGB(
                f.read_u8()? as i32,
                f.read_u8()? as i32,
                f.read_u8()? as i32,
            ));
        }
    }

    let mut surface = Surface::new(width, height, data);

    Ok(surface)
}

pub fn write_rgb(info: &BmpInfo, f: &mut File, surface: &Surface<RGB<i32>>) -> Result<()> {
    let pack_len = info.width*3;
    let padding_bytes = match (pack_len) % 4 {
        0 => vec![],
        r => vec![0u8; (4-r) as usize]
    };

    let mut buf = [0u8; 3];
    for y in 0..info.height {
        for x in 0..info.width {
            let RGB(r, g, b) = *surface.get(x as i32, y as i32).unwrap();
            buf[0] = r.clamp(0, 255) as u8;
            buf[1] = g.clamp(0, 255) as u8;
            buf[2] = b.clamp(0, 255) as u8;
            f.write(&buf)?;
        }
        f.write(&padding_bytes)?;
    }

    Ok(())
}
