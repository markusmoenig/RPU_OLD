use crate::prelude::*;

#[derive()]
pub struct SDF3DSphere<'a> {
        engine              : ScriptEngine<'a>,
}

impl SDF3D for SDF3DSphere<'_> {

    fn new() -> Self {

        let mut engine = ScriptEngine::new();
        engine.set_vector3("position", Vector3::new(0.0, 0.0, 0.0));
        engine.set_float("radius", 0.5);

        Self {
            engine,
        }
    }

    fn execute(&mut self, code: String) {
        self.engine.execute(code);
    }

    fn set_code_block(&mut self, name: String, code: String) {
        self.engine.set_code_block(name, code);
    }

    #[inline(always)]
    fn get_distance(&self, x: &Vector3<F>, instance: &Vector3<F>) -> F {

        let position = self.engine.get_vector3("position").unwrap();
        let radius = self.engine.get_float("radius").unwrap();

        let p = (x - position - instance).norm() - radius;

        return p;
    }
}