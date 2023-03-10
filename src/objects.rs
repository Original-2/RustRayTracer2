use crate::collision::Collision;
use crate::collision::collision_plane;
use crate::collision::collision_sphere;
use crate::color::Color;
use crate::light::PointLight;
use crate::material::Material;
use crate::RectLight;
use crate::vector3::Vector3;

pub trait Object {
    fn get_material(&self) -> &Material;
    fn collision(&self, start: &Vector3, dir: &Vector3) -> Collision;
    fn get_texture_color(&self, p: &Vector3) -> Color;
}

#[derive(Clone)]
pub struct Plane {
    pub m_material: Material,
    pub m_d: f64,
    pub m_n: Vector3,
    pub m_o: Vector3,
    pub m_dx: Vector3,
    pub m_dy: Vector3,
}

pub fn make_plane(m: &Material, n: Vector3, d: f64) -> Plane {

    let mut m_dx = n.unitise() * Vector3 { x: 0.0, y: 0.0, z: -1.0 };

    if m_dx.mod2() < 0.00001 {
        m_dx = Vector3 { x: 1.0, y: 0.0, z: 0.0 };
    } else {
        m_dx = m_dx.unitise();
    }

    let m_dy = (m_dx * n.unitise()).unitise();

    return Plane {
        m_material: m.clone(),
        m_d: d,
        m_n: n.unitise(),
        m_o: Vector3 { x: 0.0, y: 0.0, z: 0.0 },
        m_dx,
        m_dy,
    };
}


impl Object for Plane {

    fn get_material(&self) -> &Material {
        &self.m_material
    }

    fn collision(&self, start: &Vector3, dir: &Vector3) -> Collision {
        let n = self.m_n.dot(start) + self.m_d;
        let d = self.m_n.dot(dir);

        if d.abs() < 0.00001 {
            return Collision {
                at_object: false,
                at_sphere: false,
                sphere: &Sphere {
                    m_material: Material {
                        color: Color { r: 0.3, g: 0.3, b: 1.0 },
                        absorb_color: Color { r: 1.0, g: 1.0, b: 0.0 },
                        diff: 0.0,
                        spec: 0.8,
                        reflection: 0.4,
                        refraction: 0.6,
                        r_index: 1.5,
                        m_texture: None,
                        use_m_texture: false,
                    },
                    m_r: 0.0,
                    m_o: Vector3 { x: 0.0, y: 0.0, z: 0.0 },
                    m_dz: Vector3 { x: 0.0, y: 0.0, z: 0.0 },
                    m_dx: Vector3 { x: 0.0, y: 0.0, z: 0.0 },
                },
                at_plane: false,
                plane: &Plane {
                    m_material: Material {
                        color: Color { r: 0.3, g: 0.3, b: 1.0 },
                        absorb_color: Color { r: 1.0, g: 1.0, b: 0.0 },
                        diff: 0.0,
                        spec: 0.8,
                        reflection: 0.4,
                        refraction: 0.6,
                        r_index: 1.5,
                        m_texture: None,
                        use_m_texture: false,
                    },
                    m_d: 0.0,
                    m_n: Vector3 { x: 0.0, y: 0.0, z: 0.0 },
                    m_o: Vector3 { x: 0.0, y: 0.0, z: 0.0 },
                    m_dx: Vector3 { x: 0.0, y: 0.0, z: 0.0 },
                    m_dy: Vector3 { x: 0.0, y: 0.0, z: 0.0 },
                },
                at_light: false,
                at_rect_light: false,
                rect_light: &RectLight {
                    m_color: Color { r: 0.0, g: 0.0, b: 0.0 },
                    m_o: Vector3 { x: 0.0, y: 0.0, z: 0.0 },
                    m_dx: Vector3 { x: 0.0, y: 0.0, z: 0.0 },
                    m_dy: Vector3 { x: 0.0, y: 0.0, z: 0.0 },
                },
                at_point_light: false,
                point_light: &PointLight {
                    m_color: Color { r: 0.0, g: 0.0, b: 0.0 },
                    m_o: Vector3 { x: 0.0, y: 0.0, z: 0.0 },
                },
                ray_start: Vector3 { x: 0.0, y: 0.0, z: 0.0 },
                ray_dir: Vector3 { x: 0.0, y: 0.0, z: 0.0 },
                p: Vector3 { x: 0.0, y: 0.0, z: 0.0 },
                n: Vector3 { x: 0.0, y: 0.0, z: 0.0 },
                dist: 0.0,
                is_internal: false,
            };
        }

        let t = -n / d;

        if t < 0.0 {
            return Collision {
                at_object: false,
                at_sphere: false,
                sphere: &Sphere {
                    m_material: Material {
                        color: Color { r: 0.3, g: 0.3, b: 1.0 },
                        absorb_color: Color { r: 1.0, g: 1.0, b: 0.0 },
                        diff: 0.0,
                        spec: 0.8,
                        reflection: 0.4,
                        refraction: 0.6,
                        r_index: 1.5,
                        m_texture: None,
                        use_m_texture: false,
                    },
                    m_r: 0.0,
                    m_o: Vector3 { x: 0.0, y: 0.0, z: 0.0 },
                    m_dz: Vector3 { x: 0.0, y: 0.0, z: 0.0 },
                    m_dx: Vector3 { x: 0.0, y: 0.0, z: 0.0 },
                },
                at_plane: false,
                plane: &Plane {
                    m_material: Material {
                        color: Color { r: 0.3, g: 0.3, b: 1.0 },
                        absorb_color: Color { r: 1.0, g: 1.0, b: 0.0 },
                        diff: 0.0,
                        spec: 0.8,
                        reflection: 0.4,
                        refraction: 0.6,
                        r_index: 1.5,
                        m_texture: None,
                        use_m_texture: false,
                    },
                    m_d: 0.0,
                    m_n: Vector3 { x: 0.0, y: 0.0, z: 0.0 },
                    m_o: Vector3 { x: 0.0, y: 0.0, z: 0.0 },
                    m_dx: Vector3 { x: 0.0, y: 0.0, z: 0.0 },
                    m_dy: Vector3 { x: 0.0, y: 0.0, z: 0.0 },
                },
                at_light: false,
                at_rect_light: false,
                rect_light: &RectLight {
                    m_color: Color { r: 0.0, g: 0.0, b: 0.0 },
                    m_o: Vector3 { x: 0.0, y: 0.0, z: 0.0 },
                    m_dx: Vector3 { x: 0.0, y: 0.0, z: 0.0 },
                    m_dy: Vector3 { x: 0.0, y: 0.0, z: 0.0 },
                },
                at_point_light: false,
                point_light: &PointLight {
                    m_color: Color { r: 0.0, g: 0.0, b: 0.0 },
                    m_o: Vector3 { x: 0.0, y: 0.0, z: 0.0 },
                },
                ray_start: Vector3 { x: 0.0, y: 0.0, z: 0.0 },
                ray_dir: Vector3 { x: 0.0, y: 0.0, z: 0.0 },
                p: Vector3 { x: 0.0, y: 0.0, z: 0.0 },
                n: Vector3 { x: 0.0, y: 0.0, z: 0.0 },
                dist: 0.0,
                is_internal: false,
            };
        };
        if n > 0.00001 {
            return collision_plane(start, dir, t, &self.m_n, false, self);
        } else {
            return collision_plane(start, dir, t, &-self.m_n, true, self);
        }
    }

    fn get_texture_color(&self, p: &Vector3) -> Color {
        return if self.m_material.has_texture() {
            let mut u = (p.clone() - self.m_o).dot(&self.m_dx) / self.m_dx.mod2();
            let mut v = (p.clone() - self.m_o).dot(&self.m_dy) / self.m_dy.mod2();
            u -= u.floor();
            v -= v.floor();
            self.m_material.get_texture_color(u, v)
        } else {
            Color { r: self.m_material.color.r, g: self.m_material.color.g, b: self.m_material.color.b }
        }
    }
}

impl Plane {
    fn set_texture_axis(&mut self, o: Vector3, dx: Vector3, dy: Vector3) -> () {
        self.m_o = o;
        self.m_dx = dx;
        self.m_dy = dy;
    }
}


#[derive(Clone)]
pub struct Sphere {
    pub m_material: Material,
    pub m_r: f64,
    pub m_o: Vector3,
    pub m_dz: Vector3,
    pub m_dx: Vector3,
}

pub fn make_sphere(m: &Material, o: Vector3, r: f64) -> Sphere {
    Sphere {
        m_material: m.clone(),
        m_r: r,
        m_o: o,
        m_dz: Vector3 { x: 0.0, y: 0.0, z: 1.0 },
        m_dx: Vector3 { x: 1.0, y: 0.0, z: 0.0 },
    }
}

impl Object for Sphere {

    fn get_material(&self) -> &Material {
        &self.m_material
    }

    fn collision(&self, start: &Vector3, dir: &Vector3) -> Collision {
        let d = dir.unitise();
        let oc = self.m_o - start.clone();
        let tca = oc.dot(&d);

        if tca > 0.00001 {
            let thc2 = self.m_r * self.m_r - oc.mod2() + tca * tca;
            //println!("{}", thc2);
            if thc2 > 0.0001 {
                let thc = thc2.sqrt();
                let t1 = tca - thc;
                let t2 = tca + thc;
                //println!("{}", t2);
                //println!("{}", thc2);
                //println!("{}", t1);
                //println!("3456789");
                if t1 > thc2 && thc2 > 0.001 {
                    let p = start.clone() + d * t1;
                    return collision_sphere(start.clone(), d, t1, p - self.m_o, false, self);
                } else if t2 > thc2 && thc2 > 0.001 {
                    let p = start.clone() + d * t2;
                    return collision_sphere(start.clone(), d, t2, self.m_o - p, true, self);
                }
            }
        }


        return Collision {
            at_object: false,
            at_sphere: false,
            sphere: &Sphere {
                m_material: Material {
                    color: Color { r: 0.3, g: 0.3, b: 1.0 },
                    absorb_color: Color { r: 1.0, g: 1.0, b: 0.0 },
                    diff: 0.0,
                    spec: 0.8,
                    reflection: 0.4,
                    refraction: 0.6,
                    r_index: 1.5,
                    m_texture: None,
                    use_m_texture: false,
                },
                m_r: 0.0,
                m_o: Vector3 { x: 0.0, y: 0.0, z: 0.0 },
                m_dz: Vector3 { x: 0.0, y: 0.0, z: 0.0 },
                m_dx: Vector3 { x: 0.0, y: 0.0, z: 0.0 },
            },
            at_plane: false,
            plane: &Plane {
                m_material: Material {
                    color: Color { r: 0.3, g: 0.3, b: 1.0 },
                    absorb_color: Color { r: 1.0, g: 1.0, b: 0.0 },
                    diff: 0.0,
                    spec: 0.8,
                    reflection: 0.4,
                    refraction: 0.6,
                    r_index: 1.5,
                    m_texture: None,
                    use_m_texture: false,
                },
                m_d: 0.0,
                m_n: Vector3 { x: 0.0, y: 0.0, z: 0.0 },
                m_o: Vector3 { x: 0.0, y: 0.0, z: 0.0 },
                m_dx: Vector3 { x: 0.0, y: 0.0, z: 0.0 },
                m_dy: Vector3 { x: 0.0, y: 0.0, z: 0.0 },
            },
            at_light: false,
            at_rect_light: false,
            rect_light: &RectLight {
                m_color: Color { r: 0.0, g: 0.0, b: 0.0 },
                m_o: Vector3 { x: 0.0, y: 0.0, z: 0.0 },
                m_dx: Vector3 { x: 0.0, y: 0.0, z: 0.0 },
                m_dy: Vector3 { x: 0.0, y: 0.0, z: 0.0 },
            },
            at_point_light: false,
            point_light: &PointLight {
                m_color: Color { r: 0.0, g: 0.0, b: 0.0 },
                m_o: Vector3 { x: 0.0, y: 0.0, z: 0.0 },
            },
            ray_start: Vector3 { x: 0.0, y: 0.0, z: 0.0 },
            ray_dir: Vector3 { x: 0.0, y: 0.0, z: 0.0 },
            p: Vector3 { x: 0.0, y: 0.0, z: 0.0 },
            n: Vector3 { x: 0.0, y: 0.0, z: 0.0 },
            dist: 0.0,
            is_internal: false,
        };
    }

    fn get_texture_color(&self, p: &Vector3) -> Color {
        return if self.m_material.has_texture() {
            let n = (p.clone() - self.m_o).unitise();
            let b = n.dot(&self.m_dz).acos();
            let a = (n.dot(&self.m_dx) / b.sin()).max(-1.0).min(1.0).acos();
            let v = b / std::f64::consts::PI;
            let mut u = a / 2.0 / std::f64::consts::PI;

            if n.dot(&(self.m_dz * self.m_dx)) < 0.0 {
                u = 1.0 - u;
            }

            self.m_material.get_texture_color(u, v)
        } else {
            Color { r: self.m_material.color.r, g: self.m_material.color.g, b: self.m_material.color.b }
        }
    }
}

impl Sphere {
    fn set_texture_axis(&mut self, dz: Vector3, dx: Vector3) -> () {
        self.m_dz = dz;
        self.m_dx = dx;
    }
}
