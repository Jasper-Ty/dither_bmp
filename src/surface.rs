use std::convert::From;
use std::io::{
    self,
    Read,
    Write,
};
use std::ops::{ Index, IndexMut };


pub struct Surface<T> {
    pub data: Vec<T>,
    pub width: u32,
    pub height: u32,
}
impl<T> Index<(u32, u32)> for Surface<T> {
    type Output = T;

    fn index(&self, pair: (u32, u32)) -> &Self::Output {
        let x = pair.0;
        let y = pair.1;
        let idx = ( y * self.width ) + x;
        &(self.data[idx as usize])
    }
}
impl<T> IndexMut<(u32, u32)> for Surface<T> {
    fn index_mut(&mut self, pair: (u32, u32)) -> &mut T {
        let x = pair.0;
        let y = pair.1;
        let idx = ( y * self.width ) + x;
        &mut(self.data[idx as usize])
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn indexing() {
        let data: Vec<u8> = vec![1, 2, 3 ,4];
        let width = 2;
        let height = 2;

        let surface = Surface {
            data,
            width,
            height,
        };

        assert_eq!(1, surface[(0,0)]);
        assert_eq!(2, surface[(1,0)]);
        assert_eq!(3, surface[(0,1)]);
        assert_eq!(4, surface[(1,1)]);
    }

    #[test]
    fn assigning() {
        let data: Vec<u8> = vec![1, 2, 3 ,4];
        let width = 2;
        let height = 2;

        let mut surface = Surface {
            data,
            width,
            height,
        };

        surface[(0,0)] = 5;
        surface[(1,0)] = 6;
        surface[(0,1)] = 7;
        surface[(1,1)] = 8;

        assert_eq!(5, surface[(0,0)]);
        assert_eq!(6, surface[(1,0)]);
        assert_eq!(7, surface[(0,1)]);
        assert_eq!(8, surface[(1,1)]);
    }
}
