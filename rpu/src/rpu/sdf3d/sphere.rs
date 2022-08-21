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

    #[inline(always)]
    fn get_distance(&self, x: &Vector3<F>, instance: &Vector3<F>) -> F {

        let position = self.engine.get_vector3("position").unwrap();
        let radius = self.engine.get_float("radius").unwrap();

        let p = (x - position - instance).norm() - radius;

        return p;
    }
}

impl Script for SDF3DSphere<'_> {

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