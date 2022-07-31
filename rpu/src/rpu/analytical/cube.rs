use crate::prelude::*;

pub struct AnalyticalCube {

}

impl Analytical for AnalyticalCube {

    fn new() -> Self {
        Self {

        }
    }

    /// https://www.shadertoy.com/view/ld23DV
    fn get_distance_and_normal(&self, ray: &[Vector3<F>; 2]) -> Option<(F, Vector3<F>)> {
        let [ro, rd] = ray;

        let txx = SMatrix::<F, 4, 4>::new(1.0, 0.0, 0.0, 0.0,
                 0.0, 1.0, 0.0, 0.0,
                 0.0, 0.0, 1.0, 0.0,
                 0.0, 0.0, 0.0, 1.0);

        let rdd = (txx.clone() * Vector4::new(rd.x, rd.y, rd.z, 0.0)).xyz();
        let roo = (txx.clone() * Vector4::new(ro.x, ro.y, ro.z, 1.0)).xyz();

        let rad = Vector3::new(0.75, 0.75, 0.75);

        let m: Vector3::<F> = Vector3::new(1.0 / rdd.x, 1.0 / rdd.y, 1.0 / rdd.z);
        let n = Vector3::new(m.x * roo.x, m.y * roo.y, m.z * roo.z);
        let ma = m.abs();
        let k = Vector3::new(ma.x * rad.x, ma.y * rad.y, ma.z * rad.z);

        let t1 = -n - k;
        let t2 = -n + k;

        let t_n = ((t1.x).max(t1.y)).max(t1.z);
        let t_f = ((t2.x).min(t2.y)).min(t2.z);

        if t_n > t_f || t_f < 0.0 { return None; }

        let dist = 1.0;

        Some((dist, Vector3::new(0.0, 0.0, 0.0)))
        /*
vec4 iBox( in vec3 ro, in vec3 rd, in mat4 txx, in mat4 txi, in vec3 rad )
{
    // convert from ray to box space
	vec3 rdd = (txx*vec4(rd,0.0)).xyz;
	vec3 roo = (txx*vec4(ro,1.0)).xyz;

	// ray-box intersection in box space
    vec3 m = 1.0/rdd;
    vec3 n = m*roo;
    vec3 k = abs(m)*rad;

    vec3 t1 = -n - k;
    vec3 t2 = -n + k;

	float tN = max( max( t1.x, t1.y ), t1.z );
	float tF = min( min( t2.x, t2.y ), t2.z );

	if( tN > tF || tF < 0.0) return vec4(-1.0);

	vec3 nor = -sign(rdd)*step(t1.yzx,t1.xyz)*step(t1.zxy,t1.xyz);

    // convert to ray space

	nor = (txi * vec4(nor,0.0)).xyz;

	return vec4( tN, nor );
}
        */
    }
}