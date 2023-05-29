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


pub trait Quantize {
    fn quantize(&self, q: &QuantizationLevel) -> Self;
}
impl Quantize for u8 {
    fn quantize(&self, q: &QuantizationLevel) -> Self {
        let x = *self;
        let n = q.num_values();
        let width = 255 / (n-1);
        let idx = if x%width <= width/2 {
            x / width
        } else {
            x / width + 1
        };
        width * idx
    }
}
impl Quantize for i32 {
    fn quantize(&self, q: &QuantizationLevel) -> Self {
        let x = (*self).clamp(0, 255) as u8;
        x.quantize(q) as i32
    }
}

use crate::pixel::RGB;
impl Quantize for RGB<i32> {
    fn quantize(&self, q: &QuantizationLevel) -> Self {
        let RGB(r, g, b) = self;
        RGB(r.quantize(q), g.quantize(q), b.quantize(q))
    }
}
