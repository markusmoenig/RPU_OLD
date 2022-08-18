pub mod sphere;
//pub mod cube;

use crate::prelude::*;

pub trait SDF3D : Sync + Send {
    fn new() -> Self where Self: Sized;

    fn get_distance(&self, x: &Vector3<F>, instance: &Vector3<F>) -> F;
    fn get_normal(&self, x: &Vector3<F>, instance: &Vector3<F>) -> Vector3<F> {

        //let mut n = Vector3::new(0.0, 0.0, 0.0);

        let e = Vector2::new(1.0,-1.0)*0.5773*0.0005;

        let mut n = e.xyy() * self.get_distance(&(x + e.xyy()), instance);
        n += e.yyx() * self.get_distance(&(x + e.yyx()), instance);
        n += e.yxy() * self.get_distance(&(x + e.yxy()), instance);
        n += e.xxx() * self.get_distance(&(x + e.xxx()), instance);
        n.normalize()

        /*
    vec2 e = vec2(1.0,-1.0)*0.5773*0.0005;
    return normalize( e.xyy*map( pos + e.xyy ).x +
					  e.yyx*map( pos + e.yyx ).x +
					  e.yxy*map( pos + e.yxy ).x +
					  e.xxx*map( pos + e.xxx ).x );*/

    }

    fn execute(&mut self, code: String);
    fn set_code_block(&mut self, name: String, code: String);
}