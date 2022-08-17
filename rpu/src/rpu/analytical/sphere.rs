
use crate::prelude::*;

#[derive()]
pub struct AnalyticalSphere<'a> {
        engine              : ScriptEngine<'a>,
}

impl Analytical for AnalyticalSphere<'_> {

    fn new() -> Self {

        let mut engine = ScriptEngine::new();
        engine.set_vector3("position", Vector3::new(0.0, 0.0, 0.0));
        engine.set_float("radius", 1.0);

        Self {
            engine,
        }
    }

    fn get_bounds(&self) -> (Vector3<F>, Vector3<F>) {
        let p = self.engine.get_vector3("position").unwrap();
        let radius = self.engine.get_float("radius").unwrap();

        let position = Vector3::new(p.x, p.y, p.z);

        let half_size = Vector3::new(radius, radius, radius);
        let min = position - half_size;
        let max = position + half_size;
        (min, max)
    }

    fn execute(&mut self, code: String) {
        self.engine.execute(code);
    }

    fn set_code_block(&mut self, name: String, code: String) {
        self.engine.set_code_block(name, code);
    }

    /// https://www.shadertoy.com/view/4d2XWV
    fn get_distance_normal_uv_face(&self, ray: &[nalgebra::Vector3<F>; 2]) -> Option<HitRecord> {

        let [ro, rd] = ray;

        let p = self.engine.get_vector3("position").unwrap();
        let radius = self.engine.get_float("radius").unwrap();

        let oc = ro - p;
        let b = oc.dot(rd);
        let c = oc.dot(&oc) - radius * radius;
        let h = b*b - c;
        if h <0.0 { return None };
        let d = -b - h.sqrt();

        Some( HitRecord {
            distance        : d,
            normal          : Vector3::new(0.0 , 0.0, 0.0),
            uv              : Vector2::new(0.0, 0.0),
            face            : 0,
        })
    }
}
