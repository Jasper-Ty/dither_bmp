use crate::surface::{ RGB, SurfaceRGB };

pub fn find_nearest(p: &RGB, n: u8) -> (RGB, RGB) {
    let err_red = p.red % n;
    let err_green = p.green % n;
    let err_blue = p.blue % n;

    let red = n * (p.red / n);
    let green = n * (p.green / n);
    let blue = n * (p.blue / n);

    let nearest = RGB {
        red,
        green,
        blue
    };
    let err = RGB {
        red: err_red,
        green: err_green,
        blue: err_blue,
    };

    (nearest, err)
}

pub fn dither_rgb(surface: &mut SurfaceRGB, n: u8) {
    for y in 0..surface.height {
        for x in 0..surface.width {
            let p = surface.get_mut(x, y).unwrap();
            let (nearest, _) = find_nearest(p, n);
            let idx = y*surface.width + x;
            surface.data[idx as usize] = nearest;
        }
    }
}

