pub mod pinhole;

use crate::prelude::*;

pub type Ray = [Vector3<F>; 2];

pub trait Camera3D : Sync + Script {

    fn new() -> Self where Self: Sized;
    fn gen_ray(&self, p: Vector2<F>) -> Ray;

    fn update(&mut self) {}
}
