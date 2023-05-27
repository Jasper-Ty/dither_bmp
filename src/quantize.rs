pub enum QuantizationLevel {
    Q0,
    Q1,
    Q2,
    Q3,
    Q4,
    Q5,
    Q6,
    Q7,
}
use QuantizationLevel::*;
impl QuantizationLevel {
    pub fn num_values(&self) -> u8 {
        match *self {
            Q0 => 255,
            Q1 => 86,
            Q2 => 52,
            Q3 => 18,
            Q4 => 16,
            Q5 => 6,
            Q6 => 4,
            Q7 => 2,
        }
    }
}

use crate::pixel::Pix;
pub fn quantize_pix(p: Pix<u8>, level: &QuantizationLevel) -> Pix<u8> {
    match p {
        Pix::RGB(r, g, b) => Pix::RGB(quantize_u8(r, level), quantize_u8(g, level), quantize_u8(b, level)),
        Pix::Gray(g) => Pix::Gray(quantize_u8(g, level)),
    }

}

fn quantize_u8(x: u8, level: &QuantizationLevel) -> u8 {
    let N = level.num_values();
    let width = 255 / (N-1);
    let idx = if x%width <= width/2 {
        x / width
    } else {
        x / width + 1
    };
    width * idx
}
