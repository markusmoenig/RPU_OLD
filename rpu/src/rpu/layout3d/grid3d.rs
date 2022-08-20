use crate::prelude::*;

pub struct Grid3D<'a> {

        map                 : HashMap<(i32, i32, i32), usize>,
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

    fn set_map3d(&mut self, map: HashMap<(i32, i32, i32), usize>) {
        self.map = map;
    }

    fn traverse3d(&self, ray: &Ray, get_normal: bool, ctx: &Context) -> Option<HitRecord> {

        fn get_uv(hp: &Vector3<F>, mask: &glm::BVec3) -> Vector2<F> {
            let uv : Vector2<F>;
            if mask.x {
                uv = Vector2::new( (hp.z.abs()).fract() - 0.5, (hp.y.abs()).fract() - 0.5);
            } else
            if mask.y {
                uv = Vector2::new( (hp.x.abs()).fract() - 0.5, (hp.z.abs()).fract() - 0.5);
            } else {
                uv = Vector2::new( (hp.x.abs()).fract() - 0.5, (hp.y.abs()).fract() - 0.5);
            }
            uv
        }

        // Based on https://www.shadertoy.com/view/4dX3zl

        let [ro, rd] = &ray;

        let ray_origin = glm::floor(&glm::Vec3::new(ro.x, ro.y, ro.z));
        let ray_dir = glm::Vec3::new(rd.x, rd.y, rd.z);

	    // ivec3 mapPos = ivec3(floor(rayPos + 0.));
        let mut map_pos = glm::IVec3::new(ray_origin.x as i32, ray_origin.y as i32, ray_origin.z as i32);

        // vec3 deltaDist = abs(vec3(length(rayDir)) / rayDir);
        let lrd = glm::length(&ray_dir);
	    let delta_dist = glm::abs(&glm::Vec3::new(lrd / ray_dir.x, lrd / ray_dir.y, lrd / ray_dir.z));

    	// ivec3 rayStep = ivec3(sign(rayDir));
	    let sign_ray_step = glm::sign(&ray_dir);
        let ray_step = glm::IVec3::new(sign_ray_step.x as i32, sign_ray_step.y as i32, sign_ray_step.z as i32);

	    // vec3 sideDist = (sign(rayDir) * (vec3(mapPos) - rayPos) + (sign(rayDir) * 0.5) + 0.5) * deltaDist;
        let mut side_dist = glm::Vec3::new(0.0, 0.0, 0.0);
        side_dist.x = (rd.x.signum() * (map_pos.x as f32 - ro.x) + (rd.x.signum() * 0.5) + 0.5) * delta_dist.x;
        side_dist.y = (rd.y.signum() * (map_pos.y as f32 - ro.y) + (rd.y.signum() * 0.5) + 0.5) * delta_dist.y;
        side_dist.z = (rd.z.signum() * (map_pos.z as f32 - ro.z) + (rd.z.signum() * 0.5) + 0.5) * delta_dist.z;

        let mut mask = glm::BVec3::new(false, false, false);

        for _i in 0..14 {

            if let Some(index) = self.map.get(&(map_pos.x, map_pos.y, map_pos.z)) {

                //float d = length(vec3(mask) * (sideDist - deltaDist)); // rayDir normalized
                let dx = mask.x as i32 as f32 * (side_dist.x - delta_dist.x);
                let dy = mask.y as i32 as f32 * (side_dist.y - delta_dist.y);
                let dz = mask.z as i32 as f32 * (side_dist.z - delta_dist.z);
                let dist = glm::length(&glm::Vec3::new(dx, dy, dz));

                match &ctx.nodes[*index].object {

                    Object::SDF3D(object) => {
                        let hp = Vector3::new(map_pos.x as F + 0.5, map_pos.y as F + 0.5, map_pos.z as F + 0.5);
                        let mut t = dist;
                        for _i in 0..24 {
                            let p = ro + rd * t;
                            let d = object.get_distance(&p, &hp);
                            if d < 0.001 {

                                return Some( HitRecord {
                                    distance        : dist,
                                    node            : *index,
                                    hit_point       : p,
                                    mask            : mask,
                                    normal          : if get_normal { object.get_normal(&p, &hp) } else { Vector3::new(0.0, 0.0, 0.0) },
                                    uv              : get_uv(&p, &mask),
                                    face            : 0
                                });
                            }
                            if t > dist + 1.73205 {
                                break;
                            }
                            t += d;
                        }
                    },
                    Object::Voxel => {

                        let hp = ro + rd * dist;

                        return Some( HitRecord {
                            distance        : dist,
                            node            : *index,
                            hit_point       : hp,
                            mask            : mask,
                            normal          : Vector3::new(0.0, 0.0, 0.0),
                            uv              : get_uv(&hp, &mask),
                            face            : 0
                        });
                    }
                    _ => {},
                }
            }

			if side_dist.x < side_dist.y {
				if side_dist.x < side_dist.z {
					side_dist.x += delta_dist.x;
					map_pos.x += ray_step.x;
					mask = glm::BVec3::new(true, false, false);
				} else {
					side_dist.z += delta_dist.z;
					map_pos.z += ray_step.z;
					mask = glm::BVec3::new(false, false, true);
				}
			} else {
				if side_dist.y < side_dist.z {
					side_dist.y += delta_dist.y;
					map_pos.y += ray_step.y;
					mask = glm::BVec3::new(false, true, false);
				} else {
					side_dist.z += delta_dist.z;
					map_pos.z += ray_step.z;
					mask = glm::BVec3::new(false, false, true);
				}
			}
        }

        /*
        if let Some(index) = node_index {

            //float d = length(vec3(mask) * (sideDist - deltaDist)); // rayDir normalized
            let dx = mask.x as i32 as f32 * (side_dist.x - delta_dist.x);
            let dy = mask.y as i32 as f32 * (side_dist.y - delta_dist.y);
            let dz = mask.z as i32 as f32 * (side_dist.z - delta_dist.z);
            let dist = glm::length(&glm::Vec3::new(dx, dy, dz));


            // if mask.x || mask.y || mask.z {
            //     return Some( HitRecord {
            //         distance        : 1.0,
            //         node            : index,
            //         normal          : Vector3::new(1.0, 1.0, 1.0),
            //         uv              : Vector2::new(0.0, 0.0),
            //         face            : 0
            //     });
            // }
            let mut t = dist;
            match &ctx.nodes[index].object {

                Object::SDF3D(object) => {
                    let hp = Vector3::new(map_pos.x as F + 0.5, map_pos.y as F + 0.5, map_pos.z as F + 0.0);
                    for _i in 0..24 {
                        let p = ro + rd * t;
                        let d = object.get_distance(&p, &hp);
                        if d < 0.001 {

                            return Some( HitRecord {
                                distance        : dist,
                                node            : index,
                                normal          : if get_normal { object.get_normal(&p, &hp) } else { Vector3::new(0.0, 0.0, 0.0) },
                                uv              : Vector2::new(0.0, 0.0),
                                face            : 0
                            });
                        }
                        if t > t + 1.0 {
                            //break;
                        }
                        t += d;
                    }
                }
                _ => {},
            }
        }*/

        None
    }

    fn execute(&mut self, code: String) {
        self.engine.execute(code);
    }

    fn set_code_block(&mut self, name: String, code: String) {
        self.engine.set_code_block(name, code);
    }
}