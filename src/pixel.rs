use std::ops::{ Add, Sub };

pub enum Pix<T: Sized>{
    RGB(T, T, T),
    Gray(T),
}

impl Add for Pix<u8>{
    type Output = Self;

    fn add (self, other: Self) -> Self {
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
