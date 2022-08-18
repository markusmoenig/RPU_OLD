//pub mod sphere;
//pub mod cube;

use crate::prelude::*;

pub trait Analytical : Sync + Send {
    fn new() -> Self where Self: Sized;

    fn update(&mut self) {}

    fn get_bounds(&self) -> (Vector3<F>, Vector3<F>);

    fn get_distance_normal_uv_face(&self, ray: &[Vector3<F>; 2]) -> Option<HitRecord>;

    fn execute(&mut self, code: String);

    fn set_code_block(&mut self, name: String, code: String);
}