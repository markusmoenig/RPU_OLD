use crate::prelude::*;

pub struct Grid3D<'a> {
        engine              : ScriptEngine<'a>,
        buffer              : IndexBuffer3D,
}

impl Layout3D for Grid3D<'_> {

    fn new() -> Self {

        let engine = ScriptEngine::new();

        Self {
            engine,
            buffer          : IndexBuffer3D::new(),
        }
    }

    fn set_map3d(&mut self, map: HashMap<(i32, i32, i32), usize>) {
        self.buffer.alloc(20, 20, 20);
        for (i, v) in map {
            self.buffer.set(i.0 as usize, i.1 as usize, i.2 as usize, v);
        }
    }

    fn traverse3d(&self, ray: &Ray, get_normal: bool, ctx: &Context) -> Option<HitRecord> {

        fn get_uv(hp: &Vector3<F>, mask: &GF3) -> Vector2<F> {
            let uv : Vector2<F>;
            if mask.x > 0.5 {
                uv = Vector2::new( (hp.z.abs()).fract() - 0.5, (hp.y.abs()).fract() - 0.5);
            } else
            if mask.y > 0.5 {
                uv = Vector2::new( (hp.x.abs()).fract() - 0.5, (hp.z.abs()).fract() - 0.5);
            } else {
                uv = Vector2::new( (hp.x.abs()).fract() - 0.5, (hp.y.abs()).fract() - 0.5);
            }
            uv
        }

        // Based on https://www.shadertoy.com/view/4dX3zl

        let [ro, rd] = &ray;

        let ray_origin = glm::floor(&GF3::new(ro.x, ro.y, ro.z));
        let ray_dir = GF3::new(rd.x, rd.y, rd.z);

	    // ivec3 mapPos = ivec3(floor(rayPos + 0.));
        let mut map_pos = glm::IVec3::new(ray_origin.x as i32, ray_origin.y as i32, ray_origin.z as i32);

        // vec3 deltaDist = abs(vec3(length(rayDir)) / rayDir);
        let lrd = glm::length(&ray_dir);
	    let delta_dist = glm::abs(&GF3::new(lrd / ray_dir.x, lrd / ray_dir.y, lrd / ray_dir.z));

    	// ivec3 rayStep = ivec3(sign(rayDir));
	    let sign_ray_step = glm::sign(&ray_dir);
        let ray_step = glm::IVec3::new(sign_ray_step.x as i32, sign_ray_step.y as i32, sign_ray_step.z as i32);

	    // vec3 sideDist = (sign(rayDir) * (vec3(mapPos) - rayPos) + (sign(rayDir) * 0.5) + 0.5) * deltaDist;
        let mut side_dist = GF3::new(0.0, 0.0, 0.0);
        side_dist.x = (rd.x.signum() * (map_pos.x as F - ro.x) + (rd.x.signum() * 0.5) + 0.5) * delta_dist.x;
        side_dist.y = (rd.y.signum() * (map_pos.y as F - ro.y) + (rd.y.signum() * 0.5) + 0.5) * delta_dist.y;
        side_dist.z = (rd.z.signum() * (map_pos.z as F - ro.z) + (rd.z.signum() * 0.5) + 0.5) * delta_dist.z;

        let mut mask = GF3::new(0.0, 0.0, 0.0);

        for _i in 0..14 {
            if map_pos.x < 0 || map_pos.y < 0 || map_pos.z < 0 { continue; }
            if let Some(index) = self.buffer.get(map_pos.x as usize, map_pos.y as usize, map_pos.z as usize) {

                //float d = length(vec3(mask) * (sideDist - deltaDist)); // rayDir normalized
                let dx = mask.x * (side_dist.x - delta_dist.x);
                let dy = mask.y * (side_dist.y - delta_dist.y);
                let dz = mask.z * (side_dist.z - delta_dist.z);
                let dist = glm::length(&GF3::new(dx, dy, dz));

                match &ctx.nodes[index].object {

                    Object::SDF3D(object) => {
                        let hp = Vector3::new(map_pos.x as F + 0.5, map_pos.y as F + 0.5, map_pos.z as F + 0.5);
                        let mut t = dist;
                        let t_max = dist + 1.73205;
                        for _i in 0..24 {
                            let p = ro + rd * t;
                            let d = object.get_distance(&p, &hp);
                            if d < 0.001 {

                                return Some( HitRecord {
                                    distance        : dist,
                                    node            : index,
                                    hit_point       : p,
                                    mask            : mask,
                                    normal          : if get_normal { object.get_normal(&p, &hp) } else { Vector3::new(0.0, 0.0, 0.0) },
                                    uv              : get_uv(&p, &mask),
                                    face            : 0
                                });
                            }
                            if t > t_max {
                                break;
                            }
                            t += d;
                        }
                    },
                    Object::AnalyticalObject(object) => {

                        if let Some(_d) = object.get_distance(ray) {
                            let hp = ro + rd * dist;

                            return Some( HitRecord {
                                distance        : dist,
                                node            : index,
                                hit_point       : hp,
                                mask            : mask,
                                normal          : Vector3::new(0.0, 0.0, 0.0),
                                uv              : get_uv(&hp, &mask),
                                face            : 0
                            });
                        }
                    }
                    _ => {},
                }
            }

			if side_dist.x < side_dist.y {
				if side_dist.x < side_dist.z {
					side_dist.x += delta_dist.x;
					map_pos.x += ray_step.x;
					mask = GF3::new(1.0, 0.0, 0.0);
				} else {
					side_dist.z += delta_dist.z;
					map_pos.z += ray_step.z;
					mask = GF3::new(0.0, 0.0, 1.0);
				}
			} else {
				if side_dist.y < side_dist.z {
					side_dist.y += delta_dist.y;
					map_pos.y += ray_step.y;
					mask = GF3::new(0.0, 1.0, 0.0);
				} else {
					side_dist.z += delta_dist.z;
					map_pos.z += ray_step.z;
					mask = GF3::new(0.0, 0.0, 1.0);
				}
			}
        }

        None
    }

    fn execute(&mut self, code: String) {
        self.engine.execute(code);
    }

    fn set_code_block(&mut self, name: String, code: String) {
        _ = self.engine.set_code_block(name, code);
    }
}