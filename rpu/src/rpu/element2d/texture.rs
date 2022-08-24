use crate::prelude::*;

pub struct Texture<'a> {
    color               : Option<ColorBuffer<F>>,

    engine              : ScriptEngine<'a>,
}

impl Element2D for Texture<'_> {
    fn new() -> Self {

        let engine = ScriptEngine::new();

        Self {
            color           : None,

            engine,
        }
    }

    fn render(&mut self, node_index: usize, ctx: &Context) {
        let static_size = self.engine.get_vector2("size");

        if let Some(static_size) = static_size {
            let width = static_size.x as usize;
            let height = static_size.y as usize;
            let mut color = ColorBuffer::new(width, height, 1.0);

            for y in 0..height {
                let uv_y = (y as F / height as F) - 0.5;
                for x in 0..width {
                    let uv_x = (x as F / width as F) - 0.5;

                    let index = x * 4 + y * width * 4;

                    let mut uv = UV::new(GF2::new(uv_x, uv_y), GF4::new(0.0, 0.0, ctx.size[0] as F, ctx.size[1] as F), GF2::new(uv_x, uv_y));

                    let mut c = GF4::new(0.0, 0.0, 0.0, 1.0);
                    self.compute_color_at(&mut uv, &mut c, node_index, ctx);
                    color.pixels[index..index+4].copy_from_slice(&[c.x, c.y, c.z, c.w]);
                }
            }

            self.color = Some(color);
        }
    }

    fn get_color_at(&self, uv: &UV, node_index: usize, ctx: &Context) -> GF4 {

        if let Some(color) = &self.color {
            let xi = ((uv.p.x + 0.5) * color.size[0] as F).clamp(0.0, (color.size[0] - 1) as F) as usize;
            let yi = ((uv.p.y + 0.5) * color.size[1] as F).clamp(0.0, (color.size[1] - 1)as F) as usize;

            let index = xi * 4 + yi * color.size[0] * 4;

            return GF4::new(color.pixels[index], color.pixels[index+1], color.pixels[index+2], 1.0);
        } else {
            let mut c = GF4::new(0.0, 0.0, 0.0, 1.0);
            self.compute_color_at(uv, &mut c, node_index, ctx);
            return c;
        }
    }

    fn compute_color_at(&self, uv: &UV, color: &mut GF4, node_index: usize, ctx: &Context) {

        for child_index in &ctx.nodes[node_index].childs {
            match &ctx.nodes[*child_index].object {
                Object::Element2D(el) => el.compute_color_at(uv, color, *child_index, ctx),
                _ => {},
            }
        }

        for child_index in &ctx.nodes[node_index].elements {
            match &ctx.nodes[*child_index].object {
                Object::Element2D(el) => el.compute_color_at(uv, color, *child_index, ctx),
                _ => {},
            }
        }

        self.engine.execute_shader(uv, color);
    }

    fn get_size(&self) -> [usize; 2]
    {
        if let Some(color) = &self.color {
            return color.size;
        }
        [0, 0]
    }
}

impl Script for Texture<'_> {

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
