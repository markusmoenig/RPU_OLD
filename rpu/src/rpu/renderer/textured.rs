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

                        if let Some(texture_index) = &ctx.nodes[hit.node].texture {
                            match &ctx.nodes[*texture_index].object {
                                Object::Element2D(el) => {
                                    let uv = hit.uv;
                                    c = el.get_color_at(&[uv.x, uv.y], &mut UVRect::new(ctx.size), *texture_index, ctx);
                                },
                                _ => {},
                            }
                        } else {
                            c[0] = hit.uv.x + 0.5;
                            c[1] = hit.uv.y + 0.5;
                        }
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
        _ = self.engine.set_code_block(name, code);
    }
}