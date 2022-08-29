
pub mod buffer;
pub mod script;
pub mod camera;
pub mod analytical;
pub mod compiler;
pub mod hit;
pub mod element2d;
pub mod layout3d;
pub mod sdf3d;
pub mod renderer;

use crate::prelude::*;

pub struct RPU {

    context             : Option<Context>,

    color               : ColorBuffer<F>,
}

impl RPU {

    pub fn new(width: usize, height: usize) -> Self {

        Self {

            context     : None,

            color       : ColorBuffer::new(width, height, 0.0),
        }
    }

    pub fn compile_from_path(&mut self, path_to_main: PathBuf) -> Result<(), RPUError> {

        let mut compiler = Compiler::new();

        if let Some(main) = std::fs::read_to_string(path_to_main).ok() {
            let rc = compiler.compile(main);

            if rc.is_err() {
                return Err(rc.err().unwrap());
            }
            self.context = rc.ok();

            // println!("{}", main_code);
        }
        Ok(())
    }

    pub fn compile(&mut self, main_code: String) -> Result<(), RPUError> {

        let mut compiler = Compiler::new();

        let rc = compiler.compile(main_code);
        if rc.is_err() {
            return Err(rc.err().unwrap());
        }
        self.context = rc.ok();
        Ok(())
    }

    pub fn render(&mut self, frame: &mut [u8], rect: (usize, usize, usize, usize), stride: usize) {

        if rect.2 != self.color.size[0] || rect.3 != self.color.size[1] {
            self.color = ColorBuffer::new(rect.2, rect.3, 0.0);
        }

        if let Some(context) = &mut self.context {
            context.render_distributed(&mut self.color);
        }

        self.copy_slice_float_to_u8(frame, &self.color.pixels[..], &rect, stride);
    }

    /// Copies rect from the source frame into the dest frame
    fn _copy_slice(&self, dest: &mut [u8], source: &[u8], rect: &(usize, usize, usize, usize), dest_stride: usize) {
        for y in 0..rect.3 {
            let d = rect.0 * 4 + (y + rect.1) * dest_stride;
            let s = y * rect.2 * 4;
            dest[d..d + rect.2 * 4].copy_from_slice(&source[s..s + rect.2 * 4]);
        }
    }

    /// Copies rect from the source float frame into the dest frame
    fn copy_slice_float_to_u8(&self, dest: &mut [u8], source: &[F], rect: &(usize, usize, usize, usize), dest_stride: usize) {
        for y in 0..rect.3 {
            for x in 0..rect.2 {
                let d = (rect.0 + x) * 4 + (y + rect.1) * dest_stride * 4;
                let s = x * 4 + y * rect.2 * 4;
                let c = [(source[s] * 255.0) as u8, (source[s+1] * 255.0) as u8,  (source[s+2] * 255.0) as u8,  (source[s+3] * 255.0) as u8];
                dest[d..d + 4].copy_from_slice(&c);
            }
        }
    }

}