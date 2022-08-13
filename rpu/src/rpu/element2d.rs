pub mod texture;

use crate::prelude::*;

pub trait Element2D : Sync + Send {
    fn new() -> Self where Self: Sized;

    fn alloc(&mut self) {}

    fn get_color_at(&self, p: &[F; 2]) -> Color;
    fn get_size(&self) -> [usize; 2] { [0, 0] }

    fn execute(&mut self, code: String);
    fn set_code_block(&mut self, name: String, code: String);
}