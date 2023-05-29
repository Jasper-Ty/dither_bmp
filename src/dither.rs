use std::ops::{ 
    Sub, 
    Mul, 
    Div, 
    AddAssign 
};
use crate::{
    surface::Surface,
    quantize::{ 
        QuantizationLevel,
        Quantize,
    }
};

pub fn dither<T>(surface: &mut Surface<T>, q: &QuantizationLevel)
where
    T: 
        Copy + 
        AddAssign + 
        Sub<Output=T> + 
        Mul<i32, Output=T> + 
        Div<i32, Output=T> + 
        Quantize,
{
    let width = surface.width() as i32;
    let height = surface.height() as i32;

    for y in 0..height {
        for x in 0..width {
            let pixel = *surface.get(x,y).unwrap();
            let quantized = pixel.quantize(q);
            let error = pixel - quantized;

            *surface.get_mut(x, y).unwrap() = quantized;

            surface.get_mut(x+1,y  ).map(|p| { *p += (error * 7)/16; });
            surface.get_mut(x-1,y+1).map(|p| { *p += (error * 3)/16; });
            surface.get_mut(x  ,y+1).map(|p| { *p += (error * 5)/16; });
            surface.get_mut(x+1,y+1).map(|p| { *p += (error * 1)/16; });
        }
    }
}
