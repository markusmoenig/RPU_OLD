use crate::prelude::*;

pub struct Vertical<'a> {
    engine              : ScriptEngine<'a>,
}

impl Element2D for Vertical<'_> {
    fn new() -> Self {

        let mut engine = ScriptEngine::new();
        engine.set_vector4("color", Vector4::new(0.0, 0.0, 0.0, 1.0));

        Self {
            engine,
        }
    }

    fn compute_color_at(&self, p: &[F; 2], color: &mut Color, rect: &mut UVRect, node_index: usize, ctx: &Context) {
        //println!("{}", ctx.nodes[node_index].elements.len());
        let v_el_size = 1.0 / ctx.nodes[node_index].elements.len() as F;
        //self.engine.execute_shader(p)

        let mut y = 0.0;
        for child_index in &ctx.nodes[node_index].elements {
            match &ctx.nodes[*child_index].object {

                Object::Element2D(el) => {

                    let el_rect = [0.0, y, 1.0, v_el_size];
                    if let Some(sub) = rect.create_from(*p, el_rect) {
                        el.compute_color_at(p, color, rect, *child_index, ctx);
                    }
                    y += v_el_size;
                },
                _ => {},
            }
        }
    }
}

impl Script for Vertical<'_> {

    fn get_scope<'a>(&mut self) -> &'a Scope {
        self.engine.get_scope()
    }

    fn get_engine<'a>(&self) -> &'a ScriptEngine {
        &self.engine
    }

    fn apply_properties(&mut self, props: Vec<Property>) -> Result<(), RPUError> {
        self.engine.apply_properties(props)
    }

    fn execute(&mut self, code: String) {
        self.engine.execute(code);
    }

    fn set_code_block(&mut self, name: String, code: String) {
        _ = self.engine.set_code_block(name, code);
    }
}
