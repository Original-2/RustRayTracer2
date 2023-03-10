use crate::color::Color;
use crate::light::PointLight;
use crate::light::RectLight;
use crate::material::Material;
use crate::Plane;
use crate::Sphere;
use crate::vector3::Vector3;

#[derive(Clone)]
pub struct Collision<'a> {
    pub at_object: bool,
    pub at_sphere: bool,
    pub sphere: &'a Sphere,
    pub at_plane: bool,
    pub plane: &'a Plane,
    pub at_light: bool,
    pub at_point_light: bool,
    pub point_light: &'a PointLight,
    pub at_rect_light: bool,
    pub rect_light: &'a RectLight,
    pub ray_start: Vector3,
    pub ray_dir: Vector3,
    pub p: Vector3,
    pub n: Vector3,
    pub dist: f64,
    pub is_internal: bool,
}

pub fn collision_plane<'a>(start: &Vector3, dir: &Vector3, t: f64, n: &Vector3, inward: bool, obj: &'a Plane) -> Collision<'a> {
    Collision {
        at_object: true,
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
        at_plane: true,
        plane: obj,
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
        ray_start: start.clone(),
        ray_dir: dir.unitise(),
        p: start.clone() + dir.clone() * t,
        n: n.unitise(),
        dist: t * dir.mod1(),
        is_internal: inward,
    }
}

pub fn collision_sphere<'a>(start: Vector3, dir: Vector3, t: f64, n: Vector3, inward: bool, obj: &'a Sphere) -> Collision<'a> {
    Collision {
        at_object: true,
        at_sphere: true,
        sphere: &obj,
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
        ray_start: start,
        ray_dir: dir.unitise(),
        p: start + dir * t,
        n: n.unitise(),
        dist: t * dir.mod1(),
        is_internal: inward,
    }
}


pub fn collision_point_light<'a>(start: &Vector3, dir: &Vector3, t: f64, l: &'a PointLight) -> Collision<'a> {
    Collision {
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
        at_light: true,
        at_rect_light: false,
        rect_light: &RectLight {
            m_color: Color { r: 0.0, g: 0.0, b: 0.0 },
            m_o: Vector3 { x: 0.0, y: 0.0, z: 0.0 },
            m_dx: Vector3 { x: 0.0, y: 0.0, z: 0.0 },
            m_dy: Vector3 { x: 0.0, y: 0.0, z: 0.0 },
        },
        at_point_light: true,
        point_light: &l,
        ray_start: start.clone(),
        ray_dir: dir.unitise(),
        p: start.clone() + dir.clone() * t,
        n: Vector3 { x: 0.0, y: 0.0, z: 0.0 },
        dist: t * dir.mod1(),
        is_internal: false,
    }
}

pub fn collision_rect_light<'a>(start: &Vector3, dir: &Vector3, t: f64, l: &'a RectLight) -> Collision<'a> {
    Collision {
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
        at_light: true,
        at_rect_light: true,
        rect_light: l,
        at_point_light: false,
        point_light: &PointLight {
            m_color: Color { r: 0.0, g: 0.0, b: 0.0 },
            m_o: Vector3 { x: 0.0, y: 0.0, z: 0.0 },
        },
        ray_start: start.clone(),
        ray_dir: dir.unitise(),
        p: start.clone() + dir.clone() * t,
        n: Vector3 { x: 0.0, y: 0.0, z: 0.0 },
        dist: t * dir.mod1(),
        is_internal: false,
    }
}


impl<'a> Collision<'a> {

    pub fn is_hit(&self) -> bool {
        return if self.dist < 100000000.0 {
            true
        } else {
            false
        }
    }

    pub fn at_object(&self) -> bool {
        self.at_object
    }

    pub fn at_light(&self) -> bool {
        self.at_light
    }

}
