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

pub struct RPU {
    color_buffer            : Buffer2d<u32>,
    depth_buffer            : Buffer2d<f32>,

    i                       : f32,
}

impl RPU {

    pub fn new(width: usize, height: usize) -> Self {

        let color_buffer = Buffer2d::new([width, height], 0 as u32);
        let depth_buffer = Buffer2d::new([width, height], 1.0);

        Self {
            color_buffer,
            depth_buffer,
            i               : 0.0,
        }
    }

    pub fn render(&mut self, frame: &mut [u8], rect: (usize, usize, usize, usize)) {
        let stride = rect.2 * 4;
        //self.set(frame, width / 2, height / 2, stride, &[255, 255, 255, 255]);
        //self.line(frame, 0, 0, 100, 100, stride, &[255, 255, 255, 255]);
        //self.triangle(frame, Vector3::new(0.0, 0.0, 0.0), Vector3::new( 0.0, 100.0, 0.0), Vector3::new( 100.0, 100.0, 0.0), stride, &[255, 255, 255, 255]);

        let mvp = Mat4::perspective_fov_rh_no(1.3, rect.2 as f32, rect.3 as f32, 0.01, 100.0)
            * Mat4::translation_3d(Vec3::new(0.0, 0.0, -2.0))
            * Mat4::<f32>::scaling_3d(0.4)
            * Mat4::rotation_x((self.i * 0.002).sin() * 8.0)
            * Mat4::rotation_y((self.i * 0.004).cos() * 4.0)
            * Mat4::rotation_z((self.i * 0.008).sin() * 2.0);

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

        self.i += 1.0;
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