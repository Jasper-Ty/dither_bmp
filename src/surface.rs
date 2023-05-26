use std::convert::From;
use std::io::{
    self,
    Read,
    Write,
};

#[derive(Debug)]
pub struct RGB {
    pub red: u8,
    pub green: u8,
    pub blue: u8,
}
impl From<&[u8; 3]> for RGB {
    fn from(buf: &[u8; 3]) -> RGB {
        let red = buf[0];
        let green = buf[1];
        let blue = buf[2];
        RGB {
            red,
            green,
            blue,
        }
    }
}

#[derive(Debug)]
pub struct SurfaceRGB {
    pub data: Vec<RGB>,
    pub width: u32,
    pub height: u32,
}
impl SurfaceRGB {
    pub fn from_read(
        r: &mut impl Read, 
        width: u32, 
        height: u32,
    ) -> io::Result<SurfaceRGB> {
        let mut data = Vec::new();
        let padding = 4 - ( (width * 3) % 4 );
        let mut bin = vec![0; padding as usize];
        let mut buf = [0; 3];

        for _ in 0..height {
            for _ in 0..width {
                r.read(&mut buf)?;
                let p = RGB::from(&buf);
                data.push(p);
            }
            r.read(&mut bin);
        }

        Ok(SurfaceRGB {
            data,
            width,
            height,
        })
    }

    pub fn get(&mut self, x: u32, y: u32) -> Option<&RGB> {
        if x >= self.width || y >= self.height {
            return None;
        }
        let idx = y*self.height + x;
        let idx = idx as usize;

        Some(&self.data[idx])
    }

    pub fn get_mut(&mut self, x: u32, y: u32) -> Option<&mut RGB> {
        if x >= self.width || y >= self.height {
            return None;
        }

        let idx = y*self.height + x;
        let idx = idx as usize;

        Some(&mut self.data[idx])
    }

    pub fn write_to(&self w: impl Write) {
    }
}

struct SurfaceG {
    data: Vec<u8>,
    width: u32,
    height: u32,
}
