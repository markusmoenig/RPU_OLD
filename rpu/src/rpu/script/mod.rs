use crate::prelude::*;

pub mod engine;
pub trait Script : Sync + Send {

    fn get_scope<'a>(&mut self) -> &'a Scope;
    fn get_engine<'a>(&self) -> &'a ScriptEngine;
    fn set_code_block(&mut self, name: String, code: String);
    fn execute(&mut self, code: String);
}

// F3
#[derive(PartialEq, Debug, Clone)]
pub struct F3 {
    pub value               : Vector3<F>
}

impl F3 {

    pub fn new(v: Vector3<F>) -> Self {
        Self {
            value           : v,
        }
    }

    pub fn new_1(x: F) -> Self {
        Self {
            value           : Vector3::new(x, x, x)
        }
    }

    pub fn new_3(x: F, y: F, z: F) -> Self {
        Self {
            value           : Vector3::new(x, y, z),
        }
    }

    fn get_x(&mut self) -> F {
        self.value.x
    }

    fn set_x(&mut self, new_val: F) {
        self.value.x = new_val;
    }

    fn get_y(&mut self) -> F {
        self.value.y
    }

    fn set_y(&mut self, new_val: F) {
        self.value.y = new_val;
    }

    fn get_z(&mut self) -> F {
        self.value.z
    }

    fn set_z(&mut self, new_val: F) {
        self.value.z = new_val;
    }
}