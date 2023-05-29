use std::{
    ops::{ 
        Add, 
        Sub, 
        Mul, 
        Div, 
        AddAssign 
    }
};

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct RGB<T: Copy>(pub T, pub T, pub T);

impl<T> Add for RGB<T>
where
    T: Copy + Add<Output=T>
{
    type Output = Self;

    fn add(self, rhs: Self) -> Self {
        let RGB(rs, gs, bs) = self;
        let RGB(ro, go, bo) = rhs;
        RGB(rs+ro, gs+go, bs+bo)
    }
}
impl<T> Sub for RGB<T>
where
    T: Copy + Sub<Output=T>
{
    type Output = Self;

    fn sub(self, rhs: Self) -> Self {
        let RGB(rs, gs, bs) = self;
        let RGB(ro, go, bo) = rhs;
        RGB(rs-ro, gs-go, bs-bo)
    }
}
impl<T> Mul<T> for RGB<T> 
where
    T: Copy + Mul<Output=T>
{
    type Output = Self;
    fn mul(self, rhs: T) -> Self {
        let RGB(r, g, b) = self;
        RGB(r*rhs, g*rhs, b*rhs) 
    }
}
impl<T> Div<T> for RGB<T> 
where
    T: Copy + Div<Output=T>
{
    type Output = Self;
    fn div(self, rhs: T) -> Self {
        let RGB(r, g, b) = self;
        RGB(r/rhs, g/rhs, b/rhs) 
    }
}
impl<T> AddAssign for RGB<T> 
where
    T: Copy + Add<Output=T>
{
    fn add_assign(&mut self, other: Self) {
        *self = Self(self.0+other.0, self.1+other.1, self.2+other.2);
    }
}
