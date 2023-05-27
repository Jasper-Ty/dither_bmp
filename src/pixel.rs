use std::ops::{ Add, Sub };

#[derive(Clone, Copy, Debug)]
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
