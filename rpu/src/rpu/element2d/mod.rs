pub mod texture;
pub mod vertical;
pub mod color;

use crate::prelude::*;

#[allow(unused)]
pub trait Element2D : Sync + Send + Script {
    fn new() -> Self where Self: Sized;

    fn render(&mut self, node: usize, ctx: &Context) {}

    fn get_color_at(&self, uv: &UV, node: usize, ctx: &Context) -> GF4 { GF4::new(0.0, 0.0, 0.0, 1.0) }
    fn compute_color_at(&self, uv: &UV, color: &mut GF4, node: usize, ctx: &Context);
    fn get_size(&self) -> [usize; 2] { [0, 0] }
}

pub struct UV {
    pub p                   : GF2,
    pub rect                : GF4,
}

impl UV {

    pub fn new(p: GF2, rect: GF4) -> Self {
        Self {
            p,
            rect,
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
            return Some(UV::new(GF2::new(dx, dy), GF4::new(new_x, new_y, new_width, new_height)));
        }
        None
    }
}