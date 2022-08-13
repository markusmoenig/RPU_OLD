
pub mod buffer;
pub mod script;
pub mod camera;
pub mod analytical;
pub mod compiler;
pub mod hit;
pub mod element2d;

use crate::prelude::*;

pub struct RPU {

    context             : Option<Context>,

    color               : ByteBuffer,
    depth               : Buffer<f32>,

    camera              : Box<dyn Camera3D>,
}

impl RPU {

    pub fn new(width: usize, height: usize) -> Self {

        Self {

            context     : None,

            color       : ByteBuffer::new(width, height, 0),
            depth       : Buffer::new(width, height, -1.0),

            camera      : Box::new(Pinhole::new()),
        }
    }

    pub fn compile_from_path(&mut self, path_to_main: PathBuf) -> Result<(), Error> {

        let mut compiler = Compiler::new();
        let rc = compiler.compile_from_path(path_to_main);

        if rc.is_err() {
            return Err(rc.err().unwrap());
        }
        self.context = rc.ok();
        Ok(())
    }

    pub fn render(&mut self, frame: &mut [u8], rect: (usize, usize, usize, usize)) {

        if let Some(context) = &mut self.context {
            context.render_distributed(&mut &self.camera, &mut self.color, &mut self.depth);
        }

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