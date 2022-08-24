use crate::prelude::*;

pub mod textured;

pub trait Renderer : Sync + Send {
    fn new() -> Self where Self: Sized;

    fn render(&self, ray: &Ray,  object: &Object, ctx: &Context) -> GF4;

    fn execute(&mut self, code: String);
    fn set_code_block(&mut self, name: String, code: String);
}