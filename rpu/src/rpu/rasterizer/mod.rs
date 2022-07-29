pub mod fast;

use crate::prelude::*;

pub trait Rasterizer {

    fn new() -> Self where Self: Sized;
    fn render(&self, camera: &Box<dyn Camera3D>, color: &mut Buffer<u32>, depth: &mut Buffer<f32>);
    fn get_pixel(&self, ray: &[Vector3<F>; 2]) -> Color;
}