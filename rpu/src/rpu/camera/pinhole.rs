
use crate::prelude::*;

pub struct Pinhole<'a> {

    engine              : ScriptEngine<'a>,
}

impl Camera3D for Pinhole<'_> {

    fn new() -> Self {

        let mut engine = ScriptEngine::new();
        engine.set_vector3("origin", Vector3::new(0.0, 2.0, 5.0));
        engine.set_vector3("center", Vector3::new(0.0, 0.0, 0.0));

        Self {
            engine,
        }
    }

    fn update(&mut self) {
        self.engine.execute_block("update".to_string());
    }

    #[inline(always)]
    fn gen_ray(&self, p: Vector2<F>) -> [Vector3<F>; 2] {

        let origin = self.engine.get_vector3("origin").unwrap();
        let center = self.engine.get_vector3("center").unwrap();


        let ww = (center - origin).normalize();
        let uu = ww.cross(&Vector3::new(0.0, 1.0, 0.0)).normalize();
        let vv = uu.cross(&ww).normalize();

        let rd = (p.x * uu + p.y * vv + 2.0 * ww).normalize();

        [origin, rd]
    }

}

impl Script for Pinhole<'_> {

    fn get_scope<'a>(&mut self) -> &'a Scope {
        self.engine.get_scope()
    }

    fn get_engine<'a>(&self) -> &'a ScriptEngine {
        &self.engine
    }

    fn execute(&mut self, code: String) {
        self.engine.execute(code);
    }

    fn set_code_block(&mut self, name: String, code: String) {
        _ = self.engine.set_code_block(name, code);
    }
}

