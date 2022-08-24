use crate::prelude::*;

#[derive(Clone, Debug)]
pub struct HitRecord {
     pub distance             : F,
     pub node                 : usize,
     pub normal               : Vector3<F>,
     pub hit_point            : Vector3<F>,
     pub mask                 : GF3,
     /// Coordinates from -0.5..0.5
     pub uv                   : GF2,
     pub uv_world             : GF2,
}
