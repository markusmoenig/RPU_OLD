
//pub mod grid2d;
pub mod grid3d;

use crate::prelude::*;

#[allow(unused)]
pub trait Layout3D : Sync + Send {
    fn new() -> Self where Self: Sized;

    fn set_map2d(&mut self, map: HashMap<(i32, i32), usize>) {}
    fn set_map3d(&mut self, map: HashMap<(i32, i32, i32), usize>) {}

    fn traverse3d(&self, ray: &Ray, get_normal: bool, ctx: &Context) -> Option<HitRecord>;

    fn execute(&mut self, code: String);
    fn set_code_block(&mut self, name: String, code: String);
}