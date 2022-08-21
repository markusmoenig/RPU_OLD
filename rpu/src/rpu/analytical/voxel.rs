
use crate::prelude::*;

#[derive()]
pub struct AnalyticalVoxel<'a> {
        engine              : ScriptEngine<'a>,
}

impl Analytical for AnalyticalVoxel<'_> {

    fn new() -> Self {

        let engine = ScriptEngine::new();

        Self {
            engine,
        }
    }

    /// https://www.shadertoy.com/view/4d2XWV
    fn get_distance(&self, ray: &[nalgebra::Vector3<F>; 2]) -> Option<F>{
        Some(0.0)
    }
}

impl Script for AnalyticalVoxel<'_> {

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