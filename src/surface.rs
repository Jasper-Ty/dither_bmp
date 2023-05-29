use std::ops::{ Index, IndexMut };


pub struct Surface<T> {
    data: Vec<T>,
    width: u32,
    height: u32,
}
impl<T> Surface<T> {
    /// Create a new Surface
    /// 
    /// # Panics
    ///
    /// The `new` function will panic if the length of `data` is not `width*height`
    pub fn new(width: u32, height: u32, data: Vec<T>) -> Surface<T>{
        let len = (width*height) as usize;
        if data.len() != len {
            panic!("Bad data passed in to surface");
        }
        Surface {
            data,
            width,
            height,
        }
    }
    pub fn width(&self) -> u32 {
        self.width
    }
    pub fn height(&self) -> u32 {
        self.height
    }
    fn idx(&self, x: i32, y: i32) -> Option<usize>{
        if x < 0 || y < 0 {
            None
        } else {
            let (x, y) = (x as u32, y as u32);
            let idx = (y*(self.width)+x) as usize;
            Some(idx)
        }
    }
    pub fn get(&self, x: i32, y: i32) -> Option<&T> {
        let idx = self.idx(x, y)?;
        self.data.get(idx)
    }
    pub fn get_mut(&mut self, x: i32, y: i32) -> Option<&mut T> {
        let idx = self.idx(x, y)?;
        self.data.get_mut(idx)
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
