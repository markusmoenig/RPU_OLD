use crate::prelude::*;

pub struct Grid3D<'a> {

        map                 : HashMap<(isize, isize, isize), usize>,
        engine              : ScriptEngine<'a>,
}

impl Layout3D for Grid3D<'_> {

    fn new() -> Self {

        let engine = ScriptEngine::new();

        Self {
            map             : HashMap::new(),
            engine,
        }
    }

    fn set_map3d(&mut self, map: HashMap<(isize, isize, isize), usize>) {
        self.map = map;
    }


    fn traverse3d(&self, ray: &Ray, get_normal: bool, ctx: &Context) -> Option<HitRecord> {

        // 3D Grid traversal based on https://www.shadertoy.com/view/XtlfWs
        // CC BY 3.0 by mattz

        // ray-box intersection
        #[inline(always)]
        fn ray_box(ro: &Vector3<F>, rd: &Vector3<F>, b: Vector3<F>) -> Option<F> {
            let rdi = Vector3::new(1.0 / rd.x, 1.0 / rd.y, 1.0 / rd.z);
            let t1 = Vector3::new(
                (-b.x - ro.x) * rdi.x,
                (-b.y - ro.y) * rdi.y,
                (-b.z - ro.z) * rdi.z);
            let t2 = Vector3::new(
                (b.x - ro.x) * rdi.x,
                (b.y - ro.y) * rdi.y,
                (b.z - ro.z) * rdi.z);

            let tmin = Vector3::new(t1.x.min(t2.x), t1.y.min(t2.y), t1.z.min(t2.z));
            let tmax = Vector3::new(t1.x.max(t2.x), t1.y.max(t2.y), t1.z.max(t2.z));

            let ta = tmin.max();
            let tb = tmax.min();

            if ta <= tb {
                return Some(ta);
            }
            None
        }

        // point in box?
        #[inline(always)]
        fn inbox(c: Vector3<F>, tol: F, nbox: F) -> bool {
            let cmin = c.min();
            let cmax = c.max();
            cmin >= -tol && cmax < nbox + tol
        }

        // select whichever basis edge minimizes time to hit
        fn bselect(k: &Vector3<F>, d: &Vector3<F>, b1: Vector3<F>, b2: Vector3<F>) -> Vector3<F> {
            if (k.dot(&b1) * d.dot(&b2)).abs() < (k.dot(&b2) * d.dot(&b1)).abs() {
                b1
            } else {
                b2
            }
        }

        // for stepping through cube lattice
        #[inline(always)]
        fn stepcube(p: &Vector3<F>, d: &Vector3<F>) -> F {

            let mut g = Vector3::new(
                (p.x + 0.5).floor() - 0.5,
                (p.y + 0.5).floor() - 0.5,
                (p.z + 0.5).floor() - 0.5,
            );
            g.x += if d.x < 0.0 { 0.0 } else { 1.0 };
            g.y += if d.y < 0.0 { 0.0 } else { 1.0 };
            g.z += if d.z < 0.0 { 0.0 } else { 1.0 };

            let k = g - p;

            let b = bselect(&k, d,
                bselect(&k, d, Vector3::new(1.0, 0.0, 0.0), Vector3::new(0.0, 1.0, 0.0)),
                Vector3::new(0.0, 0.0, 1.0));

            return k.dot(&b) / d.dot(&b);
        }

        // ray-sphere intersection, used for debugging
        #[inline(always)]
        fn sphere(o: &Vector3<F>, d: &Vector3<F>, ctr: &Vector3<F>, r: F) -> Option<Vector4<F>> {
            let oc = o - ctr;
            let a = d.dot(&d);
            let b = 2.0  * oc.dot(d);
            let c = oc.dot(&oc) - r * r;
            let d_d = b * b - 4.0 * a * c;
            if d_d > 0.0 {
                let sqrtd = d_d.sqrt();
                let t = 0.5 * (-b - sqrtd) / a;
                if t >= 0.0 {
                    //vec3 n = normalize( oc + t*d );
                    //return vec4(n, t);
                    return Some(Vector4::new(0.0, 0.0, 0.0, t));
                }
            }
            None
        }

        // --------

        //let [mut ro, rd] = ray;
        let mut ro = ray[0].clone();
        let rd = ray[1].clone();

        let nbox = 3.0;
        let rsteps = 3 * nbox as i32;

        if let Some(mut curw) = ray_box(&ro, &rd, Vector3::new(0.5 * nbox, 0.5 * nbox, 0.5 * nbox)) {
            let eps = 0.001;

            // center box grid
            ro.x += 0.5*nbox - 0.5;
            ro.y += 0.5*nbox - 0.5;
            ro.z += 0.5*nbox - 0.5;

            // pixel ray distance
            let mut rw = 1000.0;

            for _i in 0..rsteps {
                let p = ro + curw * rd;

                let cbase = Vector3::new(p.x.floor(), p.y.floor(), p.z.floor());

                // for each neighbor in 2x2x2 neighborhood
                for j in 0..8 {
                    // offset to neighbor
                    let joffs = Vector3::new(
                        (j / 4) as F,
                        ((j % 4) / 2) as F,
                        (j % 2) as F);

                    // center of cell
                    let ctr = cbase + joffs;

                    // if center in box
                    if inbox(ctr, 0.0, nbox) {
                        let x = ctr.x.round() as isize;
                        let y = ctr.y.round() as isize;
                        let z = ctr.z.round() as isize;
                        if let Some(index) = self.map.get(&(x, y, z)) {
                            let mut t = curw;
                            match &ctx.nodes[*index].object {
                                Object::SDF3D(object) => {
                                    for _i in 0..24 {
                                        let p = ro + rd * t;
                                        let d = object.get_distance(&p, &ctr);
                                        if d < 0.001 {

                                            return Some( HitRecord {
                                                distance        : t,
                                                node            : *index,
                                                normal          : if get_normal { object.get_normal(&p, &ctr) } else { Vector3::new(0.0, 0.0, 0.0) },
                                                uv              : Vector2::new(0.0, 0.0),
                                                face            : 0
                                            });

                                            rw = t;
                                        }
                                        if t > curw + 1.0 {
                                            break;
                                        }
                                        t += d;
                                    }
                                }
                                _ => {},
                            }
                        }
                        /*
                        if let Some(s) = sphere(&ro, &rd, &ctr, 0.5) {
                            if s.w < rw {
                                rw = s.w;
                            }
                        }*/
                    }
                }
                curw += stepcube(&p, &rd) + eps;
            }

            /*
            if rw < 1000.0 {
                return Some( HitRecord {
                    distance        : 1.0,
                    normal          : Vector3::new(0.0, 0.0, 0.0),
                    uv              : Vector2::new(0.0, 0.0),
                    face            : 0
                });
            }*/
        }

        None
    }

    fn execute(&mut self, code: String) {
        self.engine.execute(code);
    }

    fn set_code_block(&mut self, name: String, code: String) {
        self.engine.set_code_block(name, code);
    }
}