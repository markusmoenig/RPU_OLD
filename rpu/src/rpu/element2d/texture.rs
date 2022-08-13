use crate::prelude::*;

pub struct Texture<'a> {
    color               : Option<ByteBuffer>,

    engine              : ScriptEngine<'a>,
}

impl Element2D for Texture<'_> {
    fn new() -> Self {

        let mut engine = ScriptEngine::new();
        engine.set_int("width", 300);
        engine.set_int("height", 300);

        Self {
            color           : None,

            engine,
        }
    }

    fn alloc(&mut self) {
        let width = self.engine.get_int("width").unwrap();
        let height = self.engine.get_int("height").unwrap();

        self.color = Some(ByteBuffer::new(width as usize, height as usize, 255));
    }

    fn get_color_at(&self, p: &[F; 2]) -> Color {
        let mut c = [255, 0, 0, 255];

        //let [width, height] = self.get_size();
        let [x, y] = p;

        let t = SMatrix::<F, 2, 1>::new(*x, *y);

        if t.norm() - 0.5 <= 0.0 {
            c[1] = 255;

            // let xx = (*x + 1.0) / 2.0;
            // let yy = (*y + 1.0) / 2.0;

            // c[0] = (xx * 255.0) as u8;
            // c[1] = (yy * 255.0) as u8;
        }

        //println!("{:?}", p);
        c
    }

    fn get_size(&self) -> [usize; 2]
    {
        if let Some(color) = &self.color {
            return color.size;
        }
        [0, 0]
    }

    fn execute(&mut self, code: String) {
        self.engine.execute(code);
    }

    fn set_code_block(&mut self, name: String, code: String) {
        self.engine.set_code_block(name, code);
    }
}
