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

    fn render(&mut self) {

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

                    let c = self.compute_color_at(&[uv_x, uv_y]);
                    color.pixels[index..index+4].copy_from_slice(&c);
                }
            }

            self.color = Some(color);
        }
    }

    fn get_color_at(&self, p: &[F; 2]) -> Color {

        if let Some(color) = &self.color {
            let [x, y] = p;
            let xi = ((x + 0.5) * color.size[0] as F).clamp(0.0, (color.size[0] - 1) as F) as usize;
            let yi = ((y + 0.5) * color.size[1] as F).clamp(0.0, (color.size[1] - 1)as F) as usize;

            let index = xi * 4 + yi * color.size[0] * 4;

            let mut c = [1_f32;4];
            c[0] = color.pixels[index];
            c[1] = color.pixels[index+1];
            c[2] = color.pixels[index+2];

            return c;
        } else {
            return self.compute_color_at(p);
        }
    }

    fn compute_color_at(&self, p: &[F; 2]) -> Color {
        self.engine.execute_shader(p)
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

    fn apply_properties(&mut self, props: Vec<Property>) {
        self.engine.apply_properties(props);
    }

    fn execute(&mut self, code: String) {
        self.engine.execute(code);
    }

    fn set_code_block(&mut self, name: String, code: String) {
        _ = self.engine.set_code_block(name, code);
    }
}
