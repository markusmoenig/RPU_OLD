use crate::prelude::*;

#[derive()]
pub struct AnalyticalCube<'a> {
        txx                 : SMatrix::<F, 4, 4>,
        txi                 : SMatrix::<F, 4, 4>,

        engine              : ScriptEngine<'a>,
}

impl Analytical for AnalyticalCube<'_> {

    fn new() -> Self {

        let mut engine = ScriptEngine::new();
        engine.set_vector3("position", Vector3::new(0.0, 0.0, 0.0));
        engine.set_vector3("rotation", Vector3::new(0.0, 0.0, 0.0));
        engine.set_vector3("size", Vector3::new(0.75, 0.75, 0.75));
        engine.set_float("scale", 1.0);

        Self {
            txx             : SMatrix::<F, 4, 4>::identity(),
            txi             : SMatrix::<F, 4, 4>::identity(),

            engine          : engine,
        }
    }

    fn get_bounds(&self) -> (Vector3<F>, Vector3<F>) {
        let p = self.engine.get_vector3("position").unwrap();
        let radius = self.engine.get_vector3("size").unwrap();

        let position = Vector3::new(p.x, p.y, p.z);

        let half_size = Vector3::new(radius.x / 2.0, radius.y / 2.0, radius.z / 2.0);
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

    fn update(&mut self) {
        if self.engine.execute_block("update".to_string()) {

            let r = self.engine.get_vector3("rotation").unwrap();
            let p = self.engine.get_vector3("position").unwrap();
            let s = self.engine.get_float("scale").unwrap();

            let mut txx =  Matrix4::new_rotation(Vector3::new(r.x.to_radians(), r.y.to_radians(), r.z.to_radians()));
            txx = txx.append_scaling(s);
            txx = txx.append_translation(&p);
            self.txi = txx;
            txx = txx.try_inverse().unwrap();
            self.txx = txx;
        }
    }

    /// https://iquilezles.org/articles/boxfunctions
    fn get_distance_normal_uv_face(&self, ray: &[Vector3<F>; 2]) -> Option<HitRecord> {
        let [ro, rd] = ray;

        let txx = &self.txx;
        let rdd = (txx.clone() * Vector4::new(rd.x, rd.y, rd.z, 0.0)).xyz();
        let roo = (txx.clone() * Vector4::new(ro.x, ro.y, ro.z, 1.0)).xyz();

        let rad = self.engine.get_vector3("size").unwrap();

        let m: Vector3::<F> = Vector3::new(1.0 / rdd.x, 1.0 / rdd.y, 1.0 / rdd.z);
        let s: Vector3::<F> = Vector3::new(
            if rdd.x < 0.0 { 1.0 } else { - 1.0 },
            if rdd.y < 0.0 { 1.0 } else { - 1.0 },
            if rdd.z < 0.0 { 1.0 } else { - 1.0 });
        let t1 = Vector3::new(
            m.x * (-roo.x + s.x * rad.x),
            m.y * (-roo.y + s.y * rad.y),
            m.z * (-roo.z + s.z * rad.z));

        let t2 = Vector3::new(
            m.x * (-roo.x - s.x * rad.x),
            m.y * (-roo.y - s.y * rad.y),
            m.z * (-roo.z - s.z * rad.z));

        let t_n = ((t1.x).max(t1.y)).max(t1.z);
        let t_f = ((t2.x).min(t2.y)).min(t2.z);

        if t_n > t_f || t_f < 0.0 { return None };

        /*
    if( t1.x>t1.y && t1.x>t1.z ) { oN=txi[0].xyz*s.x; oU=ro.yz+rd.yz*t1.x; oF=(1+int(s.x))/2;
    else if( t1.y>t1.z   )       { oN=txi[1].xyz*s.y; oU=ro.zx+rd.zx*t1.y; oF=(5+int(s.y))/2;
    else                         { oN=txi[2].xyz*s.z; oU=ro.xy+rd.xy*t1.z; oF=(9+int(s.z))/2;

    oT = vec2(tN,tF);*/

        let normal = Vector3::new(0.0, 0.0, 0.0);
        let mut uv : Vector2::<F>;

        if t1.x > t1.y && t1.x > t1.z {
            uv = Vector2::new(
                roo.y + rdd.y * t1.x,
                roo.z + rdd.z * t1.x
            );
        } else
        if t1.y > t1.z {
            uv = Vector2::new(
                roo.z + rdd.z * t1.y,
                roo.x + rdd.x * t1.y
            );
        } else {
            uv = Vector2::new(
                roo.x + rdd.x * t1.z,
                roo.y + rdd.y * t1.z
            );
        }

        uv /= 0.75;
        uv /= 2.0;

        /*
        let n = Vector3::new(m.x * roo.x, m.y * roo.y, m.z * roo.z);
        let ma = m.abs();
        let k = Vector3::new(ma.x * rad.x, ma.y * rad.y, ma.z * rad.z);

        let t1 = -n - k;
        let t2 = -n + k;

        let t_n = ((t1.x).max(t1.y)).max(t1.z);
        let t_f = ((t2.x).min(t2.y)).min(t2.z);

        if t_n > t_f || t_f < 0.0 { return None; }*/

        Some( HitRecord {
            distance        : t_n,
            normal          : normal,
            uv,
            face            : 0
        })
        /*
bool boxIntersect( in vec3 row, in vec3 rdw, in mat4 txx, in mat4 txi, in vec3 rad,
                   out vec2 oT, out vec3 oN, out vec2 oU, out int oF )
{
    // convert from world to box space
    vec3 rd = (txx*vec4(rdw,0.0)).xyz;
    vec3 ro = (txx*vec4(row,1.0)).xyz;


    // ray-box intersection in box space
    vec3 m = 1.0/rd;
    vec3 s = vec3((rd.x<0.0)?1.0:-1.0,
                  (rd.y<0.0)?1.0:-1.0,
                  (rd.z<0.0)?1.0:-1.0);
    vec3 t1 = m*(-ro + s*rad);
    vec3 t2 = m*(-ro - s*rad);

    float tN = max( max( t1.x, t1.y ), t1.z );
    float tF = min( min( t2.x, t2.y ), t2.z );

    if( tN>tF || tF<0.0) return false;

    // compute normal (in world space), face and UV
    if( t1.x>t1.y && t1.x>t1.z ) { oN=txi[0].xyz*s.x; oU=ro.yz+rd.yz*t1.x; oF=(1+int(s.x))/2;
    else if( t1.y>t1.z   )       { oN=txi[1].xyz*s.y; oU=ro.zx+rd.zx*t1.y; oF=(5+int(s.y))/2;
    else                         { oN=txi[2].xyz*s.z; oU=ro.xy+rd.xy*t1.z; oF=(9+int(s.z))/2;

    oT = vec2(tN,tF);
}
        */
    }
}
