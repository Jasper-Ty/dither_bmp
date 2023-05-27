use std::ops::{ Add, Sub, Mul, Div, AddAssign };
use std::convert::From;

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Pix<T>{
    RGB(T, T, T),
    Gray(T),
}

impl<T> Add for Pix<T>
where
    T: Copy+Add<Output=T>
{
    type Output = Self;

    fn add(self, other: Self) -> Self {
        match self {
            Pix::RGB(rs, gs, bs) => match other {
                Pix::RGB(ro, go, bo) => Pix::RGB(rs+ro, gs+go, bs+bo),
                Pix::Gray(o) => Pix::RGB(rs+o, gs+o, bs+o)
            },
            Pix::Gray(s) => match other {
                Pix::RGB(ro, go, bo) => Pix::RGB(s+ro, s+go, s+bo),
                Pix::Gray(o) => Pix::Gray(s+o)
            }
        }
    }
}
impl<T> AddAssign for Pix<T>
where
    T: Copy+Add<Output=T>
{
    fn add_assign(&mut self, other: Self) {
        *self = match self {
            Pix::RGB(rs, gs, bs) => match other {
                Pix::RGB(ro, go, bo) => Pix::RGB(*rs+ro, *gs+go, *bs+bo),
                Pix::Gray(o) => Pix::RGB(*rs+o, *gs+o, *bs+o)
            },
            Pix::Gray(s) => match other {
                Pix::RGB(ro, go, bo) => Pix::RGB(*s+ro, *s+go, *s+bo),
                Pix::Gray(o) => Pix::Gray(*s+o)
            }
        }
    }
}

impl<T> Sub for Pix<T>
where
    T: Copy+Sub<Output=T>
{
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        match self {
            Pix::RGB(rs, gs, bs) => match other {
                Pix::RGB(ro, go, bo) => Pix::RGB(rs-ro, gs-go, bs-bo),
                Pix::Gray(o) => Pix::RGB(rs-o, gs-o, bs-o)
            },
            Pix::Gray(s) => match other {
                Pix::RGB(ro, go, bo) => Pix::RGB(s-ro, s-go, s-bo),
                Pix::Gray(o) => Pix::Gray(s-o)
            }
        }
    }
}
impl Mul<u8> for Pix<i32> {
    type Output = Self;

    fn mul(self, other: u8) -> Self {
        let a = other as i32;
        match self {
            Pix::RGB(r, g, b) => Pix::RGB(r*a, g*a, b*a),
            Pix::Gray(g) => Pix::Gray(g*a),
        }
    }
}
impl Div<u8> for Pix<i32> {
    type Output = Self;

    fn div(self, other: u8) -> Self {
        let d = other as i32;
        match self {
            Pix::RGB(r, g, b) => Pix::RGB(r/d, g/d, b/d),
            Pix::Gray(g) => Pix::Gray(g/d),
        }
    }
}

impl From<Pix<i32>> for Pix<u8> {
    fn from(pixel: Pix<i32>) -> Self {
        match pixel {
            Pix::RGB(r, g, b) => Pix::RGB(
                r.clamp(0,255) as u8, 
                g.clamp(0,255) as u8, 
                b.clamp(0,255) as u8, 
            ),
            Pix::Gray(g) => Pix::Gray(g.clamp(0,255) as u8),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn add_RGB_RGB_u8 () {
        let p: Pix<u8> = Pix::RGB(1, 4, 7);
        let q: Pix<u8> = Pix::RGB(2, 10, 1);
        let r: Pix<u8> = Pix::RGB(3, 14, 8);

        assert_eq!(p+q, r);
    }

    #[test]
    fn add_RGB_RGB_u16 () {
        let p: Pix<u16> = Pix::RGB(
            0xA0C0, 
            0xF0D0, 
            0x7007,
        );
        let q: Pix<u16> = Pix::RGB(
            0x0B0D,
            0x0E0C,
            0x0770,
        );
        let r: Pix<u16> = Pix::RGB(
            0xABCD,
            0xFEDC,
            0x7777,
        );

        assert_eq!(p+q, r);
    }

    #[test]
    fn add_RGB_RGB_u32 () {
        let p: Pix<u32> = Pix::RGB(
            0x80A0C0E0, 
            0x10101010, 
            0x12345678,
        );
        let q: Pix<u32> = Pix::RGB(
            0x090B0D0F,
            0x01010101,
            0x87654321,
        );
        let r: Pix<u32> = Pix::RGB(
            0x89ABCDEF, 
            0x11111111, 
            0x99999999,
        );

        assert_eq!(p+q, r);
    }

    #[test]
    fn add_RGB_RGB_i8() {
        let p: Pix<i8> = Pix::RGB(1, 4, 7);
        let q: Pix<i8> = Pix::RGB(-1, -10, -1);
        let r: Pix<i8> = Pix::RGB(0, -6, 6);

        assert_eq!(p+q, r);
    }

    #[test]
    fn add_RGB_RGB_i16() {
        let p: Pix<i16> = Pix::RGB(2930, 32000, -9218);
        let q: Pix<i16> = Pix::RGB(7912, 10, -1231);
        let r: Pix<i16> = Pix::RGB(10842, 32010, -10449);

        assert_eq!(p+q, r);
    }
}
