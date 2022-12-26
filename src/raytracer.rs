use crate::Scene;
use crate::light::Light;
use crate::Material;
use crate::Color;
use crate::Collision;
use crate::objects::Object;
use crate::Vector3;

use image::{ImageBuffer, Rgb};
use std::cmp;

pub struct RayTracer {
    pub m_scene: Scene
}


const MIN_WEIGHT: f64 = 0.05;
const MAX_DEPTH: i64 = 10;
const SPEC_POWER: i64 = 50;

impl RayTracer {
    fn m_calcLocalIllumination(&self, coll: Collision, material: Material) -> Color {
        let r = coll.ray_dir.reflect(coll.n);
        let mut color = material.color * coll.plane.getTextureColor(coll.p);

        if coll.at_sphere {
            color = material.color * coll.sphere.getTextureColor(coll.p);
        }

        let mut ret = color * self.m_scene.getAmbientLightColor() * material.diff;


        for light in self.m_scene.clone().m_plights {
            let shade = light.getShadowRatio(self.m_scene.clone(), coll.p);
            if (shade < 0.00001) {
                continue;
            }
            let l = (light.getSource() - coll.p).unitise();
            if material.diff > 0.00001 {
                ret += color * light.getColor() * (material.diff * l.dot(coll.n) * shade);
            }
            if material.spec > 0.00001{
                ret += color * light.getColor() * (material.spec * l.dot(r).powi(SPEC_POWER as i32));
            }
        }

        for light in self.m_scene.clone().m_rlights {
            let shade = light.getShadowRatio(self.m_scene.clone(), coll.p);
            if (shade < 0.00001) {
                continue;
            }
            let l = (light.getSource() - coll.p).unitise();
            if material.diff > 0.00001 {
                ret += color * light.getColor() * (material.diff * l.dot(coll.n) * shade);
            }
            if material.spec > 0.00001{
                ret += color * light.getColor() * (material.spec * l.dot(r).powi(SPEC_POWER as i32));
            }
        }


        return ret;
    }

    pub fn run(&self){
    let mut camera = self.m_scene.getCamera();
    let w = camera.getW();
    let h = camera.getH();

    let mut image = ImageBuffer::<Rgb<u8>, Vec<u8>>::new(w as u32, h as u32);

    for i in 0..w {
        println!("{}", i);
        for j in 0..h{
            let dir = camera.emit(i, j);
            let color = self.m_rayTraceing(camera.getEye(), dir, 1.0, 1);

            let r = cmp::min((color.r*255.0).round() as u8, 255);
            let g = cmp::min((color.g*255.0).round() as u8, 255);
            let b = cmp::min((color.b*255.0).round() as u8, 255);
            //println!("{} {}", i, j);
            println!("{} {} {}", r, g, b);

            let pix = [r, g, b];

            image.put_pixel(i as u32, j as u32, Rgb(pix));
        }
    }


        image.save("output1.png");




    }

    fn m_rayTraceing(&self, start: Vector3, dir: Vector3, weight: f64, depth: i64) -> Color{
        if weight < MIN_WEIGHT{
            return self.m_scene.getAmbientLightColor();
        }

        let coll = self.clone().m_scene.findNearestCollision(start, dir);
        if !coll.isHit() {
            return self.m_scene.getAmbientLightColor();
        }else if coll.atLight() {
            if coll.at_plight {
                return coll.plight.getColor();
            }
            return coll.rlight.getColor();
        }else if depth <= MAX_DEPTH {
            let mut ret = Color {r: 0.0, g: 0.0, b: 0.0};
            if coll.at_sphere {
                let obj = coll.clone().sphere;
                if obj.GetMaterial().diff > 0.00001 || obj.GetMaterial().spec > 0.00001 {
                    ret += self.m_calcLocalIllumination(coll.clone(), obj.GetMaterial());
                }
                if obj.GetMaterial().refl > 0.00001 {
                    ret += self.m_calcReflection(coll.clone(), obj.GetMaterial(), weight, depth);
                }
                if obj.GetMaterial().refr > 0.00001{
                    ret += self.m_calcRefraction(coll.clone(), obj.GetMaterial(), weight, depth);
                }
            }

            if coll.at_plane {
                let obj = coll.clone().plane;
                if obj.GetMaterial().diff > 0.00001 || obj.GetMaterial().spec > 0.00001 {
                    ret += self.m_calcLocalIllumination(coll.clone(), obj.GetMaterial());
                }
                if obj.GetMaterial().refl > 0.00001 {
                    ret += self.m_calcReflection(coll.clone(), obj.GetMaterial(), weight, depth);
                }
                if obj.GetMaterial().refr > 0.00001{
                    ret += self.m_calcRefraction(coll.clone(), obj.GetMaterial(), weight, depth);
                }
            }
            return ret.Clamp();
        }
        else{
            return self.m_scene.getAmbientLightColor();
        }
    }

    fn m_calcRefraction(&self, coll: Collision, material: Material, weight: f64, depth: i64) -> Color {
        let mut rindex = material.rindex;
        if coll.is_internal {
            rindex = 1.0 / rindex;
        }
        let r = coll.ray_dir.refract(coll.n, rindex);

        if (r.mod2() < 0.00001) {
            return Color{r: 0.0, g: 0.0, b: 0.0};
        }


        let ret = self.m_rayTraceing(coll.p, r, weight * material.refr, depth + 1);

        if !coll.is_internal{
            return ret * material.refr;
        }else{
            return ret * (material.absorb_color * -coll.dist).exp() * material.refr;
        }
    }

    fn m_calcReflection(&self, coll: Collision, material: Material, weight: f64, depth: i64) -> Color {
        let r = coll.ray_dir.reflect(coll.n);
        let ret = self.m_rayTraceing(coll.p, r, weight * material.refl, depth + 1);
        return ret * (material.color * material.refl);
    }
}
