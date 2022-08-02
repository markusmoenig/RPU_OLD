use crate::prelude::*;

pub struct Node {
    pub childs              : Vec<Node>,
    pub object              : Object,
}

impl Node {

    pub fn new() -> Self {
        Self {
            childs          : vec![],
            object          : Object::Empty
        }
    }
}