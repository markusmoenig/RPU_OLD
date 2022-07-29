use nalgebra::*;
use crate::rpuc::{buffer::Buffer2d, rasterizer, Pipeline, Target};
use vek::*;


struct Cube<'a> {
    mvp: Mat4<f32>,
    positions: &'a [Vec4<f32>],
}

impl<'a> Pipeline for Cube<'a> {
    type Vertex = (usize, Rgba<f32>);
    type VsOut = Rgba<f32>;
    type Pixel = u32;

    #[inline(always)]
    fn vert(&self, (v_index, v_color): &Self::Vertex) -> ([f32; 4], Self::VsOut) {
        ((self.mvp * self.positions[*v_index]).into_array(), *v_color)
    }

    #[inline(always)]
    fn frag(&self, v_color: &Self::VsOut) -> Self::Pixel {
        let bytes = v_color.map(|e| (e * 255.0) as u8).into_array();
        (bytes[2] as u32) << 16
            | (bytes[1] as u32) << 8
            | (bytes[0] as u32) << 0
            | (bytes[3] as u32) << 24
    }
}


type Color = [u8; 4];
pub struct RPU {
    color_buffer            : Buffer2d<u32>,
    depth_buffer            : Buffer2d<f32>,
}

impl RPU {

    pub fn new(width: usize, height: usize) -> Self {

        let color_buffer = Buffer2d::new([width, height], 0 as u32);
        let depth_buffer = Buffer2d::new([width, height], 1.0);

        Self {
            color_buffer,
            depth_buffer
        }
    }

    pub fn render(&mut self, frame: &mut [u8], rect: (usize, usize, usize, usize)) {
        let stride = rect.2 * 4;
        //self.set(frame, width / 2, height / 2, stride, &[255, 255, 255, 255]);
        //self.line(frame, 0, 0, 100, 100, stride, &[255, 255, 255, 255]);
        //self.triangle(frame, Vector3::new(0.0, 0.0, 0.0), Vector3::new( 0.0, 100.0, 0.0), Vector3::new( 100.0, 100.0, 0.0), stride, &[255, 255, 255, 255]);

        let mvp = Mat4::perspective_fov_rh_no(1.3, rect.2 as f32, rect.3 as f32, 0.01, 100.0)
            * Mat4::translation_3d(Vec3::new(0.0, 0.0, -2.0))
            * Mat4::<f32>::scaling_3d(0.4);
            // * Mat4::rotation_x((i as f32 * 0.002).sin() * 8.0)
            // * Mat4::rotation_y((i as f32 * 0.004).cos() * 4.0)
            // * Mat4::rotation_z((i as f32 * 0.008).sin() * 2.0);

        self.color_buffer.clear(0);
        self.depth_buffer.clear(1.0);

        Cube {
            mvp,
            positions: &[
                Vec4::new(-1.0, -1.0, -1.0, 1.0), // 0
                Vec4::new(-1.0, -1.0, 1.0, 1.0),  // 1
                Vec4::new(-1.0, 1.0, -1.0, 1.0),  // 2
                Vec4::new(-1.0, 1.0, 1.0, 1.0),   // 3
                Vec4::new(1.0, -1.0, -1.0, 1.0),  // 4
                Vec4::new(1.0, -1.0, 1.0, 1.0),   // 5
                Vec4::new(1.0, 1.0, -1.0, 1.0),   // 6
                Vec4::new(1.0, 1.0, 1.0, 1.0),    // 7
            ],
        }
        .draw::<rasterizer::Triangles<_, rasterizer::BackfaceCullingEnabled>, _>(
            &[
                // -x
                (0, Rgba::green()),
                (3, Rgba::blue()),
                (2, Rgba::red()),
                (0, Rgba::green()),
                (1, Rgba::red()),
                (3, Rgba::blue()),
                // +x
                (7, Rgba::blue()),
                (4, Rgba::green()),
                (6, Rgba::red()),
                (5, Rgba::red()),
                (4, Rgba::green()),
                (7, Rgba::blue()),
                // -y
                (5, Rgba::blue()),
                (0, Rgba::red()),
                (4, Rgba::green()),
                (1, Rgba::green()),
                (0, Rgba::red()),
                (5, Rgba::blue()),
                // +y
                (2, Rgba::red()),
                (7, Rgba::blue()),
                (6, Rgba::green()),
                (2, Rgba::red()),
                (3, Rgba::green()),
                (7, Rgba::blue()),
                // -z
                (0, Rgba::red()),
                (6, Rgba::green()),
                (4, Rgba::blue()),
                (0, Rgba::red()),
                (2, Rgba::blue()),
                (6, Rgba::green()),
                // +z
                (7, Rgba::green()),
                (1, Rgba::red()),
                (5, Rgba::blue()),
                (3, Rgba::blue()),
                (1, Rgba::red()),
                (7, Rgba::green()),
            ],
            &mut self.color_buffer,
            Some(&mut self.depth_buffer),
        );

        let pixels = bytemuck::cast_slice::<u32, u8>(&self.color_buffer.items[..]);

        self.copy_slice(frame, pixels, &rect, stride);

    }


    /// Copies rect from the source frame into the dest frame
    fn copy_slice(&self, dest: &mut [u8], source: &[u8], rect: &(usize, usize, usize, usize), dest_stride: usize) {
        for y in 0..rect.3 {
            let d = rect.0 * 4 + (y + rect.1) * dest_stride;
            let s = y * rect.2 * 4;
            dest[d..d + rect.2 * 4].copy_from_slice(&source[s..s + rect.2 * 4]);
        }
    }

    pub fn barycentric(&self, a: &Vector3<f32>, b: &Vector3<f32>, c: &Vector3<f32>, p: (f32, f32)) -> Vector3<f32> {
        let cross = Vector3::new( c.x - a.x, b.x - a.x, a.x - p.0).cross(&Vector3::new(c.y - a.y, b.y - a.y, a.y - p.1));

        Vector3::new(
            1.0 - (cross.y + cross.x) / cross.z,
            cross.y / cross.z,
            cross.x / cross.z,
        )
    }

    pub fn triangle(&self, frame: &mut [u8], v1: Vector3<f32>, v2: Vector3<f32>, v3: Vector3<f32>, stride: usize, color: &Color) {

    let x0 = vec![v1.x, v2.y, v3.z]
        .iter()
        .fold(&v1.x, |xmin, x| if xmin > x { x } else { xmin })
        .round() as i32;
    let y0 = vec![v1.y, v2.y, v3.y]
        .iter()
        .fold(&v1.y, |ymin, y| if ymin > y { y } else { ymin })
        .round() as i32;
    let x1 = vec![v1.x, v2.x, v3.x]
        .iter()
        .fold(&v1.x, |xmax, x| if xmax < x { x } else { xmax })
        .round() as i32;
    let y1 = vec![v1.y, v2.y, v3.y]
        .iter()
        .fold(&v1.y, |ymax, y| if ymax < y { y } else { ymax })
        .round() as i32;

    for y in y0..=y1 {
        for x in x0..=x1 {
            let bc = self.barycentric(&v1, &v2, &v3, (x as f32, y as f32));
            //sh.fragment(&bc);
            if bc.x < 0.0 || bc.y < 0.0 || bc.z < 0.0 {
                continue;
            }
            self.set(frame, x as usize, y as usize, stride, color);
        }
    }

    }

    pub fn line(&self, frame: &mut [u8], mut x0: i32, mut y0: i32, mut x1: i32, mut y1: i32, stride: usize, color: &Color) {

        let mut steep = false;

        if (x0 -x1).abs() < (y0 - y1).abs() {
            std::mem::swap(&mut x0, &mut y0);
            std::mem::swap(&mut x1, &mut y1);
            steep = true;
        }

        if x0 > x1 {
            std::mem::swap(&mut x0, &mut x1);
            std::mem::swap(&mut y0, &mut y1);
        }

        let dx = x1 - x0;
        let dy = y1 - y0;

        let derror2 = dy.abs() * 2;
        let mut error2 = 0;
        let mut y = y0;

        for x in x0..=x1 {
            if steep {
                self.set(frame, y as usize, x as usize, stride, color);
            } else {
                self.set(frame, x as usize, y as usize, stride, color);
            }
            error2 += derror2;
            if error2 > dx {
                y += if y1 > y0 { 1 } else {-1};//?1:-1);
                error2 -= dx*2;
            }
        }
    }

    pub fn set(&self, frame: &mut [u8], x: usize, y: usize, stride: usize, color: &Color) {
        let i = x * 4 + y * stride;
        frame[i..i + 4].copy_from_slice(color);
    }
}