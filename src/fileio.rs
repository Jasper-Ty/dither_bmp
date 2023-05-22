use std::fs::File;
use std::io::{ Read, Write };

use crate::little_endian::{ 
    read_u16, 
    read_u32,
    write_u16,
    write_u32,
};

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
    pub fn read (filepath: &str) -> std::io::Result<BMPFile> {
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

    fn read_fileheader (file: &mut File) -> std::io::Result<FileHeader> {
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

    fn read_infoheader (file: &mut File) -> std::io::Result<InfoHeader> {
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

    fn read_extra (file: &mut File, offset: usize) -> std::io::Result<Vec<u8>> {
        let size = offset-FILEHEADER_SIZE-INFOHEADER_SIZE;
        if size < 0 {
            panic!("Offset too close?");
        }
        let mut buf = vec![0; size];
        file.read_exact(&mut buf)?;

        Ok(buf)
    }

    fn read_image_data (file: &mut File, offset: usize) -> std::io::Result<Vec<u8>> {
        let mut image_data: Vec<u8> = Vec::new();

        file.read_to_end(&mut image_data)?;

        Ok(image_data)
    }

    pub fn write (&self, path: &str) -> std::io::Result<()> {
        let mut file = File::create(path)?;

        self.write_file_header(&mut file);
        self.write_info_header(&mut file);
        self.write_extra(&mut file);
        self.write_image_data(&mut file);

        Ok(())
    }

    fn write_file_header (&self, file: &mut File) -> std::io::Result<()> {
        let signature = write_u16 (self.file_header.signature);
        let file_size = write_u32 (self.file_header.file_size);
        let reserved = write_u32 (self.file_header.reserved);
        let offset = write_u32 (self.file_header.offset);
        
        file.write(&signature)?;
        file.write(&file_size)?;
        file.write(&reserved)?;
        file.write(&offset)?;
        
        Ok(())
    }

    fn write_info_header (&self, file: &mut File) -> std::io::Result<()> {
        let size = write_u32 (self.info_header.size);
        let width = write_u32 (self.info_header.width);
        let height = write_u32 (self.info_header.height);
        let planes = write_u16 (self.info_header.planes);
        let bits_per_pixel = write_u16 (self.info_header.bits_per_pixel);
        let compression = write_u32 (self.info_header.compression);
        let image_size = write_u32 (self.info_header.image_size);
        let h_resolution = write_u32 (self.info_header.h_resolution);
        let v_resolution = write_u32 (self.info_header.v_resolution);
        let num_colors = write_u32 (self.info_header.num_colors);
        let important_colors = write_u32 (self.info_header.important_colors);

        file.write(&size)?;
        file.write(&width)?;
        file.write(&height)?;
        file.write(&planes)?;
        file.write(&bits_per_pixel)?;
        file.write(&compression)?;
        file.write(&image_size)?;
        file.write(&h_resolution)?;
        file.write(&v_resolution)?;
        file.write(&num_colors)?;
        file.write(&important_colors)?;

        Ok(())
    }

    fn write_extra (&self, file: &mut File) -> std::io::Result<()> {
        file.write(&(self.extra))?;
        Ok(())
    }

    fn write_image_data (&self, file: &mut File) -> std::io::Result<()> {
        file.write(&(self.image_data))?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;


    #[test]
    fn read_bmp_file() {
        let bmp_file = BMPFile::read("bmp_examples/greenland_grid_velo.bmp")
            .expect("Should be able to open bmp");

        println!("file_header: {:?}", bmp_file.file_header);
        println!("info_header: {:?}", bmp_file.info_header);
    }

    #[test]
    fn dup_bmp_file() {
        let path = "bmp_examples/greenland_grid_velo.bmp";
        let bmp_file = BMPFile::read(path)
            .expect("Should be able to open bmp");

        bmp_file.write("bmp_examples/dup.bmp");
    }

    #[test]
    fn read_fileheader() {
        
    }
}
