use crate::prelude::*;

pub struct Noise<'a> {
    engine                  : ScriptEngine<'a>,
    color                   : GF4,
}

impl Element2D for Noise<'_> {
    fn new() -> Self {

        let engine = ScriptEngine::new();

        Self {
            engine,
            color           : Vector4::new(0.0, 0.0, 0.0, 1.0),
        }
    }

    fn compute_color_at(&self, uv : &UV, color: &mut GF4, _node: usize, _ctx: &Context) {
        use noise::{NoiseFn, Perlin};

        let value = Perlin::new();
        let v = value.get([uv.world.x * 20.0, uv.world.y * 20.0]);

        *color = GF4::new(v, v, v, v);
        self.engine.execute_shader(uv, color);
    }
}

impl Script for Noise<'_> {

    fn get_scope<'a>(&mut self) -> &'a Scope {
        self.engine.get_scope()
    }

    fn get_engine<'a>(&self) -> &'a ScriptEngine {
        &self.engine
    }

    fn apply_properties(&mut self, props: Vec<Property>) -> Result<(), RPUError> {
        let rc = self.engine.apply_properties(props);
        if let Some(color) = self.engine.get_vector4("color") {
            self.color = color;
        }
        rc
    }

    fn execute(&mut self, code: String) {
        self.engine.execute(code);
    }

    fn set_code_block(&mut self, name: String, code: String) {
        _ = self.engine.set_code_block(name, code);
    }
}
