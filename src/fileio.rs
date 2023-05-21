use std::fs::File;
use std::io::{ Read, Error };

use crate::util::{ read_u16, read_u32 };

#[derive(Debug)]
pub struct FileHeader {
    pub signature: u16,
    pub file_size: u32,
    pub reserved: u32,
    pub offset: u32,
}
const FILEHEADER_SIZE: usize = 14;

#[derive(Debug)]
pub struct InfoHeader {
    pub size: u32,
    pub width: u32,
    pub height: u32,
    pub planes: u16,
    pub bits_per_pixel: u16,
    pub compression: u32,
    pub image_size: u32,
    pub h_resolution: u32,
    pub v_resolution: u32,
    pub num_colors: u32,
    pub important_colors: u32,
}
const INFOHEADER_SIZE: usize = 40;

pub struct BMPFile {
    pub file_header: FileHeader,
    pub info_header: InfoHeader,
    pub extra: Vec<u8>,
    pub image_data: Vec<u8>,
}

impl BMPFile {
    pub fn build (filepath: &str) -> Result<BMPFile, Error> {
    /* Open file ----------------------------------------------------------- */
        let mut file = File::open(filepath)?;

    /* Read fixed-length segments ------------------------------------------ */
        let file_header = BMPFile::read_fileheader(&mut file)?;
        let info_header = BMPFile::read_infoheader(&mut file)?;

    /* Read variable-length segments --------------------------------------- */
        let offset = file_header.offset as usize;
        let mut extra: Vec<u8> = BMPFile::read_extra(&mut file, offset)?;
        let mut image_data = BMPFile::read_image_data(&mut file, offset)?;

    /* Done ---------------------------------------------------------------- */
        Ok(BMPFile {
            file_header,
            info_header,
            extra,
            image_data
        })
    }

    fn read_fileheader (file: &mut File) -> Result<FileHeader, Error> {
    /* Initialize buffer and read file into it ----------------------------- */
        let mut buffer = [0; FILEHEADER_SIZE];
        let n = file.read(&mut buffer)?;
        if n != FILEHEADER_SIZE {
            panic!("Not able to read?");
        }

    /* Read off struct from buffer ----------------------------------------- */
        let signature = read_u16(&buffer[0..2]);
        let file_size = read_u32(&buffer[2..6]); 
        let reserved = read_u32(&buffer[6..10]);
        let offset = read_u32(&buffer[10..14]);

    /* Done ---------------------------------------------------------------- */
        Ok(FileHeader {
            signature,
            file_size,
            reserved,
            offset,
        })
    }

    fn read_infoheader (file: &mut File) -> Result<InfoHeader, Error> {
    /* Initialize buffer and read file into it ----------------------------- */
        let mut buffer = [0; INFOHEADER_SIZE];
        let n = file.read(&mut buffer)?;
        if n != INFOHEADER_SIZE {
            panic!("Not able to read?");
        }

    /* Read off struct from buffer ----------------------------------------- */
        let size = read_u32(&buffer[0..4]);
        let width = read_u32(&buffer[4..8]);
        let height = read_u32(&buffer[8..12]);
        let planes = read_u16(&buffer[12..14]);
        let bits_per_pixel = read_u16(&buffer[14..16]);
        let compression = read_u32(&buffer[16..20]);
        let image_size = read_u32(&buffer[20..24]);
        let h_resolution = read_u32(&buffer[24..28]);
        let v_resolution = read_u32(&buffer[28..32]);
        let num_colors = read_u32(&buffer[32..36]);
        let important_colors = read_u32(&buffer[36..40]);

    /* Done ---------------------------------------------------------------- */
        Ok(InfoHeader {
            size,
            width,
            height,
            planes,
            bits_per_pixel,
            compression,
            image_size,
            h_resolution,
            v_resolution,
            num_colors,
            important_colors
        })
    }

    fn read_extra (file: &mut File, offset: usize) -> Result<Vec<u8>, Error> {
        let size = offset-FILEHEADER_SIZE-INFOHEADER_SIZE;
        if size <= 0 {
            panic!("Offset too close?");
        }
        let mut buf = vec![0; size];
        file.read_exact(&mut buf)?;

        Ok(buf)
    }

    fn read_image_data (file: &mut File, offset: usize) -> Result<Vec<u8>, Error> {
        let mut image_data: Vec<u8> = Vec::new();

        file.read_to_end(&mut image_data)?;

        Ok(image_data)
    }
}

#[cfg(test)]
mod tests {
    use super::*;


    #[test]
    fn build_bmpfile () {
        let bmp_file = BMPFile::build("bmp_examples/lena.bmp")
            .expect("Should be able to open lena.bmp");

        println!("file_header: {:?}", bmp_file.file_header);
        println!("info_header: {:?}", bmp_file.info_header);

    }

    #[test]
    fn read_fileheader() {
        
    }
}
