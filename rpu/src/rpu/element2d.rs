pub mod texture;

use crate::prelude::*;

pub trait Element2D : Sync + Send {
    fn new() -> Self where Self: Sized;

    //fn update(&mut self) {}
    //fn get_distance_normal_uv_face(&self, ray: &[Vector3<F>; 2]) -> Option<(F, Vector3<F>, Vector2<F>, u8)>;

    fn alloc(&mut self) {}

    fn get_color_at(&self, p: &[usize; 2]) -> Color;
    fn get_size(&self) -> [usize; 2] { [0, 0] }


    fn execute(&mut self, code: String);
    fn set_code_block(&mut self, name: String, code: String);
}