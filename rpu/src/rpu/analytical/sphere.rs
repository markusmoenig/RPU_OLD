
use crate::prelude::*;

pub struct AnalyticalSphere {

}

impl Analytical for AnalyticalSphere {

    fn new() -> Self {
        Self {

        }
    }

    /// https://www.shadertoy.com/view/4d2XWV
    fn get_distance_and_normal(&self, ray: &[Vector3<F>; 2]) -> Option<(F, Vector3<F>)> {

        let [ro, rd] = ray;

        let sph = Vector4::new(0.0, 0.0, 0.0, 1.0);

        let oc = ro - sph.xyz();
        let b = oc.dot(rd);
        let c = oc.dot(&oc) - sph.w * sph.w;
        let h = b*b - c;
        if h <0.0 { return None };
        let d = -b - h.sqrt();
        Some((d, Vector3::new(0.0 , 0.0, 0.0)))
    }
}