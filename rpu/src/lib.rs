pub mod rpu;

pub use crate::rpu::RPU as RPU;
pub use crate::rpu::buffer::Buffer as Buffer;

pub type I = i32;
pub type F = f32;
pub type Color = [u8; 4];

pub mod prelude {
    pub use nalgebra::*;
    pub use crate::rpu::RPU;

    pub use crate::rpu::buffer::Buffer;
    pub use crate::rpu::buffer::ByteBuffer;

    pub use crate::rpu::script::ScriptEngine;
    pub use crate::rpu::element2d::Element2D;

    pub use crate::rpu::camera::Camera3D;
    pub use crate::rpu::camera::pinhole::Pinhole;

    pub use crate::rpu::analytical::Analytical;
    pub use crate::rpu::analytical::sphere::AnalyticalSphere;
    pub use crate::rpu::analytical::cube::AnalyticalCube;

    pub use crate::rpu::compiler::Compiler;
    pub use crate::rpu::compiler::Error;
    pub use crate::rpu::compiler::scanner::Scanner;
    pub use crate::rpu::compiler::scanner::Token;
    pub use crate::rpu::compiler::context::Context;
    pub use crate::rpu::compiler::node::Node;
    pub use crate::rpu::compiler::object::Object;
    pub use crate::rpu::compiler::context::*;

    pub use crate::rpu::hit::*;

    pub use crate::I;
    pub use crate::F;
    pub use crate::Color;

    pub use std::path::PathBuf;

    pub use rhai::{Engine, Scope};
}

#[macro_use]
extern crate alloc;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
