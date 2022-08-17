pub mod sphere;
//pub mod cube;

use crate::prelude::*;

pub trait SDF3D : Sync + Send {
    fn new() -> Self where Self: Sized;

    fn get_distance(&self, x: &Vector3<F>, instance: &Vector3<F>) -> F;

    fn execute(&mut self, code: String);
    fn set_code_block(&mut self, name: String, code: String);
}