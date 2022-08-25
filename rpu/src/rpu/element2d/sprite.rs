use crate::prelude::*;

pub struct Sprite<'a> {
    pub position            : GF3,
    pub texture             : Option<usize>,

    engine                  : ScriptEngine<'a>,
}

impl Element2D for Sprite<'_> {
    fn new () -> Self {

        let engine = ScriptEngine::new();

        Self {
            position        : GF3::new(0.0, 0.0, 0.0),
            texture         : None,
            engine
        }
    }

    fn name(&self) -> String {
        "Noise".to_string()
    }

    fn get_position(&self) -> Option<GF3> {
        Some(self.position)
     }

    fn get_texture(&self) -> Option<usize> {
        self.texture
    }

    fn compute_color_at(&self, uv : &UV, color: &mut GF4, _node: usize, _ctx: &Context) {


        self.engine.execute_shader(uv, color);
    }
}

impl Script for Sprite<'_> {

    fn get_scope<'a>(&mut self) -> &'a Scope {
        self.engine.get_scope()
    }

    fn get_engine<'a>(&self) -> &'a ScriptEngine {
        &self.engine
    }

    fn apply_properties(&mut self, props: Vec<Property>) -> Result<(), RPUError> {
        let rc = self.engine.apply_properties(props);
        if let Some(position) = self.engine.get_vector3("position") {
            self.position = position;
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