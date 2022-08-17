use rhai::Map;

use crate::prelude::*;

pub struct Grid2D<'a> {

        map                 : HashMap<(usize, usize), usize>,
        engine              : ScriptEngine<'a>,
}

impl Layout3D for Grid2D<'_> {

    fn new() -> Self {

        let mut engine = ScriptEngine::new();

        Self {
            map             : HashMap::new(),
            engine,
        }
    }

    fn set_map_element(&mut self, map: HashMap<(usize, usize), usize>) {
        self.map = map;
    }


    fn traverse(&self, ray: &Ray, ctx: &Context) -> Option<HitRecord> {

        println!("here");
        None
    }

    fn execute(&mut self, code: String) {
        self.engine.execute(code);
    }

    fn set_code_block(&mut self, name: String, code: String) {
        self.engine.set_code_block(name, code);
    }
}