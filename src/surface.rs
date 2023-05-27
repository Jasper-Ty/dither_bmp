use std::ops::{ Index, IndexMut };

pub struct Surface<T> {
    data: Vec<T>,
    width: u32,
    height: u32,
}
impl<T> Surface<T> {
    pub fn get(&self, x: u32, y: u32) -> Option<&T> {
        let idx = y*(self.width)+x;
        self.data.get(idx as usize)
    }
    pub fn get_mut(&mut self, x: u32, y: u32) -> Option<&mut T> {
        let idx = y*(self.width)+x;
        self.data.get_mut(idx as usize)
    }
    pub fn iter(&self) -> SurfaceIterator<T> {
        SurfaceIterator {
            surface: &self,
            x: 0,
            y: 0,
        }
    }
}

pub struct SurfaceIterator<'a, T> {
    surface: &'a Surface<T>,
    x: u32,
    y: u32,
}
impl<'a, T> Iterator for SurfaceIterator<'a, T> {
    type Item = (u32, u32, &'a T);

    fn next(&mut self) -> Option<Self::Item> {
        let x = self.x;
        let y = self.y;
        let width = self.surface.width;
        
        let idx = y * (width) + x;
        let item = self.surface.data.get(idx as usize);
        
        if x == width - 1 { 
            self.x = 0;
            self.y += 1;
        } else {
            self.x += 1;
        }

        match item {
            Some(t) => Some((x, y, t)),
            None => None
        }
    }
}

impl<T: Clone> Surface<T> {
    pub fn new(width: u32, height: u32, fill: T) -> Surface<T>{
        let mut data: Vec<T> = Vec::with_capacity((width*height) as usize);
        for _ in 0..width*height {
            data.push(fill.clone());
        }
        Surface {
            data,
            width,
            height
        }
    }
    pub fn width(&self) -> u32 {
        self.width
    }
    pub fn height(&self) -> u32 {
        self.height
    }
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
    fn new() {
        let surface = Surface::new(10, 10, 0);
    }

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

    #[test]
    fn iterator() {
        let data: Vec<u8> = vec![1, 2, 3 ,4];
        let width = 2;
        let height = 2;

        let mut surface = Surface {
            data,
            width,
            height,
        };

        let mut iter = surface.iter();

        assert_eq!(Some((0, 0, &1)), iter.next());
        assert_eq!(Some((1, 0, &2)), iter.next());
        assert_eq!(Some((0, 1, &3)), iter.next());
        assert_eq!(Some((1, 1, &4)), iter.next());
        assert_eq!(None, iter.next());
    }
}
