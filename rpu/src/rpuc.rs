//! [![crates.io](https://img.shields.io/crates/v/euc.svg)](https://crates.io/crates/euc)
//! [![crates.io](https://docs.rs/euc/badge.svg)](https://docs.rs/euc)
//!
//! <img src="misc/example.png" alt="Utah teapot, rendered with Euc" width="100%"/>
//!
//! # Example
//! ```ignore
//! struct Example;
//!
//! impl Pipeline for Example {
//!     type Vertex = [f32; 2];
//!     type VsOut = ();
//!     type Pixel = [u8; 4];
//!
//!     // Vertex shader
//!     fn vert(&self, pos: &Self::Vertex) -> ([f32; 3], Self::VsOut) {
//!         ([pos[0], pos[1], 0.0], ())
//!     }
//!
//!     // Fragment shader
//!     fn frag(&self, _: &Self::VsOut) -> Self::Pixel {
//!         [255, 0, 0, 255] // Red
//!     }
//! }
//!
//! fn main() {
//!     let mut color = Buffer2d::new([640, 480], [0; 4]);

//!     Example.draw::<Triangles<(f32,)>, _>(
//!         &[
//!             [-1.0, -1.0],
//!             [ 1.0, -1.0],
//!             [ 0.0,  1.0],
//!         ],
//!         &mut color,
//!         None,
//!     );
//! }
//! ```

//#![no_std]

//#[macro_use]
//extern crate alloc;

pub mod buffer;
pub mod interpolate;
pub mod rasterizer;

// Reexports
pub use self::{
    interpolate::Interpolate,
    rasterizer::{DepthStrategy, Rasterizer},
};

/// Represents the high-level structure of a rendering pipeline.
///
/// Conventionally, uniform data is stores as state within the type itself.
///
/// This governs the following things:
///
/// - Vertex position and data calculation (computed by the vertex shader)
/// - Determining whether each polygon is 'backfacing', and optionally skipping it
/// - Rasterization (performed internally by `euc`)
/// - Comparing the fragment depth against the depth buffer to determine whether it is occluded,
///   and optionally skipping it
/// - Fragment output calculation (computed by the fragment shader)
///
/// In the future, `euc` may extend its capabilities to include compute, geometry, and tesselation
/// shaders.
pub trait Pipeline
where
    Self: Sized,
{
    /// The type of the vertex shader input data.
    ///
    /// This usually consists of the vertex's position, normal, colour, texture coordinates, and
    /// other such per-vertex information. When vertex indexing is used, this tends to consist of
    /// the vertex index.
    type Vertex;

    /// The type of the data that gets passed on from the vertex shader to the fragment shader.
    ///
    /// This usually consists of the fragment's normal, colour, texture coordinates and other such
    /// per-fragment information.
    type VsOut: Clone + Interpolate;

    /// The type of emitted pixels.
    ///
    /// This type is emitted by the fragment shader and usually corresponds to the colour of the
    /// pixel.
    type Pixel: Clone;

    /// The vertex shader
    fn vert(&self, vertex: &Self::Vertex) -> ([f32; 4], Self::VsOut);

    /// The fragment shader
    fn frag(&self, vs_out: &Self::VsOut) -> Self::Pixel;

    /// A method used to determine what depth buffer strategy should be used when determining
    /// fragment occlusion.
    ///
    /// This method will be called at minimum only once per draw call, but may be called an
    /// arbitrary number of times.
    #[inline(always)]
    fn get_depth_strategy(&self) -> DepthStrategy {
        DepthStrategy::IfLessWrite
    }

    /// Perform a draw call with the given uniform data, vertex array, output target and supplement
    /// type.
    ///
    /// The supplement type is commonly used to represent additional surfaces required by the
    /// rasterizer, such as a depth buffer target.
    fn draw<R: Rasterizer, T: Target<Item = Self::Pixel>>(
        &self,
        vertices: &[Self::Vertex],
        target: &mut T,
        supplement: <R as Rasterizer>::Supplement,
    ) {
        R::draw::<Self, T>(self, vertices, target, supplement)
    }
}

/// Represents a 2-dimensional rendering target that can have pixel data read and written to it.
pub trait Target {
    /// The type of items contained within this target.
    type Item: Clone;

    /// Get the dimensions of the target.
    fn size(&self) -> [usize; 2];

    /// Set the item at the specified location in the target to the given item. The validity of the
    /// location is not checked, and as such this method is marked `unsafe`.
    unsafe fn set(&mut self, pos: [usize; 2], item: Self::Item);

    /// Get a copy of the item at the specified location in the target. The validity of the
    /// location is not checked, and as such this method is marked `unsafe`.
    unsafe fn get(&self, pos: [usize; 2]) -> Self::Item;

    /// Clear the target with copies of the specified item.
    fn clear(&mut self, fill: Self::Item);
}

impl<T: Default + Clone> Target for (T,) {
    type Item = T;

    fn size(&self) -> [usize; 2] {
        [1; 2]
    }

    unsafe fn set(&mut self, _pos: [usize; 2], _item: Self::Item) {}

    unsafe fn get(&self, _pos: [usize; 2]) -> Self::Item {
        Self::Item::default()
    }

    fn clear(&mut self, _fill: Self::Item) {}
}