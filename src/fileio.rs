mod file_header;
mod info_header;
mod image_data;

use std::fs::File;
use std::io;

pub use file_header::FileHeader;
pub use info_header::InfoHeader;

#[derive(Debug)]
pub struct BMPFile {
    pub file_header: FileHeader,
    pub info_header: InfoHeader,
    pub color_table: Option<Vec<u8>>,
    pub gap_1: Option<Vec<u8>>,
    pub image_data: Vec<u8>,
    pub gap_2: Option<Vec<u8>>,
}


impl BMPFile {
    fn from_file(file: &mut File) -> io::Result<BMPFile> {
        let file_header = FileHeader::from_read(file)?;
        let info_header = InfoHeader::from_read(file)?;

        let bpp = info_header.bits_per_pixel;
        
        let color_table = 
            if bpp <= 8 {
                let n = 1 << bpp;
                let n = n as usize;
                Some(vec![0; n])
            } else { None };
        let gap_1 = None;

        let image_data = Vec::new();
        let gap_2 = None;

        Ok(BMPFile {
            file_header,
            info_header,
            color_table,
            gap_1,
            image_data,
            gap_2,
        })
    }
}
