
use crate::prelude::*;
use rayon::{*, slice::ParallelSliceMut, iter::{IndexedParallelIterator, ParallelIterator}};

pub struct RasterCPU {

}

impl Rasterizer for RasterCPU {

    fn new() -> Self {

        Self {

        }
    }

    fn render(&self, camera: &Box<dyn Camera3D>, color: &mut ByteBuffer, depth: &mut Buffer<f32>) {

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
                    let c = self.get_pixel(&ray);

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

    #[inline(always)]
    fn get_pixel(&self, ray: &[Vector3<F>; 2]) -> Color {

        let [ro, rd] = ray;
        let mut color = [255, 0, 0, 255];

        let sph = Vector4::new(0.0, 0.0, 0.0, 1.0);

        let oc = ro - sph.xyz();
        let b = oc.dot(rd);
        let c = oc.dot(&oc) - sph.w * sph.w;
        let h = b*b - c;
        //if h<0.0 return -1.0;
        if h >= 0.0 {
            //let d = -b - h.sqrt();
            color[1] = 255;
        }
        /*
        let mut t = 0.0001;

        for _d in 0..10 {

            let p = ro + t * rd;

            let dist = p.norm() - 1.0;

            if dist < 0.0001 {
                c[1] = 255;
                break;
            }

            t += dist;
        }*/

        color
    }

}