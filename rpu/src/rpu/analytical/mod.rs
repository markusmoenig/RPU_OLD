//pub mod sphere;
//pub mod cube;
pub mod voxel;

use crate::prelude::*;

pub trait Analytical : Sync + Send + Script {
    fn new() -> Self where Self: Sized;

    fn update(&mut self) {}

    //fn get_distance_normal_uv_face(&self, ray: &[Vector3<F>; 2]) -> Option<HitRecord>;
    fn get_distance(&self, ray: &[Vector3<F>; 2]) -> Option<F>;
}