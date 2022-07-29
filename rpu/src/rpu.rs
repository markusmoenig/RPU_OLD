use nalgebra::*;

pub mod buffer;
pub mod rasterizer;
pub mod camera;

use crate::prelude::*;

pub struct RPU {

    color               : Buffer<u32>,
    depth               : Buffer<f32>,

    camera              : Box<dyn Camera3D>,
    rasterizer          : Box<dyn Rasterizer>,
}

impl RPU {

    pub fn new(width: usize, height: usize) -> Self {

        Self {

            color       : Buffer::new(width, height, 0),
            depth       : Buffer::new(width, height, -1.0),

            camera      : Box::new(Pinhole::new()),
            rasterizer  : Box::new(RasterFast::new()),
        }
    }

    pub fn render(&mut self, frame: &mut [u8], rect: (usize, usize, usize, usize)) {

        self.rasterizer.render(&mut &self.camera, &mut self.color, &mut self.depth);

        let pixels = bytemuck::cast_slice::<u32, u8>(&self.color.pixels[..]);

        self.copy_slice(frame, pixels, &rect, self.color.size[0] as usize * 4);

        /*
        let width = rect.2;
        let height = rect.3;
        for y in 0..height {
            for x in 0..width {
                let o = x * 4 + y * width * 4;

                let xx = x as f64 / width as f64;
                let yy = y as f64 / height as f64;
                let ratio = width as f64 / height as f64;
                let coord = Vector2::new((xx - 0.5) * ratio, yy - 0.5);

                let c = self.compute(coord);
                frame[o..o + 4].copy_from_slice(&c);
            }
        }*/
    }

    pub fn compute(&mut self, p: Vector2<f64>) -> [u8; 4] {
        let mut c = [0, 0, 0, 255];

        let ro = Vector3::new(0.0, 0.0, 4.0);
        let ta = Vector3::new(0.0, 0.0, 0.0);

        let rd = self.camera(p, ro, ta);

        let mut t = 0.0001;

        for _d in 0..100 {

            let p = ro + t * rd;

            let d = p.norm() - 1.0;

            if d < 0.001 {
                c[0] = 255;
                break;
            }

            t += d;
        }

        c
    }

    pub fn camera(&self, p: Vector2<f64>, ro: Vector3<f64>, ta: Vector3<f64>) -> Vector3<f64> {

        let ww = (ta - ro).normalize();
        let uu = ww.cross(&Vector3::new(0.0, 1.0, 0.0)).normalize();
        let vv = uu.cross(&ww).normalize();

        let rd = (p.x * uu + p.y * vv + 2.0 * ww).normalize();

        rd
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