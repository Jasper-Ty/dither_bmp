use std::ops::{ Add, Sub };

use super::util::{ 
    u8_sum_clamp, 
    u8_diff_clamp, 
    u8_scale256,
};
use super::fileio::BMPFile;

enum Pix {
    P8(u8),
    P24(Pixel),
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Pixel {
    pub red: u8,
    pub green: u8,
    pub blue: u8,
}

impl Add for Pixel {
    type Output = Pixel;

    // clips
    fn add (self, other: Pixel) -> Pixel {
        const MAX: u16 = u8::MAX as u16;
        let red = u8_sum_clamp (self.red, other.red);
        let green = u8_sum_clamp (self.green, other.green);
        let blue = u8_sum_clamp (self.blue, other.blue);

        Pixel {
            red,
            green,
            blue,
        }
    }
}

impl Sub for Pixel {
    type Output = Pixel;

    // clips
    fn sub (self, other: Pixel) -> Pixel {
        let red = u8_diff_clamp (self.red, other.red);
        let green = u8_diff_clamp (self.green, other.green);
        let blue = u8_diff_clamp (self.blue, other.blue);
        Pixel {
            red,
            green,
            blue,
        }
    }
}

impl Pixel {
    pub fn scale256 (&mut self, numerator: u8, denominator: u8) {
        self.red = u8_scale256 (self.red, numerator, denominator);
        self.green = u8_scale256 (self.green , numerator, denominator);
        self.blue = u8_scale256 (self.blue , numerator, denominator); }
}

pub struct Surface {
    data: Vec<Pixel>,
    pub width: usize,
    pub height: usize,
}

impl Surface {
    pub fn new(width: usize, height: usize) -> Surface {
        let mut data:Vec<Pixel> = Vec::new();

        for _i in 0..height*width {
            let mut p = Pixel { 
                red: 0, 
                green: 0, 
                blue: 0 
            };
            data.push(p);
        }

        Surface {
            data,
            width,
            height,
        }
    }

    pub fn get (&self, x: usize, y: usize) -> Option<&Pixel> {
        if x >= self.width || y >= self.height {
            None
        } else {
            Some(&(self.data[y * self.width + x]))
        }
    }

    pub fn get_mut (&mut self, x: usize, y: usize) -> Option<&mut Pixel> {
        if x >= self.width || y >= self.height {
            None
        } else {
            Some(&mut(self.data[y * self.width + x]))
        }
    }
}

/*
impl From<&BMPFile> for Surface {
    fn from(bmp_file: &BMPFile) -> Self {
        let width = bmp_file.info_header.width as usize;
        let height = bmp_file.info_header.height as usize;
        let mut data:Vec<Pixel> = Vec::new();

        let mut iter = bmp_file.image_data.iter();
        let row_padding =  calc_row_padding (width);

        println!("width: {}, height: {}", width, height);
        println!("row_padding: {}", row_padding);

        for x in 0..height {
            for y in 0..width {
                let red = iter.next().unwrap();
                let green = iter.next().unwrap();
                let blue = iter.next().unwrap();
                let mut p = Pixel {
                    red: *red,
                    green: *green,
                    blue: *blue,
                };
                data.push(p);
            }
            for i in 0..row_padding {
                let discard = iter.next().unwrap();
            }
        }

        println!("data.len() : {}", data.len());

        Surface {
            data, 
            width,
            height,
        }
    }
}
*/

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn create_surface() {
        let surface = Surface::new(100, 100);
    }

    /*
    #[test]
    fn from_bmp_file() {
        let mut bmp_file = BMPFile::read("bmp_examples/greenland_grid_velo.bmp")
            .expect ("Should be able to open bmp."); 
        let surface = Surface::from(&bmp_file);
    }
    */

    #[test]
    fn pixel_add_1 () {
        let a = Pixel { red: 34, green: 92, blue: 17 };
        let b = Pixel { red: 58, green: 12, blue: 148 };

        let c = a + b;
        assert_eq!(c, Pixel { red:34+58, green:92+12, blue:17+148 });
    }

    #[test]
    fn pixel_add_2 () {
    }

    #[test]
    fn pixel_add_3 () {
    }

    #[test]
    fn pixel_add_clip_max() {
    }

    #[test]
    fn pixel_add_clip_min() {
    }
}
