use crate::prelude::*;

/// Converts an array of bytes to u32
pub fn c_to_u32(bytes: &Color) -> u32 {
    (bytes[2] as u32) << 16
        | (bytes[1] as u32) << 8
        | (bytes[0] as u32) << 0
        | (bytes[3] as u32) << 24
}

#[derive(Clone)]
pub struct Buffer<T> {
    pub pixels          : Vec<T>,
    pub size            : [usize; 2],
}

impl<T: Clone> Buffer<T> {

    pub fn new (width: usize, height: usize, fill: T) -> Self {

        Self {
            pixels      : vec![fill; width * height],
            size        : [width, height]
        }
    }

    #[inline(always)]
    pub unsafe fn set(&mut self, [x, y]: [usize; 2], pixel: T) {
        let [width, _] = self.size;
        *self.pixels.get_unchecked_mut(y * width + x) = pixel;
    }

    #[inline(always)]
    pub unsafe fn get(&self, [x, y]: [usize; 2]) -> T {
        let [width, _] = self.size;
        self.pixels.get_unchecked(y * width + x).clone()
    }

    pub fn clear(&mut self, fill: T) {
        for p in &mut self.pixels {
            *p = fill.clone();
        }
    }
}

#[derive(Clone)]
pub struct ColorBuffer<T> {
    pub pixels          : Vec<T>,
    pub size            : [usize; 2],
}

impl<T: Clone> ColorBuffer<T> {
    pub fn new (width: usize, height: usize, fill: T) -> Self {

        Self {
            pixels      : vec![fill; width * height * 4],
            size        : [width, height]
        }
    }
}