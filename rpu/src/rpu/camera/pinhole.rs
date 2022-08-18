
use crate::prelude::*;

pub struct Pinhole {

    origin          :  Vector3<F>,
    center          :  Vector3<F>
}

impl Camera3D for Pinhole {

    fn new() -> Self {

        let origin = Vector3::new(0.0, 0.0, 10.0);
        let center = Vector3::new(0.0, 0.0, 0.0);

        Self {
            origin,
            center,
        }
    }

    #[inline(always)]
    fn gen_ray(&self, p: Vector2<F>) -> [Vector3<F>; 2] {
        let ww = (self.center - self.origin).normalize();
        let uu = ww.cross(&Vector3::new(0.0, 1.0, 0.0)).normalize();
        let vv = uu.cross(&ww).normalize();

        let rd = (p.x * uu + p.y * vv + 2.0 * ww).normalize();

        [self.origin, rd]
    }
}

