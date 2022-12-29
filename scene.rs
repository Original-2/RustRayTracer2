use crate::color::Color;
use crate::bmp::BmpNew;
use crate::bmp::Bmp_From_file;
use crate::material::Material;
use crate::vector3::Vector3;
use crate::camera::Camera;
use crate::collision::Collision;
use crate::objects::Sphere;
use crate::objects::Plane;
use crate::light::RectLight;
use crate::light::PointLight;
use crate::objects::Object;
use crate::light::Light;
use crate::light::MakePointLight;
use crate::light::makeRectLight;
use crate::objects::makePlane;
use crate::objects::makeSphere;




#[derive(Clone)]
pub struct Scene{
    m_camera: Camera,
    m_ambient_color: Color,

    pub m_plights: Vec<PointLight>,
    pub m_rlights: Vec<RectLight>,
    pub m_planes: Vec<Plane>,
    pub m_spheres: Vec<Sphere>,
}

pub fn makeScene() -> Scene{
    let cam = Camera {
        m_eye: Vector3 {x: 0.0, y: -1.0, z: -0.5},
        m_dir: Vector3 {x: 0.0, y: 1.0, z: 0.0},
        m_up: Vector3 {x: 0.0, y: 0.0, z: 1.0},
        m_w: 500,
        m_h: 500,
        m_fovy: 45.0 * 3.14159265358979311599796346854 / 180.0,
        m_dist: 1.0,
        m_dw: (Vector3 {x: 0.0, y: 0.0, z: 1.0}  * ((45.0 * 3.14159265358979311599796346854 / 180.0)as f64).tan() * -1.0 * Vector3 {x: 0.0, y: 1.0, z: 0.0}).unitise() * ((45.0 * 3.14159265358979311599796346854 / 180.0)as f64).tan(),
        m_dh: Vector3 {x: 0.0, y: 0.0, z: 1.0}  * ((45.0 * 3.14159265358979311599796346854 / 180.0)as f64).tan() * -1.0,
        m_film: BmpNew(500, 500, Color {r: 1.0, g: 0.2, b: 0.2,}),
    };

    let l1 = vec![makeRectLight(Color {r: 1.0, g: 1.0, b: 1.0}, Vector3 {x: 0.0, y: -0.8, z: 0.5}, Vector3 {x: 0.3, y: 0.0, z: 0.0}, Vector3 {x: 0.0, y: 0.3, z: 0.0})];
    let l2 = vec![MakePointLight(Color {r: 1.0, g: 1.0, b: 1.0}, Vector3 {x: 0.0, y: -1.0, z: 0.9})];

    let groundMaterial = Material {
        color: Color{r: 0.3, g: 0.6, b: 0.3},
        absorb_color: Color{r: 0.0, g: 0.0, b: 0.0},
        diff: 0.8,
        spec: 0.2,
        refl: 0.2,
        refr: 0.0,
        rindex: 1.0,
        m_texture: Bmp_From_file(String::from("../textures/floor.bmp")),
        use_m_texture: false,};
    let mirrorMaterial = Material {
        color: Color{r: 0.9, g: 0.9, b: 0.9},
        absorb_color: Color{r: 0.0, g: 0.0, b: 0.0},
        diff: 0.1,
        spec: 0.8,
        refl: 0.9,
        refr: 0.0,
        rindex: 1.0,
        m_texture: BmpNew(100, 100, Color {r: 0.0, g: 0.0, b: 0.0}),
        use_m_texture: false,};
    let redWall = Material {
        color: Color{r: 1.0, g: 0.0, b: 0.0},
        absorb_color: Color{r: 0.0, g: 0.0, b: 0.0},
        diff: 0.9,
        spec: 0.2,
        refl: 0.1,
        refr: 0.0,
        rindex: 1.0,
        m_texture: BmpNew(100, 100, Color {r: 0.0, g: 0.0, b: 0.0}),
        use_m_texture: false,};
    let blueWall = Material {
        color: Color{r: 0.0, g: 0.0, b: 1.0},
        absorb_color: Color{r: 0.0, g: 0.0, b: 0.0},
        diff: 0.9,
        spec: 0.2,
        refl: 0.1,
        refr: 0.0,
        rindex: 1.0,
        m_texture: BmpNew(100, 100, Color {r: 0.0, g: 0.0, b: 0.0}),
        use_m_texture: false,};
    let writeWall = Material {
        color: Color{r: 0.8, g: 0.8, b: 0.8},
        absorb_color: Color{r: 0.0, g: 0.0, b: 0.0},
        diff: 0.8,
        spec: 0.2,
        refl: 0.5,
        refr: 0.0,
        rindex: 1.0,
        m_texture: Bmp_From_file(String::from("../textures/board.bmp")),
        use_m_texture: false,};
    let blueGlass = Material {
        color: Color{r: 0.3, g: 0.3, b: 1.0},
        absorb_color: Color{r: 1.0, g: 1.0, b: 0.0},
        diff: 0.0,
        spec: 0.8,
        refl: 0.4,
        refr: 0.6,
        rindex: 1.5,
        m_texture: BmpNew(100, 100, Color {r: 0.0, g: 0.0, b: 0.0}),
        use_m_texture: false,};
    let writeGlass = Material {
        color: Color{r: 0.8, g: 0.8, b: 0.8},
        absorb_color: Color{r: 0.0, g: 0.0, b: 0.0},
        diff: 0.8,
        spec: 0.5,
        refl: 0.4,
        refr: 0.0,
        rindex: 1.0,
        m_texture: Bmp_From_file(String::from("../textures/earth.bmp")),
        use_m_texture: false,};

    let ground = makePlane(groundMaterial.clone(), Vector3 {x: 0.0, y: 0.0, z: 1.0}, 1.0);
    let ceiling = makePlane(writeWall.clone(), Vector3 {x: 0.0, y: 0.0, z: -1.0}, 1.0);
    let wall1 = makePlane(writeWall.clone(), Vector3 {x: 0.0, y: -1.0, z: 0.0}, 1.0);
    let wall2 = makePlane(blueWall.clone(), Vector3 {x: -1.0, y: 0.0, z: 0.0}, 1.0);
    let wall3 = makePlane(redWall.clone(), Vector3 {x: 1.0, y: 0.0, z: 0.0}, 1.0);
    //let wall4 = makePlane(writeWall.clone(), Vector3 {x: 0.0, y: 1.0, z: 0.0}, 1.0);
    let ball1 = makeSphere(writeGlass.clone(), Vector3 {x: 0.5, y: 0.2, z: -0.6} , 0.4);
    let ball2 = makeSphere(blueGlass.clone(), Vector3 {x: -0.5, y: 0.2, z: -0.6}, 0.4);


    let planes = vec![ground, ceiling, wall1, wall2, wall3];
    let spheres = vec![ball1, ball2];

    //for i in otemp {
    //    o.push(i.clone().to_owned());
    //};

    //o.sort_by(|a, b| a.GetMaterial().compare(b.GetMaterial()));

    let out = Scene {
        m_camera: cam,
        m_ambient_color: Color {r: 0.4, g: 0.2, b: 0.3},
        m_plights: l2,
        m_rlights: l1,
        m_planes: planes,
        m_spheres: spheres,
    };

    return *Box::new(out);


}










impl Scene{
    pub fn getCamera(&self) -> Camera {
        return self.m_camera.clone();
    }

    pub fn getAmbientLightColor(&self) -> Color {
        return self.m_ambient_color;
    }



    pub fn findNearestCollision(&self, start: Vector3, dir: Vector3) -> Collision{
        let mut ret: Collision = Collision{
            at_object: false,
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
                    m_texture: BmpNew(100, 100, Color {r: 0.0, g: 0.0, b: 0.0}),
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
                    m_texture: BmpNew(100, 100, Color {r: 0.0, g: 0.0, b: 0.0}),
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
            ray_start: Vector3 {x: 0.0, y: 0.0, z: 0.0},
            ray_dir: Vector3 {x: 0.0, y: 0.0, z: 0.0},
            p: Vector3 {x: 0.0, y: 0.0, z: 0.0},
            n: Vector3 {x: 0.0, y: 0.0, z: 0.0},
            dist: 100000000000000000.0,
            is_internal: false};

        for l in self.m_rlights.clone() {
            let coll = l.collision(start, dir);
            if coll.clone().isHit() && coll.clone().dist + 0.001 < ret.dist {
                ret = coll;
            }
        }

        for l in self.m_plights.clone() {
            let coll = l.collision(start, dir);
            if coll.clone().isHit() && coll.clone().dist + 0.001 < ret.dist {
                ret = coll;
            }
        }


        for o in self.m_planes.clone() {
            let coll = o.clone().collision(start, dir);
            if coll.clone().isHit() && coll.dist.clone() + 0.001 < ret.dist && coll.at_object {
                ret = coll.clone();
                }
            }
        for o in self.m_spheres.clone() {
            let coll = o.collision(start, dir);
            if coll.clone().isHit() && coll.dist.clone() + 0.001 < ret.dist  && coll.at_object{
                ret = coll.clone();
                }
            }
        return ret;
    }
}

