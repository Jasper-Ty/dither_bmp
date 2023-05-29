use std::{
    env,
    fs::File,
    io::{
        Read,
        Write,
        Seek,
        SeekFrom,
        Result,
    }
};

use dither_bmp::bmp;
use dither_bmp::quantize::QuantizationLevel::*;
use dither_bmp::{ read_rgb, write_rgb };
use dither_bmp::dither::dither;

fn main() -> Result<()> {
    let args: Vec<String> = env::args().collect();
    if args.len() < 3 {
        println!("USAGE: {} BMP_FILE OUT_FILE QUANTIZATION_LEVEL", &args[0]);
        return Ok(());
    }

    let in_path = &args[1];
    let out_path = &args[2];
    let q = match args.get(3) {
        Some(s) => match s.trim().parse::<i32>() {
            Ok(0) => Q0,
            Ok(1) => Q1,
            Ok(2) => Q2,
            Ok(3) => Q3,
            Ok(4) => Q4,
            Ok(5) => Q5,
            Ok(6) => Q6,
            Ok(7) => Q7,
            _ => Q4,
        }
        None => Q4,
    };
    
    let mut f: File = File::open(in_path)?;
    bmp::check_sig(&mut f)?;
    bmp::check_compression(&mut f)?;
    let info = bmp::BmpInfo::from_file(&mut f)?;

    println!("info: {:?}", info);

    match info.bits_per_pixel {
        24 => { 
            f.seek(SeekFrom::Start(info.offset))?;
            let mut surface = read_rgb(&info, &mut f)?;
            dither(&mut surface, &q);

            let mut pre = vec![0u8; info.offset as usize];
            f.seek(SeekFrom::Start(0))?;
            f.read(&mut pre)?;

            let mut out = File::create(out_path)?;
            out.write(&pre)?;
            out.flush()?;
            write_rgb(&info, &mut out, &surface)?; 
        },
        _ => {}
    };

    Ok(())
}

