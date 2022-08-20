use crate::prelude::*;

pub struct Node {
    pub childs              : Vec<Node>,
    pub object              : Object,
    pub texture             : Option<usize>
}

impl Node {

    pub fn new() -> Self {
        Self {
            childs          : vec![],
            object          : Object::Empty,
            texture         : None,
        }
    }
}