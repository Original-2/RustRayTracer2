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
use crate::material::Material;
use crate::vector3::Vector3;
use crate::collision::Collision;
use crate::objects::Sphere;
use crate::objects::Plane;
use crate::light::RectLight;
use crate::scene::Scene;
use crate::raytracer::RayTracer;
use crate::scene::makeScene;
use image;
use image::ImageBuffer;
use image::GenericImageView;

use image::Rgb;
use image::GenericImage;

use std::thread;
fn go(filename: String, h_from: i32, h: i32, w_from: i32, w: i32) {
    let scene = makeScene();
    let engine = RayTracer{
        m_scene: scene
    };
    engine.run(filename, h_from, h, w_from, w);
}

fn main() {
    let mut image = ImageBuffer::<Rgb<u8>, Vec<u8>>::new(400 as u32,400 as u32);

    let mut img = image::open("out1.png").unwrap();
    for y in 0..100 {
        for x in 0..100 {
            let p = img.get_pixel(x, y);
            let pix = [p[0], p[1], p[2]];
            image.put_pixel((y) as u32, (x) as u32, Rgb(pix));
        }
    }

    let mut img = image::open("out2.png").unwrap();
    for y in 0..100 {
        for x in 0..100 {
            let p = img.get_pixel(x, y);
            let pix = [p[0], p[1], p[2]];
            image.put_pixel((y+100) as u32, (x) as u32, Rgb(pix));
        }
    }

    let mut img = image::open("out3.png").unwrap();
    for y in 0..100 {
        for x in 0..100 {
            let p = img.get_pixel(x, y);
            let pix = [p[0], p[1], p[2]];
            image.put_pixel((y+200) as u32, (x) as u32, Rgb(pix));
        }
    }

    let mut img = image::open("out4.png").unwrap();
    for y in 0..100 {
        for x in 0..100 {
            let p = img.get_pixel(x, y);
            let pix = [p[0], p[1], p[2]];
            image.put_pixel((y+300) as u32, (x) as u32, Rgb(pix));
        }
    }

    let mut img = image::open("out5.png").unwrap();
    for y in 0..100 {
        for x in 0..100 {
            let p = img.get_pixel(x, y);
            let pix = [p[0], p[1], p[2]];
            image.put_pixel((y) as u32, (x+100) as u32, Rgb(pix));
        }
    }

    let mut img = image::open("out6.png").unwrap();
    for y in 0..100 {
        for x in 0..100 {
            let p = img.get_pixel(x, y);
            let pix = [p[0], p[1], p[2]];
            image.put_pixel((y+100) as u32, (x+100) as u32, Rgb(pix));
        }
    }

    let mut img = image::open("out7.png").unwrap();
    for y in 0..100 {
        for x in 0..100 {
            let p = img.get_pixel(x, y);
            let pix = [p[0], p[1], p[2]];
            image.put_pixel((y+200) as u32, (x+100) as u32, Rgb(pix));
        }
    }

    let mut img = image::open("out8.png").unwrap();
    for y in 0..100 {
        for x in 0..100 {
            let p = img.get_pixel(x, y);
            let pix = [p[0], p[1], p[2]];
            image.put_pixel((y+300) as u32, (x+100) as u32, Rgb(pix));
        }
    }

    let mut img = image::open("out9.png").unwrap();
    for y in 0..100 {
        for x in 0..100 {
            let p = img.get_pixel(x, y);
            let pix = [p[0], p[1], p[2]];
            image.put_pixel((y) as u32, (x+200) as u32, Rgb(pix));
        }
    }

    let mut img = image::open("out10.png").unwrap();
    for y in 0..100 {
        for x in 0..100 {
            let p = img.get_pixel(x, y);
            let pix = [p[0], p[1], p[2]];
            image.put_pixel((y+100) as u32, (x+200) as u32, Rgb(pix));
        }
    }

    let mut img = image::open("out11.png").unwrap();
    for y in 0..100 {
        for x in 0..100 {
            let p = img.get_pixel(x, y);
            let pix = [p[0], p[1], p[2]];
            image.put_pixel((y+200) as u32, (x+200) as u32, Rgb(pix));
        }
    }

    let mut img = image::open("out12.png").unwrap();
    for y in 0..100 {
        for x in 0..100 {
            let p = img.get_pixel(x, y);
            let pix = [p[0], p[1], p[2]];
            image.put_pixel((y+300) as u32, (x+200) as u32, Rgb(pix));
        }
    }

    let mut img = image::open("out13.png").unwrap();
    for y in 0..100 {
        for x in 0..100 {
            let p = img.get_pixel(x, y);
            let pix = [p[0], p[1], p[2]];
            image.put_pixel((y) as u32, (x+300) as u32, Rgb(pix));
        }
    }

    let mut img = image::open("out14.png").unwrap();
    for y in 0..100 {
        for x in 0..100 {
            let p = img.get_pixel(x, y);
            let pix = [p[0], p[1], p[2]];
            image.put_pixel((y+100) as u32, (x+300) as u32, Rgb(pix));
        }
    }

    let mut img = image::open("out15.png").unwrap();
    for y in 0..100 {
        for x in 0..100 {
            let p = img.get_pixel(x, y);
            let pix = [p[0], p[1], p[2]];
            image.put_pixel((y+200) as u32, (x+300) as u32, Rgb(pix));
        }
    }

    let mut img = image::open("out16.png").unwrap();
    for y in 0..100 {
        for x in 0..100 {
            let p = img.get_pixel(x, y);
            let pix = [p[0], p[1], p[2]];
            image.put_pixel((y+300) as u32, (x+300) as u32, Rgb(pix));
        }
    }
    image.save("finalmente.png");

}
