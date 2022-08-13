use crate::prelude::*;
use rayon::{slice::ParallelSliceMut, iter::{IndexedParallelIterator, ParallelIterator}};

pub struct Context {
    pub textures                : Vec<Object>,
    pub root                    : Node,
}

impl Context {

    pub fn new() -> Self {
        Self {
            textures            : vec![],
            root                : Node::new(),
        }
    }

    pub fn update(&mut self) {
        match &mut self.root.object {
            Object::Empty => {},
            Object::AnalyticalObject(object) => {
                object.update();
            },
            _ => {},
        }
    }

    pub fn render_distributed(&mut self, camera: &Box<dyn Camera3D>, color: &mut ByteBuffer, _depth: &mut Buffer<f32>) {
        let [width, height] = color.size;

        self.update();

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

                    let c = self.get_color(&ray,&[x as usize, y as usize], &color.size, &self.root);

                    pixel.copy_from_slice(&c);
                }
            });
    }

    pub fn render(&mut self, camera: &Box<dyn Camera3D>, color: &mut ByteBuffer, _depth: &mut Buffer<f32>) {

        self.update();

        let [width, height] = color.size;
        let ratio = width as F / height as F;

        for y in 0..height {
            for x in 0..width {
                let i = y * 4 * width + x * 4;
                let xx = x as F / width as F;
                let yy = y as F / height as F;
                let coord = Vector2::new((xx - 0.5) * ratio, yy - 0.5);

                let ray = camera.gen_ray(coord);

                let c = self.get_color(&ray,&[x, y], &color.size, &self.root);

                color.pixels[i..i + 4].copy_from_slice(&c);
            }
        }
    }

    #[inline(always)]
    fn get_color(&self, ray: &[Vector3<F>; 2], p: &[usize; 2], size: &[usize;2], node: &Node) -> Color {
        let mut c = [0, 0, 0, 255];

            match &node.object {
                Object::Empty => {},
                Object::AnalyticalObject(object) => {
                    if let Some(hit) = object.get_distance_normal_uv_face(&ray) {

                        let tex_index= 0_usize;
                        match &self.textures[tex_index] {
                            Object::Element2D(el) => {
                                let uv = hit.2;
                                c = el.get_color_at(&[uv.x, uv.y]);
                            },
                            _ => {},
                        }
                    }
                },
                Object::Element2D(element) => {
                    let [width, height]= size;
                    let [x, y]= p;

                    let xx = (*x as F / *width as F) * 2.0 - 1.0;
                    let yy = (*y as F / *height as F) * 2.0 - 1.0;
                    c = element.get_color_at(&[xx, yy]);
                }
            }
        c
    }

}