use std::fs::remove_file;
use std::thread;

use image;
use image::{GenericImageView, ImageBuffer};
use image::Rgb;

use crate::collision::Collision;
use crate::color::Color;
use crate::light::RectLight;
use crate::material::Material;
use crate::objects::Plane;
use crate::objects::Sphere;
use crate::raytracer::RayTracer;
use crate::scene::make_scene;
use crate::scene::Scene;
use crate::vector3::Vector3;

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


fn go(filename: String, h_from: i32, h: i32, w_from: i32, w: i32) {
    let scene = make_scene();
    let engine = RayTracer {
        m_scene: scene
    };
    engine.run(filename, h_from, h, w_from, w);
}

fn main() {

    let handle1 = thread::spawn(move || {
        go(String::from("out1.png"), 0, 100, 0, 100);
    });
    let handle2 = thread::spawn(move || {
        go(String::from("out2.png"), 100, 100, 0, 100);
    });
    let handle3 = thread::spawn(move || {
        go(String::from("out3.png"), 200, 100, 0, 100);
    });
    let handle4 = thread::spawn(move || {
        go(String::from("out4.png"), 300, 100, 0, 100);
    });
    let handle5 = thread::spawn(move || {
        go(String::from("out5.png"), 0, 100, 100, 100);
    });
    let handle6 = thread::spawn(move || {
        go(String::from("out6.png"), 100, 100, 100, 100);
    });
    let handle7 = thread::spawn(move || {
        go(String::from("out7.png"), 200, 100, 100, 100);
    });
    let handle8 = thread::spawn(move || {
        go(String::from("out8.png"), 300, 100, 100, 100);
    });
    let handle9 = thread::spawn(move || {
        go(String::from("out9.png"), 0, 100, 200, 100);
    });
    let handle10 = thread::spawn(move || {
        go(String::from("out10.png"), 100, 100, 200, 100);
    });
    let handle11 = thread::spawn(move || {
        go(String::from("out11.png"), 200, 100, 200, 100);
    });
    let handle12 = thread::spawn(move || {
        go(String::from("out12.png"), 300, 100, 200, 100);
    });
    let handle13 = thread::spawn(move || {
        go(String::from("out13.png"), 0, 100, 300, 100);
    });
    let handle14 = thread::spawn(move || {
        go(String::from("out14.png"), 100, 100, 300, 100);
    });
    let handle15 = thread::spawn(move || {
        go(String::from("out15.png"), 200, 100, 300, 100);
    });
    let handle16 = thread::spawn(move || {
        go(String::from("out16.png"), 300, 100, 300, 100);
    });

    handle1.join().unwrap();
    handle2.join().unwrap();
    handle3.join().unwrap();
    handle4.join().unwrap();
    handle5.join().unwrap();
    handle6.join().unwrap();
    handle7.join().unwrap();
    handle8.join().unwrap();
    handle9.join().unwrap();
    handle10.join().unwrap();
    handle11.join().unwrap();
    handle12.join().unwrap();
    handle13.join().unwrap();
    handle14.join().unwrap();
    handle15.join().unwrap();
    handle16.join().unwrap();

    let mut image = ImageBuffer::<Rgb<u8>, Vec<u8>>::new(400 as u32, 400 as u32);

    populate_final_image_with_part(&mut image, "out1.png", 0, 0);
    populate_final_image_with_part(&mut image, "out2.png", 100, 0);
    populate_final_image_with_part(&mut image, "out3.png", 200, 0);
    populate_final_image_with_part(&mut image, "out4.png", 300, 0);
    populate_final_image_with_part(&mut image, "out5.png", 0, 100);
    populate_final_image_with_part(&mut image, "out6.png", 100, 100);
    populate_final_image_with_part(&mut image, "out7.png", 200, 100);
    populate_final_image_with_part(&mut image, "out8.png", 300, 100);
    populate_final_image_with_part(&mut image, "out9.png", 0, 200);
    populate_final_image_with_part(&mut image, "out10.png", 100, 200);
    populate_final_image_with_part(&mut image, "out11.png", 200, 200);
    populate_final_image_with_part(&mut image, "out12.png", 300, 200);
    populate_final_image_with_part(&mut image, "out13.png", 0, 300);
    populate_final_image_with_part(&mut image, "out14.png", 100, 300);
    populate_final_image_with_part(&mut image, "out15.png", 200, 300);
    populate_final_image_with_part(&mut image, "out16.png", 300, 300);

    image.save("../result.png").expect("Error while saving image");
}

fn populate_final_image_with_part(image: &mut ImageBuffer<Rgb<u8>, Vec<u8>>, source : &str, y : u32, x : u32) {
    let source_img = image::open(source).unwrap();
    for iy in 0..100 {
        for ix in 0..100 {
            let p = source_img.get_pixel(ix, iy);
            let pix = [p[0], p[1], p[2]];
            image.put_pixel((iy + y) as u32, (ix + x) as u32, Rgb(pix));
        }
    }
    remove_file(source).unwrap();
}
