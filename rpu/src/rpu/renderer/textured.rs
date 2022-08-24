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

    fn render(&self, ray: &Ray, object: &Object, ctx: &Context) -> GF4 {
        let mut c = GF4::new(0.0, 0.0, 0.0, 1.0);

            match object {
                Object::Layout3D(layout) => {

                    if let Some(hit) = layout.traverse3d(&ray,true, ctx) {

                        if let Some(texture_index) = &ctx.nodes[hit.node].texture {
                            match &ctx.nodes[*texture_index].object {
                                Object::Element2D(el) => {
                                    let p = hit.uv;

                                    let mut uv = UV::new(p, GF4::new(0.0, 0.0, ctx.size[0] as F, ctx.size[1] as F));
                                    c = el.get_color_at(&mut uv, *texture_index, ctx);
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