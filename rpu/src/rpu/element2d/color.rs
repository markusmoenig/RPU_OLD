use crate::prelude::*;

pub struct ColorElement<'a> {
    engine                  : ScriptEngine<'a>,
    color                   : Vector4<F>,
}

impl Element2D for ColorElement<'_> {
    fn new() -> Self {

        let engine = ScriptEngine::new();

        Self {
            engine,
            color           : Vector4::new(0.0, 0.0, 0.0, 1.0),
        }
    }

    fn compute_color_at(&self, p: &[F; 2], color: &mut Color, rect: &mut UVRect, node: usize, ctx: &Context) {
        color[0] = self.color.x;
        color[1] = self.color.y;
        color[2] = self.color.z;
        color[3] = self.color.w;
        //self.engine.execute_shader(p)
    }
}

impl Script for ColorElement<'_> {

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
