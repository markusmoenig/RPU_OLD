use crate::prelude::*;

#[derive()]
pub struct SDF3DCube<'a> {
        engine              : ScriptEngine<'a>,
}

impl SDF3D for SDF3DCube<'_> {

    fn new() -> Self {

        let mut engine = ScriptEngine::new();
        engine.set_vector3("position", Vector3::new(0.0, 0.0, 0.0));
        engine.set_vector3("size", Vector3::new(0.5, 0.5, 0.5));
        engine.set_vector3("rotation", Vector3::new(0.0, 0.0, 0.0));

        Self {
            engine,
        }
    }

    #[inline(always)]
    fn get_distance(&self, x: &Vector3<F>, instance: &Vector3<F>) -> F {

        let position = self.engine.get_vector3("position").unwrap();
        let size = self.engine.get_vector3("size").unwrap();
        //let rotation = self.engine.get_vector3("rotation").unwrap();

        let q : GF3 = glm::convert((x - position - instance).abs() - size);
        glm::length(&glm::max(&q, 0.0)) + q.x.max(q.y.max(q.z)).min(0.0)

        //vec3 q = abs(p) - b;
        //return length(max(q,0.0)) + min(max(q.x,max(q.y,q.z)),0.0);
    }
}

impl Script for SDF3DCube<'_> {

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