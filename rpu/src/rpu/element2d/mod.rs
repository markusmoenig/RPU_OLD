pub mod texture;
pub mod vertical;
pub mod color;

use crate::prelude::*;

#[allow(unused)]
pub trait Element2D : Sync + Send + Script {
    fn new() -> Self where Self: Sized;

    fn render(&mut self, node: usize, ctx: &Context) {}

    fn get_color_at(&self, p: &[F; 2], rect: &mut UVRect, node: usize, ctx: &Context) -> Color { [0.0, 0.0, 0.0, 1.0] }
    fn compute_color_at(&self, p: &[F; 2], color: &mut Color, rect: &mut UVRect, node: usize, ctx: &Context);
    fn get_size(&self) -> [usize; 2] { [0, 0] }
}

pub struct UVRect {
    pub rect                : [F; 4],
    pub global_size         : [F; 2]
}

impl UVRect {

    pub fn new(size: [usize;2]) -> Self {
        Self {
            rect            : [0.0, 0.0, size[0] as F, size[1] as F],
            global_size     : [size[0] as F, size[1] as F],
        }
    }

    pub fn new_sub(rect: [F;4]) -> Self {
        Self {
            rect,
            global_size     : [rect[2] as F, rect[3] as F],
        }
    }

    /// Creates a new uv and UVRect from the given p and the normalized subrect, both relative to this rect. None if p does not fit in the new subrect.
    pub fn create_from(&self, p: [F;2], srn: [F; 4]) -> Option<([F;2], UVRect)> {

        let new_x = self.rect[0] + srn[0] * self.rect[2];
        let new_y = self.rect[1] + srn[1] * self.rect[3];
        let new_width = srn[2] * self.rect[2];
        let new_height = srn[3] * self.rect[3];

        let px = (p[0] + 0.5) * self.rect[2];
        let py = (p[1] + 0.5) * self.rect[3];

        if px >= new_x && px <= new_x + new_width && py >= new_y && py <= new_y + new_height {
            return Some(([1.0, 1.0], UVRect::new_sub([new_x, new_y, new_width, new_height])));
        }
        None
    }
}