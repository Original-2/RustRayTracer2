use crate::bmp::bmp_from_file;
use crate::bmp::BmpNew;
use crate::camera::Camera;
use crate::collision::Collision;
use crate::color::Color;
use crate::light::Light;
use crate::light::MakePointLight;
use crate::light::makeRectLight;
use crate::light::PointLight;
use crate::light::RectLight;
use crate::material::Material;
use crate::objects::make_plane;
use crate::objects::make_sphere;
use crate::objects::Object;
use crate::objects::Plane;
use crate::objects::Sphere;
use crate::vector3::Vector3;

#[derive(Clone)]
pub struct Scene {
    m_camera: Camera,
    m_ambient_color: Color,

    pub m_p_lights: Vec<PointLight>,
    pub m_r_lights: Vec<RectLight>,
    pub m_planes: Vec<Plane>,
    pub m_spheres: Vec<Sphere>,
}

pub fn make_scene() -> Scene {

    let cam = Camera {
        m_eye: Vector3 { x: 0.0, y: -1.0, z: -0.5 },
        m_dir: Vector3 { x: 0.0, y: 1.0, z: 0.0 },
        m_up: Vector3 { x: 0.0, y: 0.0, z: 1.0 },
        m_w: 500,
        m_h: 500,
        m_fov_y: 45.0 * std::f64::consts::PI / 180.0,
        m_dist: 1.0,
        m_dw: (Vector3 { x: 0.0, y: 0.0, z: 1.0 } * ((45.0 * std::f64::consts::PI / 180.0) as f64).tan() * -1.0 * Vector3 { x: 0.0, y: 1.0, z: 0.0 }).unitise() * ((45.0 * std::f64::consts::PI / 180.0) as f64).tan(),
        m_dh: Vector3 { x: 0.0, y: 0.0, z: 1.0 } * ((45.0 * std::f64::consts::PI / 180.0) as f64).tan() * -1.0,
        m_film: BmpNew(500, 500, Color { r: 1.0, g: 0.2, b: 0.2 }),
    };

    let l1 = vec![makeRectLight(Color { r: 1.0, g: 1.0, b: 1.0 }, Vector3 { x: 0.0, y: -0.8, z: 0.5 }, Vector3 { x: 0.3, y: 0.0, z: 0.0 }, Vector3 { x: 0.0, y: 0.3, z: 0.0 })];
    let l2 = vec![MakePointLight(Color { r: 1.0, g: 1.0, b: 1.0 }, Vector3 { x: 0.0, y: -1.0, z: 0.9 })];

    let ground_material = Material {
        color: Color { r: 0.3, g: 0.6, b: 0.3 },
        absorb_color: Color { r: 0.0, g: 0.0, b: 0.0 },
        diff: 0.8,
        spec: 0.2,
        reflection: 0.2,
        refraction: 0.0,
        r_index: 1.0,
        m_texture: Some(bmp_from_file(String::from("../textures/floor.bmp"))),
        use_m_texture: false,
    };

    let red_wall = Material {
        color: Color { r: 1.0, g: 0.0, b: 0.0 },
        absorb_color: Color { r: 0.0, g: 0.0, b: 0.0 },
        diff: 0.9,
        spec: 0.2,
        reflection: 0.1,
        refraction: 0.0,
        r_index: 1.0,
        m_texture: None,
        use_m_texture: false,
    };

    let blue_wall = Material {
        color: Color { r: 0.0, g: 0.0, b: 1.0 },
        absorb_color: Color { r: 0.0, g: 0.0, b: 0.0 },
        diff: 0.9,
        spec: 0.2,
        reflection: 0.1,
        refraction: 0.0,
        r_index: 1.0,
        m_texture: None,
        use_m_texture: false,
    };

    let write_wall = Material {
        color: Color { r: 0.8, g: 0.8, b: 0.8 },
        absorb_color: Color { r: 0.0, g: 0.0, b: 0.0 },
        diff: 0.8,
        spec: 0.2,
        reflection: 0.5,
        refraction: 0.0,
        r_index: 1.0,
        m_texture: Some(bmp_from_file(String::from("../textures/board.bmp"))),
        use_m_texture: false,
    };

    let blue_glass = Material {
        color: Color { r: 0.3, g: 0.3, b: 1.0 },
        absorb_color: Color { r: 1.0, g: 1.0, b: 0.0 },
        diff: 0.0,
        spec: 0.8,
        reflection: 0.4,
        refraction: 0.6,
        r_index: 1.5,
        m_texture: None,
        use_m_texture: false,
    };

    let write_glass = Material {
        color: Color { r: 0.8, g: 0.8, b: 0.8 },
        absorb_color: Color { r: 0.0, g: 0.0, b: 0.0 },
        diff: 0.8,
        spec: 0.5,
        reflection: 0.4,
        refraction: 0.0,
        r_index: 1.0,
        m_texture: Some(bmp_from_file(String::from("../textures/earth.bmp"))),
        use_m_texture: false,
    };

    let ground = make_plane(&ground_material, Vector3 { x: 0.0, y: 0.0, z: 1.0 }, 1.0);
    let ceiling = make_plane(&write_wall, Vector3 { x: 0.0, y: 0.0, z: -1.0 }, 1.0);
    let wall1 = make_plane(&write_wall, Vector3 { x: 0.0, y: -1.0, z: 0.0 }, 1.0);
    let wall2 = make_plane(&blue_wall, Vector3 { x: -1.0, y: 0.0, z: 0.0 }, 1.0);
    let wall3 = make_plane(&red_wall, Vector3 { x: 1.0, y: 0.0, z: 0.0 }, 1.0);
    //let wall4 = make_plane(write_wall, Vector3 {x: 0.0, y: 1.0, z: 0.0}, 1.0);
    let ball1 = make_sphere(&write_glass, Vector3 { x: 0.5, y: 0.2, z: -0.6 }, 0.4);
    let ball2 = make_sphere(&blue_glass, Vector3 { x: -0.5, y: 0.2, z: -0.6 }, 0.4);

    let planes = vec![ground, ceiling, wall1, wall2, wall3];
    let spheres = vec![ball1, ball2];

    let out = Scene {
        m_camera: cam,
        m_ambient_color: Color { r: 0.4, g: 0.2, b: 0.3 },
        m_p_lights: l2,
        m_r_lights: l1,
        m_planes: planes,
        m_spheres: spheres,
    };

    return out;
}


impl Scene {

    pub fn get_camera(&self) -> &Camera {
        return &self.m_camera;
    }

    pub fn get_ambient_light_color(&self) -> Color {
        return self.m_ambient_color;
    }

    pub fn find_nearest_collision(&self, start: Vector3, dir: Vector3) -> Collision {
        let mut ret: Collision = Collision {
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
            dist: 100000000000000000.0,
            is_internal: false,
        };

        for l in &self.m_r_lights {
            let coll = l.collision(&start, &dir);
            if coll.is_hit() && coll.dist + 0.001 < ret.dist {
                ret = coll;
            }
        }

        for l in &self.m_p_lights {
            let coll = l.collision(&start, &dir);
            if coll.is_hit() && coll.dist + 0.001 < ret.dist {
                ret = coll;
            }
        }


        for o in &self.m_planes {
            let coll = o.collision(&start, &dir);
            if coll.is_hit() && coll.dist + 0.001 < ret.dist && coll.at_object {
                ret = coll;
            }
        }
        for o in &self.m_spheres {
            let coll = o.collision(&start, &dir);
            if coll.is_hit() && coll.dist + 0.001 < ret.dist && coll.at_object {
                ret = coll;
            }
        }
        return ret;
    }
}

