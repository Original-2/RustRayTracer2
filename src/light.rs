use crate::vector3::Vector3;
use crate::color::Color;
use crate::collision::Collision_plight;
use crate::collision::Collision_rlight;
use crate::collision::Collision;
use crate::Scene;
use crate::objects::Object;


const SHADOW_SAMPLE: i32 = 4;

pub trait Light {
    fn getColor (&self) -> Color;
    fn getSource (&self) -> Vector3;
    fn collision (&self, start: Vector3, dir: Vector3) -> Collision;
    fn getShadowRatio (&self, scene: Scene, p: Vector3) -> f64;
}


#[derive(Clone)]
pub struct PointLight {
    pub m_color: Color,
    pub m_o: Vector3,
}

pub fn MakePointLight (c: Color, o: Vector3) -> PointLight {
    PointLight {
        m_color: c,
        m_o: o,
    }
}

impl Light for PointLight {
    fn getColor(&self) -> Color {
        self.m_color
    }

    fn getSource(&self) -> Vector3 {
        self.m_o
    }

    fn collision(&self, start: Vector3, dir: Vector3) -> Collision {
        return Collision_plight(start, dir, 10000000.0, self.clone()); // cannot intersect line of sight
    }


    fn getShadowRatio(&self, scene: Scene, p: Vector3) -> f64 {
        let dir = self.m_o - p;
        let dist = dir.mod1();

        for o in scene.m_planes.iter() {
            let coll = o.collision(p, dir);
            if coll.clone().isHit() && coll.dist + 0.00001 < dist{
                return 0.0;
            }
        }

        for o in scene.m_spheres.iter() {
            let coll = o.collision(p, dir);
            if coll.clone().isHit() && coll.dist + 0.00001 < dist{
                return 0.0;
            }
        }

        return 1.0;
    }
}

#[derive(Clone)]
pub struct RectLight {
    pub m_color: Color,
    pub m_o: Vector3,
    pub m_dx: Vector3,
    pub m_dy: Vector3,
}

pub fn makeRectLight(c: Color, o: Vector3, dx: Vector3, dy: Vector3) -> RectLight {
        RectLight {
            m_color: c,
            m_o: o,
            m_dx: dx,
            m_dy: dy,}
}

impl Light for RectLight {
    fn getColor(&self) -> Color {
        self.m_color
    }

    fn getSource(&self) -> Vector3 {
        self.m_o
    }


    fn collision(&self, start: Vector3, dir: Vector3) -> Collision {
        let n = self.m_dx * self.m_dy;
        let d = n.dot(dir);
        if d.abs() < 0.00001 {
            return Collision_rlight(start, dir, 10000000.0, self.clone());
        }
        let t = (n.dot(self.m_o) - n.dot(start)) / d;
        if t < 0.00001 {
            return Collision_rlight(start, dir, 10000000.0, self.clone());
        }
        let p = start + dir * t - self.m_o;
        if p.dot(self.m_dx).abs() + 0.00001 < self.m_dx.mod2() && p.dot(self.m_dy).abs() + 0.00001 < self.m_dy.mod2() {
            return Collision_rlight(start, dir, t, self.clone());
        }else{
            return Collision_rlight(start, dir, 10000000.0, self.clone());
        }
    }




    fn getShadowRatio(&self, scene: Scene, p: Vector3) -> f64 {
        let mut ret = SHADOW_SAMPLE * SHADOW_SAMPLE;
        for i in 0..SHADOW_SAMPLE {
            for j in 0..SHADOW_SAMPLE {
                let x = ((i as f64) + 0.5) * 2.0 / (SHADOW_SAMPLE as f64) - 1.0;
                let y = ((j as f64)  + 0.5) * 2.0 / (SHADOW_SAMPLE as f64) - 1.0;
                let c = self.m_o + self.m_dx * x + self.m_dy * y;
                let dir = c - p;
                let dist = dir.mod1();
                for o in scene.m_planes.iter() {
                    let coll = o.collision(p, dir);
                    if coll.clone().isHit() && coll.dist + 0.00001 < dist {
                        ret-=1;
                        break;
                    }
                }

                for o in scene.m_spheres.iter() {
                    let coll = o.collision(p, dir);
                    if coll.clone().isHit() && coll.dist + 0.00001 < dist {
                        ret-=1;
                        break;
                    }
                }

            }
        }
    return (1 * ret / SHADOW_SAMPLE / SHADOW_SAMPLE) as f64;
    }
}
