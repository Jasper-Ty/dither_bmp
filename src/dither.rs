use crate::pixel::Pix;
use crate::surface::Surface;
use crate::quantize::{ 
    QuantizationLevel,
    Quantize,
};

pub fn dither(surface: &mut Surface<Pix<i32>>, q: &QuantizationLevel) {
    let width = surface.width();
    let height = surface.height();

    for y in 0..height {
        for x in 0..width {
            let pixel = surface[(x,y)];
            let quantized = pixel.quantize(q);
            let error = pixel - quantized;

            surface[(x,y)] = quantized;

            if x+1< width {
                surface[(x+1,y)] += (error * 7)/16;
            }
            if x > 0 && y+1 < height {
                surface[(x-1,y+1)] += (error * 3)/16;
            }
            if y+1 < height {
                surface[(x,y+1)] += (error * 5)/16;
            }
            if x+1 < width && y+1 < height {
                surface[(x+1,y+1)] += (error * 1)/16;
            }
        }
    }
}
