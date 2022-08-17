
pub mod grid2d;

use crate::prelude::*;

pub trait Layout3D : Sync + Send {
    fn new() -> Self where Self: Sized;

    fn set_map_element(&mut self, map: HashMap<(usize, usize), usize>);
    fn traverse(&self, ray: &Ray, ctx: &Context) -> Option<HitRecord>;

    fn execute(&mut self, code: String);
    fn set_code_block(&mut self, name: String, code: String);
}