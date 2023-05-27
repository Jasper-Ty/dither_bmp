pub mod bmp;
pub mod util;
pub mod surface;
pub mod dither;
pub mod clamped;
pub mod pixel;
pub mod quantize;

use surface::Surface;
use pixel::Pix;
use std::fs::File;
use std::io::{ 
    self, 
    Write,
    Seek, 
    SeekFrom 
};
use bmp::BmpInfo;
use util::ReadLittleEndian;
pub fn read_rgb(info: &BmpInfo, f: &mut File) -> io::Result<Surface<Pix<i32>>> {
    let pack_len = info.width*3;
    let full_len = match (pack_len) % 4 {
        0 => pack_len,
        r => pack_len-r+4,
    } as u64;

    let mut surface = Surface::new(
        info.width,
        info.height,
        Pix::RGB(0i32, 0i32, 0i32)
    );

    for y in 0..info.height {
        let offset = info.offset + (y as u64)*full_len;
        f.seek(SeekFrom::Start(offset))?;
        for x in 0..info.width {
            surface[(x, y)] = Pix::RGB(
                f.read_u8()? as i32,
                f.read_u8()? as i32,
                f.read_u8()? as i32,
            );
        }
    }

    Ok(surface)
}

pub fn write_rgb(info: &BmpInfo, f: &mut File, surface: &Surface<Pix<i32>>) -> io::Result<()> {
    let pack_len = info.width*3;
    let padding_bytes = match (pack_len) % 4 {
        0 => vec![],
        r => vec![0u8; (4-r) as usize]
    };

    let mut buf = [0u8; 3];
    for y in 0..info.height {
        for x in 0..info.width {
            let p = surface[(x, y)];
            match p {
                Pix::RGB(r, g, b) => {
                    buf[0] = r.clamp(0, 255) as u8;
                    buf[1] = g.clamp(0, 255) as u8;
                    buf[2] = b.clamp(0, 255) as u8;
                },
                Pix::Gray(g) => { 
                    buf[0] = g.clamp(0, 255) as u8;
                    buf[1] = g.clamp(0, 255) as u8;
                    buf[2] = g.clamp(0, 255) as u8;
                },
            }
            f.write(&buf)?;
        }
        f.write(&padding_bytes)?;
    }

    Ok(())
}
