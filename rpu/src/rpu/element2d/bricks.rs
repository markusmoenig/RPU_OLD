use crate::prelude::*;

pub struct Bricks<'a> {
    engine                  : ScriptEngine<'a>,
    color                   : GF4,
}

impl Element2D for Bricks<'_> {
    fn new() -> Self {

        let engine = ScriptEngine::new();

        Self {
            engine,
            color           : Vector4::new(0.0, 0.0, 0.0, 1.0),
        }
    }

    fn compute_color_at(&self, uv : &UV, color: &mut GF4, _node: usize, _ctx: &Context) {

        let cell = 1.0;
        let ratio = 3.0;
        let brick = 1.0;

        let mut u = uv.world * 8.0;

        let bevelx = 0.07;
        let bevel = GF2::new(bevelx, bevelx);
        let gap_x = 0.08;
        let gap = GF2::new(gap_x, gap_x);
        let round = 0.2;
        //let missing = 0.0;

        let w = GF2::new(ratio,1.0);
        u = u.component_mul(&GF2::new(cell, cell).component_div(&w));

        if brick == 1.0 {
            u.x += 0.5 * u.y.floor() % 2.0;
        }

        //hash = hash21(floor(U))

        let t = glm::fract(&u) - GF2::new(1.0, 1.0) / 2.0;
        let s = w.component_mul(&t);

        let a = w / 2.0 - gap - glm::abs(&s);
        let b = a.component_mul(&GF2::new(2.0, 2.0)).component_div(&bevel);
        let mut m = b.x.min(b.y);
        if a.x < round && a.y < round {
           m = (round - glm::length(&(GF2::new(round, round) - a))) * 2.0 / glm::dot(&bevel,&glm::normalize(&(GF2::new(round, round) - a)));
        }

        //if MISSING > missingHash(floor(U)) {
        //    isMissing = true
        //}

        *color = GF4::new(m, m, m, m);
        self.engine.execute_shader(uv, color);
    }
}

impl Script for Bricks<'_> {

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
