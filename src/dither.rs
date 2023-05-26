use crate::surface::{ RGB, SurfaceRGB };

pub fn find_nearest(p: &RGB, n: u8) -> (RGB, RGB) {
    let err_red = p.red % n;
    let err_green = p.red % n;
    let err_blue = p.red % n;

    let red = p.red - err_red;
    let green = p.red - err_green;
    let blue = p.red - err_blue;

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
            p.red = nearest.red;
            p.green = nearest.green;
            p.blue = nearest.blue;
        }
    }
}

