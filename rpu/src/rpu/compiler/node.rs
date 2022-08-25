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

    pub id                  : String,
    pub childs              : Vec<usize>,
    pub elements            : Vec<usize>,

    pub object              : Object,
    pub texture             : Option<usize>,

    pub indent              : usize,
}

impl Node {

    pub fn new(id: String) -> Self {
        Self {
            id,
            childs          : vec![],
            elements        : vec![],

            object          : Object::Empty,
            texture         : None,
            indent          : 0
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