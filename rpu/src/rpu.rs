use nalgebra::*;

pub mod buffer;
pub mod camera;
pub mod world;
pub mod analytical;

use crate::prelude::*;

pub struct RPU {
    color               : ByteBuffer,
    depth               : Buffer<f32>,

    camera              : Box<dyn Camera3D>,
    world               : World,
}

impl RPU {

    pub fn new(width: usize, height: usize) -> Self {

        Self {
            color       : ByteBuffer::new(width, height, 0),
            depth       : Buffer::new(width, height, -1.0),

            camera      : Box::new(Pinhole::new()),
            world       : World::new(),
        }
    }

    pub fn render(&mut self, frame: &mut [u8], rect: (usize, usize, usize, usize)) {

        self.world.render_distributed(&mut &self.camera, &mut self.color, &mut self.depth);
        // self.world.render(&mut &self.camera, &mut self.color, &mut self.depth);

        self.copy_slice(frame, &self.color.pixels[..], &rect, self.color.size[0] as usize * 4);
    }

    /// Copies rect from the source frame into the dest frame
    fn copy_slice(&self, dest: &mut [u8], source: &[u8], rect: &(usize, usize, usize, usize), dest_stride: usize) {
        for y in 0..rect.3 {
            let d = rect.0 * 4 + (y + rect.1) * dest_stride;
            let s = y * rect.2 * 4;
            dest[d..d + rect.2 * 4].copy_from_slice(&source[s..s + rect.2 * 4]);
        }
    }
}