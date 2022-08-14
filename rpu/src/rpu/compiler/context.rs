use crate::prelude::*;
use rayon::{slice::ParallelSliceMut, iter::{IndexedParallelIterator, ParallelIterator}};

pub struct Context {
    pub textures                : Vec<Object>,
    pub nodes                   : Vec<Node>,

    // pub bvh_nodes               : Vec<BVHNode>,
    // bvh                         : Option<BVH>,
}

impl Context {

    pub fn new() -> Self {
        Self {
            textures            : vec![],
            nodes               : vec![],

            // bvh_nodes           : vec![],
            // bvh                 : None
        }
    }

    pub fn update(&mut self) {
        for o in &mut self.nodes {
            match &mut o.object {
                Object::Empty => {},
                Object::AnalyticalObject(object) => {
                    object.update();
                },
                _ => {},
            }
        }
    }

    pub fn build(&mut self) {
        //self.bvh = Some(BVH::build(&mut self.bvh_nodes));
    }

    pub fn render_distributed(&mut self, camera: &Box<dyn Camera3D>, color: &mut ByteBuffer, _depth: &mut Buffer<f32>) {
        let [width, height] = color.size;

        self.update();

        const LINES: usize = 20;
        let ratio = width as F / height as F;

        // let num_objects = self.bvh_nodes.len();
        // println!("num {}", num_objects);

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

                    let c = [0, 0, 0, 255];

                    /*
                    if num_objects > 1 {
                        if let Some(bvh) = &self.bvh {
                            let [ro, rd] = ray;
                            let r = bvh::ray::Ray::new(bvh::Vector3::new(ro.x, ro.y, ro.z), bvh::Vector3::new(rd.x, rd.y, rd.z));

                            let hit_bvh_nodes = bvh.traverse(&r, &self.bvh_nodes);
                            if hit_bvh_nodes.len() != 1 {
                                println!("note {}", hit_bvh_nodes.len());
                            }
                            let mut hit = false;
                            for n in &hit_bvh_nodes {
                                if let Some(c) = self.get_color(&ray,&[x as usize, y as usize], &color.size, &self.nodes[n.index]) {
                                    pixel.copy_from_slice(&c);
                                    hit = true;
                                    break;
                                }
                            }
                            if hit == false {
                                pixel.copy_from_slice(&c);
                            }
                        }
                    } else {
                        */
                        let mut hit = false;
                        for i in 0..self.nodes.len() {
                            if let Some(c) = self.get_color(&ray,&[x as usize, y as usize], &color.size, &self.nodes[i]) {
                                pixel.copy_from_slice(&c);
                                hit = true;
                                break;
                            }
                        }
                        if hit == false {
                            pixel.copy_from_slice(&c);
                        }
                    // }
                }
            });
    }

    /*
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
    }*/

    #[inline(always)]
    fn get_color(&self, ray: &[Vector3<F>; 2], p: &[usize; 2], size: &[usize;2], node: &Node) -> Option<Color> {
        let mut c = [0, 0, 0, 255];

            match &node.object {
                Object::Empty => {},
                Object::AnalyticalObject(object) => {
                    if let Some(hit) = object.get_distance_normal_uv_face(&ray) {

                        let tex_index= 0_usize;
                        match &self.textures[tex_index] {
                            Object::Element2D(el) => {
                                let uv = hit.uv;
                                c = el.get_color_at(&[uv.x, -uv.y]);
                            },
                            _ => {},
                        }
                    } else {
                        return None;
                    }
                },
                Object::Element2D(element) => {
                    let [width, height]= size;
                    let [x, y]= p;

                    let xx = (*x as F / *width as F) - 0.5;
                    let yy = (*y as F / *height as F) - 0.5;
                    c = element.get_color_at(&[xx, -yy]);
                }
            }
        Some(c)
    }

}