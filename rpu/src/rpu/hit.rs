use crate::prelude::*;

#[derive(Clone, Debug)]
pub struct HitRecord {
     pub distance             : F,
     pub normal               : Vector3<F>,
     /// Coordinates from -0.5..0.5
     pub uv                   : Vector2<F>,
     pub face                 : u8
}
