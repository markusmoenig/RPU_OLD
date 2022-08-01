use crate::prelude::*;
use rayon::{slice::ParallelSliceMut, iter::{IndexedParallelIterator, ParallelIterator}};

pub struct World {

    pub analytical_objects      : Vec<Box<dyn Analytical>>,
}

impl World {

    pub fn new() -> Self {

        let mut analytical_objects : Vec<Box<dyn Analytical>> = vec![];

        let cube = Box::new(AnalyticalCube::new());
        // let sphere = Box::new(AnalyticalSphere::new());
        analytical_objects.push(cube);
        // analytical_objects.push(sphere);

        Self {
            analytical_objects,
        }
    }

    pub fn update(&mut self) {
        self.analytical_objects[0].get_rotation().y += 1.0;
        self.analytical_objects[0].update();
    }

    pub fn render_distributed(&self, camera: &Box<dyn Camera3D>, color: &mut ByteBuffer, _depth: &mut Buffer<f32>) {

        let [width, height] = color.size;

        const LINES: usize = 20;
        let ratio = width as F / height as F;

        color.pixels
            .par_rchunks_exact_mut(width * LINES * 4)
            .enumerate()
            .for_each(|(j, line)| {
                for (i, pixel) in line.chunks_exact_mut(4).enumerate() {
                    let i = (LINES - j - 1) * width * LINES + i;
                    let x = (i % width) as F;
                    let y = (i / width) as F;

                    let xx = x as F / width as F;
                    let yy = y as F / height as F;

                    let coord = Vector2::new((xx - 0.5) * ratio, yy - 0.5);

                    let ray = camera.gen_ray(coord);
                    let c = self.get_color(&ray);

                    pixel.copy_from_slice(&c);
                }
            });

        /*
        for y in 0..height {
            for x in 0..width {
                let i = y * 4 * width + x * 4;
                let xx = x as F / width as F;
                let yy = y as F / height as F;
                let ratio = width as F / height as F;
                let coord = Vector2::new((xx - 0.5) * ratio, yy - 0.5);

                let ray = camera.gen_ray(coord);

                let c = self.get_pixel(&ray);

                color.pixels[i..i + 4].copy_from_slice(&c);
            }
        }*/
    }

    pub fn render(&self, camera: &Box<dyn Camera3D>, color: &mut ByteBuffer, _depth: &mut Buffer<f32>) {

        let [width, height] = color.size;
        let ratio = width as F / height as F;

        for y in 0..height {
            for x in 0..width {
                let i = y * 4 * width + x * 4;
                let xx = x as F / width as F;
                let yy = y as F / height as F;
                let coord = Vector2::new((xx - 0.5) * ratio, yy - 0.5);

                let ray = camera.gen_ray(coord);

                let c = self.get_color(&ray);

                color.pixels[i..i + 4].copy_from_slice(&c);
            }
        }
    }

    #[inline(always)]
    fn get_color(&self, ray: &[Vector3<F>; 2]) -> Color {
        let mut c = [0, 0, 0, 255];

        for a in &self.analytical_objects {
            if let Some(_dn) = a.get_distance_and_normal(ray) {
                c[0] = 255;
            }
        }
        c
    }
}