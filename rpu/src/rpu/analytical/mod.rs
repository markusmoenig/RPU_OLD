pub mod sphere;
pub mod cube;

use crate::prelude::*;

pub trait Analytical : Sync {
    fn new() -> Self where Self: Sized;

    fn get_rotation(&mut self) -> &mut Vector3<F>;
    fn set_rotation(&mut self, rot: Vector3<F>);

    fn update(&mut self) {}

    fn get_distance_and_normal(&self, ray: &[Vector3<F>; 2]) -> Option<(F, Vector3<F>)>;
}