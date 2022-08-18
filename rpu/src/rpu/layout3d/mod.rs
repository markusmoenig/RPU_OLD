
//pub mod grid2d;
pub mod grid3d;

use crate::prelude::*;

pub trait Layout3D : Sync + Send {
    fn new() -> Self where Self: Sized;

    fn set_map2d(&mut self, map: HashMap<(isize, isize), usize>) {}
    fn set_map3d(&mut self, map: HashMap<(isize, isize, isize), usize>) {}

    fn traverse3d(&self, ray: &Ray, get_normal: bool, ctx: &Context) -> Option<HitRecord>;

    fn execute(&mut self, code: String);
    fn set_code_block(&mut self, name: String, code: String);
}