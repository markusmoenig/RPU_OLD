use crate::prelude::*;

#[derive(PartialEq, Clone, Debug)]
pub enum NodeType {
    Unknown,
    Element2D,
    Object3D,
    Layout3D,
    Texture,
}

use NodeType::*;

pub struct Node {

    pub childs              : Vec<usize>,
    pub elements            : Vec<usize>,

    pub object              : Object,
    pub texture             : Option<usize>,
    pub indention           : usize,
}

impl Node {

    pub fn new() -> Self {
        Self {
            childs          : vec![],
            elements        : vec![],

            object          : Object::Empty,
            texture         : None,
            indention       : 0
        }
    }

    /// Returns the type of the node object.
    pub fn get_node_type(&self) -> NodeType {
        match &self.object {
            Object::AnalyticalObject(_v) => Object3D,
            Object::SDF3D(_v) => Object3D,
            Object::Element2D(_v) => Element2D,
            _ => Unknown,
        }
    }
}