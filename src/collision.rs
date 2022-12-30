use crate::color::Color;
use crate::material::Material;
use crate::vector3::Vector3;
use crate::light::PointLight;
use crate::light::RectLight;
use crate::bmp::BmpNew;
use crate::Sphere;
use crate::Plane;

#[derive(Clone)]
pub struct Collision{
    pub at_object: bool,
    pub at_sphere: bool,
    pub sphere: Sphere,
    pub at_plane: bool,
    pub plane: Plane,
    pub at_light: bool,
    pub at_plight: bool,
    pub plight: PointLight,
    pub at_rlight: bool,
    pub rlight: RectLight,
    pub ray_start: Vector3,
    pub ray_dir: Vector3,
    pub p: Vector3,
    pub n: Vector3,
    pub dist: f64,
    pub is_internal: bool,
}

pub fn Collision_plane (start: Vector3, dir: Vector3, t: f64, n: Vector3, inward: bool,  obj: Plane) -> Collision {
    Collision { at_object: true,
                at_sphere: false,
                sphere: Sphere {
                    m_material: Material {
                        color: Color{r: 0.3, g: 0.3, b: 1.0},
                        absorb_color: Color{r: 1.0, g: 1.0, b: 0.0},
                        diff: 0.0,
                        spec: 0.8,
                        refl: 0.4,
                        refr: 0.6,
                        rindex: 1.5,
                        m_texture: None,
                        use_m_texture: false,},
                m_r: 0.0,
                m_o: Vector3 {x: 0.0, y: 0.0, z: 0.0},
                m_dz: Vector3 {x: 0.0, y: 0.0, z: 0.0},
                m_dx: Vector3 {x: 0.0, y: 0.0, z: 0.0},
                    },
                at_plane: true,
                plane: obj,
            at_light: false,
            at_rlight: false,
            rlight: RectLight {
                m_color: Color {r: 0.0, g: 0.0, b: 0.0},
                m_o: Vector3 {x: 0.0, y: 0.0, z: 0.0},
                m_dx: Vector3 {x: 0.0, y: 0.0, z: 0.0},
                m_dy: Vector3 {x: 0.0, y: 0.0, z: 0.0}
            },
            at_plight: false,
            plight: PointLight {
                    m_color: Color {r: 0.0, g: 0.0, b: 0.0},
                    m_o: Vector3 {x: 0.0, y: 0.0, z: 0.0},
            },
            ray_start: start,
            ray_dir: dir.unitise(),
            p: start + dir * t,
            n: n.unitise(),
            dist: t * dir.mod1(),
            is_internal: inward,}
}

pub fn Collision_sphere (start: Vector3, dir: Vector3, t: f64, n: Vector3, inward: bool,  obj: Sphere) -> Collision {
    Collision { at_object: true,
                at_sphere: true,
                sphere: obj,
                at_plane: false,
                plane: Plane {
                    m_material: Material {
                        color: Color{r: 0.3, g: 0.3, b: 1.0},
                        absorb_color: Color{r: 1.0, g: 1.0, b: 0.0},
                        diff: 0.0,
                        spec: 0.8,
                        refl: 0.4,
                        refr: 0.6,
                        rindex: 1.5,
                        m_texture: None,
                        use_m_texture: false,},
                    m_d: 0.0,
                    m_n: Vector3 {x: 0.0, y: 0.0, z: 0.0},
                    m_o: Vector3 {x: 0.0, y: 0.0, z: 0.0},
                    m_dx: Vector3 {x: 0.0, y: 0.0, z: 0.0},
                    m_dy: Vector3 {x: 0.0, y: 0.0, z: 0.0}},
            at_light: false,
            at_rlight: false,
            rlight: RectLight {
                m_color: Color {r: 0.0, g: 0.0, b: 0.0},
                m_o: Vector3 {x: 0.0, y: 0.0, z: 0.0},
                m_dx: Vector3 {x: 0.0, y: 0.0, z: 0.0},
                m_dy: Vector3 {x: 0.0, y: 0.0, z: 0.0}
            },
            at_plight: false,
            plight: PointLight {
                    m_color: Color {r: 0.0, g: 0.0, b: 0.0},
                    m_o: Vector3 {x: 0.0, y: 0.0, z: 0.0},
            },
            ray_start: start,
            ray_dir: dir.unitise(),
            p: start + dir * t,
            n: n.unitise(),
            dist: t * dir.mod1(),
            is_internal: inward,}
}


pub fn Collision_plight(start: Vector3, dir: Vector3, t: f64, l: PointLight) -> Collision {
    Collision { at_object: false,
                at_sphere: false,
                sphere: Sphere {
                    m_material: Material {
                        color: Color{r: 0.3, g: 0.3, b: 1.0},
                        absorb_color: Color{r: 1.0, g: 1.0, b: 0.0},
                        diff: 0.0,
                        spec: 0.8,
                        refl: 0.4,
                        refr: 0.6,
                        rindex: 1.5,
                        m_texture: None,
                        use_m_texture: false,},
                m_r: 0.0,
                m_o: Vector3 {x: 0.0, y: 0.0, z: 0.0},
                m_dz: Vector3 {x: 0.0, y: 0.0, z: 0.0},
                m_dx: Vector3 {x: 0.0, y: 0.0, z: 0.0},
                    },
                at_plane: false,
                plane: Plane {
                    m_material: Material {
                        color: Color{r: 0.3, g: 0.3, b: 1.0},
                        absorb_color: Color{r: 1.0, g: 1.0, b: 0.0},
                        diff: 0.0,
                        spec: 0.8,
                        refl: 0.4,
                        refr: 0.6,
                        rindex: 1.5,
                        m_texture: None,
                        use_m_texture: false,},
                    m_d: 0.0,
                    m_n: Vector3 {x: 0.0, y: 0.0, z: 0.0},
                    m_o: Vector3 {x: 0.0, y: 0.0, z: 0.0},
                    m_dx: Vector3 {x: 0.0, y: 0.0, z: 0.0},
                    m_dy: Vector3 {x: 0.0, y: 0.0, z: 0.0}},
                at_light: true,
                at_rlight: false,
                rlight: RectLight {
                    m_color: Color {r: 0.0, g: 0.0, b: 0.0},
                    m_o: Vector3 {x: 0.0, y: 0.0, z: 0.0},
                    m_dx: Vector3 {x: 0.0, y: 0.0, z: 0.0},
                    m_dy: Vector3 {x: 0.0, y: 0.0, z: 0.0}
                },
                at_plight: true,
                plight: l,
                ray_start: start,
                ray_dir: dir.unitise(),
                p: start + dir * t,
                n: Vector3 {x: 0.0, y: 0.0, z: 0.0},
                dist: t * dir.mod1(),
                is_internal: false,}
}

pub fn Collision_rlight(start: Vector3, dir: Vector3, t: f64, l: RectLight) -> Collision {
    Collision { at_object: false,
                at_sphere: false,
                sphere: Sphere {
                    m_material: Material {
                        color: Color{r: 0.3, g: 0.3, b: 1.0},
                        absorb_color: Color{r: 1.0, g: 1.0, b: 0.0},
                        diff: 0.0,
                        spec: 0.8,
                        refl: 0.4,
                        refr: 0.6,
                        rindex: 1.5,
                        m_texture: None,
                        use_m_texture: false,},
                m_r: 0.0,
                m_o: Vector3 {x: 0.0, y: 0.0, z: 0.0},
                m_dz: Vector3 {x: 0.0, y: 0.0, z: 0.0},
                m_dx: Vector3 {x: 0.0, y: 0.0, z: 0.0},
                    },
                at_plane: false,
                plane: Plane {
                    m_material: Material {
                        color: Color{r: 0.3, g: 0.3, b: 1.0},
                        absorb_color: Color{r: 1.0, g: 1.0, b: 0.0},
                        diff: 0.0,
                        spec: 0.8,
                        refl: 0.4,
                        refr: 0.6,
                        rindex: 1.5,
                        m_texture: None,
                        use_m_texture: false,},
                    m_d: 0.0,
                    m_n: Vector3 {x: 0.0, y: 0.0, z: 0.0},
                    m_o: Vector3 {x: 0.0, y: 0.0, z: 0.0},
                    m_dx: Vector3 {x: 0.0, y: 0.0, z: 0.0},
                    m_dy: Vector3 {x: 0.0, y: 0.0, z: 0.0}},
                at_light: true,
                at_rlight: true,
                rlight: l,
                at_plight: false,
                plight: PointLight {
                        m_color: Color {r: 0.0, g: 0.0, b: 0.0},
                        m_o: Vector3 {x: 0.0, y: 0.0, z: 0.0},
                },
                ray_start: start,
                ray_dir: dir.unitise(),
                p: start + dir * t,
                n: Vector3 {x: 0.0, y: 0.0, z: 0.0},
                dist: t * dir.mod1(),
                is_internal: false,}
}


impl Collision {
    pub fn isHit(&self) -> bool {
        if self.dist < 100000000.0 {
            return true;
        }else{
            return false;
        }
    }

    pub fn atObject(&self) -> bool {
        self.at_object
    }

    pub fn atLight(&self) -> bool {
        self.at_light
    }
}
