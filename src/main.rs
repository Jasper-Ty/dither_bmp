use std::fs::File;
use std::io::{ 
    self, 

    Seek, 
    SeekFrom, 

    Read,
    Write,

    Error, 
    ErrorKind
};
use std::env;

use dither_bmp::bmp;
use dither_bmp::surface::{ Surface, RGB };

fn main() -> io::Result<()> {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        println!("USAGE: {} BMP_FILE", &args[0]);
        return Ok(());
    }

    let path = &args[1];

    let mut f: File = File::open(path)?;
    bmp::check_sig(&mut f)?;
    bmp::check_compression(&mut f)?;
    let info = bmp::BmpInfo::from_file(&mut f)?;

    match info.bits_per_pixel {
        24 => { 
            dither_24(&info, &mut f);
        },
        _ => {}
    };


    Ok(())
}

fn dither_24 (info: &bmp::BmpInfo, f: &mut File) {
    let surface = bmp::read_image(&info, f);
}
