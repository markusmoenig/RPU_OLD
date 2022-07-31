pub mod rpu;

pub use crate::rpu::RPU as RPU;
pub use crate::rpu::buffer::Buffer as Buffer;

pub type F = f32;
pub type Color = [u8; 4];

pub mod prelude {
    pub use nalgebra::*;
    pub use crate::rpu::RPU;

    pub use crate::rpu::buffer::Buffer;
    pub use crate::rpu::buffer::ByteBuffer;

    pub use crate::rpu::camera::Camera3D;
    pub use crate::rpu::camera::pinhole::Pinhole;

    pub use crate::rpu::analytical::Analytical;
    pub use crate::rpu::analytical::sphere::AnalyticalSphere;
    pub use crate::rpu::analytical::cube::AnalyticalCube;

    pub use crate::rpu::world::World;

    pub use crate::F;
    pub use crate::Color;
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
