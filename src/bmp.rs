use std::fs::File;
use std::io::{ 
    self,
    Seek,
    SeekFrom,

    Read,

    Error, 
    ErrorKind 
};

use super::util::ReadLittleEndian;

pub fn check_sig(f: &mut File) -> io::Result<()> {
    let mut buf = [0; 2];

    f.read(&mut buf)?;
    match buf{
        [66, 77] => Ok(()),
        _ => Err(Error::new(ErrorKind::Other, "Bad BMP signature!")),
    }
}

pub fn check_compression(f: &mut File) -> io::Result<()> {
    let mut buf = [0; 4];

    f.seek(SeekFrom::Start(30))?;
    f.read(&mut buf)?;
    match buf {
        [0, 0, 0, 0] => Ok(()),
        _ => Err(Error::new(ErrorKind::Other, "Weird BMP file!")),
    }
}


#[derive(Debug)]
pub struct BmpInfo {
    pub offset: u64,
    pub width: u32,
    pub height: u32,
    pub bits_per_pixel: u16,
}
impl BmpInfo {
    pub fn from_file(f: &mut File) -> io::Result<BmpInfo> {
        f.seek(SeekFrom::Start(10))?;
        let offset = f.read_u32()? as u64;
        f.seek(SeekFrom::Start(18))?;
        let width = f.read_u32()?;
        f.seek(SeekFrom::Start(22))?;
        let height = f.read_u32()?;
        f.seek(SeekFrom::Start(28))?;
        let bits_per_pixel = f.read_u16()?; 
        Ok(BmpInfo {
            offset,
            width,
            height,
            bits_per_pixel,
        })
    }
}

use crate::surface::{Surface, RGB};
pub fn read_image(info: &BmpInfo, f: &mut File) -> io::Result<Surface<RGB<i32>>> {
    let pack_len = info.width*3;
    let full_len = match (pack_len) % 4 {
        0 => pack_len,
        r => pack_len-r+4,
    } as u64;

    let mut surface = Surface::new(
        info.width,
        info.height,
        RGB { red: 0, green: 0, blue: 0 },
    );

    for y in 0..info.height {
        let offset = info.offset + (y as u64)*full_len;
        f.seek(SeekFrom::Start(offset))?;
        for x in 0..info.width {
            surface[(x, y)] = RGB {
                red: f.read_u8()? as i32,
                green: f.read_u8()? as i32,
                blue: f.read_u8()? as i32,
            };
        }
    }

    Ok(surface)
}
