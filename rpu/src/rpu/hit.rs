use crate::prelude::*;

#[derive(Clone, Debug)]
pub struct HitRecord {
     pub distance             : F,
     pub normal               : Vector3<F>,
     /// Coordinates from -0.5..0.5
     pub uv                   : Vector2<F>,
     pub face                 : u8
}

pub struct BVHNode {
    pub index                 : usize,
    pub node_index            : usize,

    pub min                   : bvh::Vector3,
    pub max                   : bvh::Vector3,
}

impl Bounded for BVHNode {
    fn aabb(&self) -> AABB {
        AABB::with_bounds(self.min, self.max)
    }
}

impl BHShape for BVHNode{
    fn set_bh_node_index(&mut self, index: usize) {
        self.node_index = index;
    }
    fn bh_node_index(&self) -> usize {
        self.node_index
    }
}