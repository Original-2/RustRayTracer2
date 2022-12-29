mod vector3;
mod color;
mod bmp;
mod material;
mod objects;
mod collision;
mod camera;
mod light;
mod scene;
mod raytracer;


use crate::color::Color;
use crate::bmp::Bmp;
use crate::material::Material;
use crate::vector3::Vector3;
use crate::camera::Camera;
use crate::collision::Collision;
use crate::objects::Sphere;
use crate::objects::Plane;
use crate::light::RectLight;
use crate::light::PointLight;
use crate::scene::Scene;
use crate::raytracer::RayTracer;
use crate::scene::makeScene;

fn main() {
    let scene = makeScene();
    let engine = RayTracer{
        m_scene: scene
    };
    engine.run();
}
