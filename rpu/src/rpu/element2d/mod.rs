pub mod texture;
pub mod vertical;
pub mod color;
pub mod noise;
pub mod bricks;
pub mod sprite;

use crate::prelude::*;

#[allow(unused)]
pub trait Element2D : Sync + Send + Script {
    fn new() -> Self where Self: Sized;

    fn name(&self) -> String {"".to_string()}
    fn render(&mut self, node: usize, ctx: &Context) {}

    fn get_color_at(&self, uv: &UV, node: usize, ctx: &Context) -> GF4 { GF4::new(0.0, 0.0, 0.0, 1.0) }
    fn compute_color_at(&self, uv: &UV, color: &mut GF4, node: usize, ctx: &Context);
    fn get_size(&self) -> [usize; 2] { [0, 0] }

    // For sprites

    fn get_position(&self) -> Option<GF3> { None }
    fn get_texture(&self) -> Option<usize> { None }
}

pub struct UV {
    pub p                   : GF2,
    pub rect                : GF4,
    pub world               : GF2,
}

impl UV {

    pub fn new(p: GF2, rect: GF4, world: GF2) -> Self {
        Self {
            p,
            rect,
            world,
        }
    }

    /// Creates a new uv and UVRect from the given p and the normalized subrect, both relative to this rect. None if p does not fit in the new subrect.
    pub fn create_sub(&self, srn: GF4) -> Option<UV> {
        let new_x = self.rect[0] + srn[0] * self.rect[2];
        let new_y = self.rect[1] + srn[1] * self.rect[3];
        let new_width = srn[2] * self.rect[2];
        let new_height = srn[3] * self.rect[3];

        let px = (self.p[0] + 0.5) * self.rect[2];
        let py = (self.p[1] + 0.5) * self.rect[3];

        if px >= new_x && px <= new_x + new_width && py >= new_y && py <= new_y + new_height {
            let dx = (px - new_x) / new_width - 0.5;
            let dy = (py - new_y) / new_height - 0.5;
            return Some(UV::new(GF2::new(dx, dy), GF4::new(new_x, new_y, new_width, new_height), self.world));
        }
        None
    }

    pub fn pixelate(&self, v: F) -> UV {

        let r = (self.rect[2] / self.rect[3]) * (100.0 - v);
        let pixel_size = GF2::new(r, r);

        let mut n = UV::new(self.p, self.rect, self.world);

        n.p = glm::floor(&self.p.component_mul(&pixel_size)).component_div(&pixel_size);
        n.world = glm::floor(&self.world.component_mul(&pixel_size)).component_div(&pixel_size);
        //rc.x += 1.0 / (pixel_size.x * 2.0);
        //rc.y += 1.0 / (pixel_size.y * 2.0);
        n
    }
}