pub mod pinhole;

use crate::prelude::*;

pub type Ray = [Vector3<F>; 2];

pub trait Camera3D : Sync {

    fn new() -> Self where Self: Sized;
    fn gen_ray(&self, p: Vector2<F>) -> Ray;

    fn update(&mut self) {}

    fn execute(&mut self, code: String);
    fn set_code_block(&mut self, name: String, code: String);
}
