use crate::prelude::*;

#[derive()]
pub struct Textured<'a> {
        engine              : ScriptEngine<'a>,
}

impl Renderer for Textured<'_> {

    fn new() -> Self {

        let engine = ScriptEngine::new();
        Self {
            engine,
        }
    }

    fn render(&self, ray: &Ray, object: &Object, ctx: &Context) -> Color {
        let mut c = [0.0, 0.0, 0.0, 1.0];

            match object {
                Object::Layout3D(layout) => {

                    if let Some(hit) = layout.traverse3d(&ray,true, ctx) {
                        c[0] = hit.normal.x;
                        c[1] = hit.normal.y;
                        c[2] = hit.normal.z;
                    }
                }
                _ => {},
            }
        c
    }

    fn execute(&mut self, code: String) {
        self.engine.execute(code);
    }

    fn set_code_block(&mut self, name: String, code: String) {
        self.engine.set_code_block(name, code);
    }
}