
use crate::prelude::*;

pub struct AnalyticalSphere<'a> {
        position            : Vector3<F>,
        scale               : F,
        rotation            : Vector3<F>,

        engine              : ScriptEngine<'a>,
    }


impl Analytical for AnalyticalSphere<'_> {

    fn new() -> Self {

        let mut engine = ScriptEngine::new();
        engine.set_vector3("position", Vector3::new(0.0, 0.0, 0.0));

        Self {
            position        : Vector3::new(0.0, 0.0, 0.0),
            scale           : 1.0,
            rotation        : Vector3::new(0.0, 0.0, 0.0),
            engine,
        }
    }

    fn execute(&mut self, code: String) {

    }

    fn set_code_block(&mut self, name: String, code: String) {
        self.engine.set_code_block(name, code);
    }

    /// https://www.shadertoy.com/view/4d2XWV
    fn get_distance_normal_uv_face(&self, ray: &[Vector3<F>; 2]) -> Option<(F, Vector3<F>, Vector2<F>, u8)> {

        let [ro, rd] = ray;

        let sph = Vector4::new(0.0, 0.0, 0.0, 1.0);

        let oc = ro - sph.xyz();
        let b = oc.dot(rd);
        let c = oc.dot(&oc) - sph.w * sph.w;
        let h = b*b - c;
        if h <0.0 { return None };
        let d = -b - h.sqrt();
        Some((d, Vector3::new(0.0 , 0.0, 0.0), Vector2::new(0.0, 0.0), 0))
    }
}