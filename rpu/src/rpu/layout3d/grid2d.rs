use crate::prelude::*;

pub struct Grid2D<'a> {

        map                 : HashMap<(isize, isize), usize>,
        engine              : ScriptEngine<'a>,
}

impl Layout3D for Grid2D<'_> {

    fn new() -> Self {

        let mut engine = ScriptEngine::new();

        Self {
            map             : HashMap::new(),
            engine,
        }
    }

    fn set_map2d(&mut self, map: HashMap<(isize, isize), usize>) {
        self.map = map;
    }


    fn traverse3d(&self, ray: &Ray, ctx: &Context) -> Option<HitRecord> {

        let [ro, rd] = ray;

	    let mut pos = Vector2::new(ro.x.floor(), ro.z.floor());
        let rdi = Vector3::new(1.0 / rd.x, 1.0 / rd.y, 1.0 / rd.z);
        let rda = rdi.abs();
	    let rds = Vector2::new(rd.x.signum(), rd.z.signum());
	    let mut dis = Vector2::new(
            (pos.x - ro.x + 0.5 + rds.x * 0.5) * rdi.x,
            (pos.y - ro.z + 0.5 + rds.y * 0.5) * rdi.z);

        let h = 0.1;

	    let mut mm = Vector2::new(0.0, 0.0);
        for _i in 0..24 {

            // intersect box
            let ce = Vector3::new( pos.x+0.5, 0.5*h, pos.y+0.5 );
            let rb = Vector3::new(0.3,h*0.5,0.3);
            let ra = Vector3::new(rb.x + 0.12, rb.y + 0.12, rb.z + 0.12);
            let rc = ro - ce;
            let t_n = Vector3::new(
                -rdi.x*rc.x - rda.x*ra.x,
                -rdi.y*rc.y - rda.y*ra.y,
                -rdi.z*rc.z - rda.z*ra.z).max();
            let t_f = Vector3::new(
                -rdi.x*rc.x + rda.x*ra.x,
                -rdi.y*rc.y + rda.y*ra.y,
                -rdi.z*rc.z + rda.z*ra.z).max();

            if t_n < t_f {//&& tF > 0.0 )

                let p = ro + rd * t_n;
                //println!("{}, {}", p.x, p.z);
                let x = p.x.round() as isize;
                let y = p.y.round() as isize;

                if self.map.contains_key(&(x, y)) {
                    return Some( HitRecord {
                        distance        : t_n,
                        normal          : Vector3::new(0.0, 0.0, 0.0),
                        uv              : Vector2::new(0.0, 0.0),
                        face            : 0
                    });
                }
            }

            // step to next cell

            // mm = step( dis.xy, dis.yx );
            mm.x = if dis.y < dis.x { 0.0 } else { 1.0 };
            mm.y = if dis.x < dis.y { 0.0 } else { 1.0 };
            //dis += mm*rda.xz;
            dis.x += mm.x * rda.x;
            dis.y += mm.y * rda.z;
            //pos += mm*rds;
            pos.x += mm.x * rds.x;
            pos.y += mm.y * rds.y;
        }

        //println!("here");
        None
    }

    fn execute(&mut self, code: String) {
        self.engine.execute(code);
    }

    fn set_code_block(&mut self, name: String, code: String) {
        self.engine.set_code_block(name, code);
    }
}