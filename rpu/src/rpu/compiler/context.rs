
use crate::prelude::*;

pub struct Context {

    pub root                    : Node,
}

impl Context {

    pub fn new() -> Self {
        Self {
            root                : Node::new(),
        }
    }

    pub fn render(&mut self, frame: &mut [u8], rect: (usize, usize, usize, usize)) {

    }
}