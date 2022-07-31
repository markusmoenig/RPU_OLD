pub mod pinhole;

use crate::prelude::*;

pub trait Camera3D : Sync {

    fn new() -> Self where Self: Sized;
    fn gen_ray(&self, p: Vector2<F>) -> [Vector3<F>; 2];
}
