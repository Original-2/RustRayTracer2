use std::cmp;

use image::{ImageBuffer, Rgb};

use crate::Collision;
use crate::Color;
use crate::light::Light;
use crate::Material;
use crate::objects::Object;
use crate::Scene;
use crate::Vector3;

pub struct RayTracer {
    pub m_scene: Scene,
}

const MIN_WEIGHT: f64 = 0.05;
const MAX_DEPTH: i64 = 10;
const SPEC_POWER: i64 = 50;

impl RayTracer {

    fn m_calc_local_illumination(&self, coll: &Collision, material: &Material) -> Color {
        let r = coll.ray_dir.reflect(&coll.n);
        let mut color = material.color * coll.plane.get_texture_color(&coll.p);

        if coll.at_plane {
            color = material.color * coll.plane.get_texture_color(&coll.p);
        }

        if coll.at_sphere {
            color = material.color * coll.sphere.get_texture_color(&coll.p);
        }

        let mut ret = color * self.m_scene.get_ambient_light_color() * material.diff;

        for light in &self.m_scene.m_p_lights {
            let shade = light.get_shadow_ratio(&self.m_scene, &coll.p);
            if shade < 0.00001 {
                continue;
            }
            let l = (light.get_source() - coll.p).unitise();
            if material.diff > 0.00001 {
                ret += color * light.get_color() * (material.diff * l.dot(&coll.n) * shade);
            }
            if material.spec > 0.00001 {
                ret += color * light.get_color() * (material.spec * l.dot(&r).powi(SPEC_POWER as i32));
            }
        }

        for light in &self.m_scene.m_r_lights {
            let shade = light.get_shadow_ratio(&self.m_scene, &coll.p);
            if shade < 0.00001 {
                continue;
            }
            let l = (light.get_source() - coll.p).unitise();
            if material.diff > 0.00001 {
                ret += color * light.get_color() * (material.diff * l.dot(&coll.n) * shade);
            }
            if material.spec > 0.00001 {
                ret += color * light.get_color() * (material.spec * l.dot(&r).powi(SPEC_POWER as i32));
            }
        }

        return ret;
    }

    pub fn run(&self, filename: String, h_from: i32, h: i32, w_from: i32, w: i32) {
        let camera = self.m_scene.get_camera();

        let mut image = ImageBuffer::<Rgb<u8>, Vec<u8>>::new(w as u32, h as u32);

        for i in w_from..(w_from + w) {
            //println!("{}", i);
            for j in h_from..(h_from + h) {
                let dir = camera.emit(i, j);
                let color = self.m_ray_traceing(camera.get_eye(), dir, 1.0, 1);

                let r = cmp::min((color.r * 255.0).round() as u8, 255);
                let g = cmp::min((color.g * 255.0).round() as u8, 255);
                let b = cmp::min((color.b * 255.0).round() as u8, 255);
                let pix = [r, g, b];

                image.put_pixel((i - w_from) as u32, (j - h_from) as u32, Rgb(pix));
            }
        }

        image.save(filename.as_str()).expect(format!("Could not save file {}", filename.as_str()).as_str());
    }

    fn m_ray_traceing(&self, start: Vector3, dir: Vector3, weight: f64, depth: i64) -> Color {
        if weight < MIN_WEIGHT {
            return self.m_scene.get_ambient_light_color();
        }

        let coll = self.m_scene.find_nearest_collision(start, dir);
        return if !coll.is_hit() {
            self.m_scene.get_ambient_light_color()
        } else if coll.at_light() {
            if coll.at_point_light {
                return coll.point_light.get_color();
            }
            coll.rect_light.get_color()
        } else if depth <= MAX_DEPTH {
            let mut ret = Color { r: 0.0, g: 0.0, b: 0.0 };
            if coll.at_sphere {
                let obj = coll.sphere;
                if obj.get_material().diff > 0.00001 || obj.get_material().spec > 0.00001 {
                    ret += self.m_calc_local_illumination(&coll, obj.get_material());
                }
                if obj.get_material().reflection > 0.00001 {
                    ret += self.m_calc_reflection(&coll, obj.get_material(), weight, depth);
                }
                if obj.get_material().refraction > 0.00001 {
                    ret += self.m_calc_refraction(&coll, obj.get_material(), weight, depth);
                }
            }

            if coll.at_plane {
                let obj = coll.plane;
                if obj.get_material().diff > 0.00001 || obj.get_material().spec > 0.00001 {
                    ret += self.m_calc_local_illumination(&coll, obj.get_material());
                }
                if obj.get_material().reflection > 0.00001 {
                    ret += self.m_calc_reflection(&coll, obj.get_material(), weight, depth);
                }
                if obj.get_material().refraction > 0.00001 {
                    ret += self.m_calc_refraction(&coll, obj.get_material(), weight, depth);
                }
            }
            ret.clamp()
        } else {
            self.m_scene.get_ambient_light_color()
        }
    }

    fn m_calc_refraction(&self, coll: &Collision, material: &Material, weight: f64, depth: i64) -> Color {
        let mut rindex = material.r_index;
        if coll.is_internal {
            rindex = 1.0 / rindex;
        }
        let r = coll.ray_dir.refract(&coll.n, rindex);

        if r.mod2() < 0.00001 {
            return Color { r: 0.0, g: 0.0, b: 0.0 };
        }

        let ret = self.m_ray_traceing(coll.p, r, weight * material.refraction, depth + 1);

        return if !coll.is_internal {
            ret * material.refraction
        } else {
            ret * (material.absorb_color * -coll.dist).exp() * material.refraction
        }
    }

    fn m_calc_reflection(&self, coll: &Collision, material: &Material, weight: f64, depth: i64) -> Color {
        let r = coll.ray_dir.reflect(&coll.n);
        let ret = self.m_ray_traceing(coll.p, r, weight * material.reflection, depth + 1);
        return ret * (material.color * material.reflection);
    }
}
