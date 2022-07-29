
use crate::prelude::*;
use nalgebra::*;

pub struct RasterFast {

}

impl Rasterizer for RasterFast {

    fn new() -> Self {

        Self {

        }
    }

    fn render(&self, camera: &Box<dyn Camera3D>, color: &mut Buffer<u32>, depth: &mut Buffer<f32>) {

        let [width, height] = color.size;

        for y in 0..height {
            for x in 0..width {
                let xx = x as f64 / width as f64;
                let yy = y as f64 / height as f64;
                let ratio = width as f64 / height as f64;
                let coord = Vector2::new((xx - 0.5) * ratio, yy - 0.5);

                let ray = camera.gen_ray(coord);

                let c = self.get_pixel(&ray);

                unsafe {
                    color.set([x, y], crate::rpu::buffer::c_to_u32(&c));
                }
                //let c = self.compute(coord);
                //frame[o..o + 4].copy_from_slice(&c);
            }
        }
    }

    fn get_pixel(&self, ray: &[Vector3<F>; 2]) -> Color {

        let b = [0, 255, 0, 255];
        b
    }

}