use std::{
    fs::File,
    io::{
        Seek,
        SeekFrom,
        Read,
        Error, 
        ErrorKind,
        Result,
    }
};

use super::util::ReadLittleEndian;

pub fn check_sig(f: &mut File) -> Result<()> {
    let mut buf = [0; 2];

    f.read(&mut buf)?;
    match buf{
        [66, 77] => Ok(()),
        _ => Err(Error::new(ErrorKind::Other, "Bad BMP signature!")),
    }
}

pub fn check_compression(f: &mut File) -> Result<()> {
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
    pub fn from_file(f: &mut File) -> Result<BmpInfo> {
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

