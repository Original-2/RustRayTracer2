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

    let mut image = ImageBuffer::<Rgb<u8>, Vec<u8>>::new(400 as u32,400 as u32);

    let mut img = image::open("out1.png").unwrap();
    for y in 0..100 {
        for x in 0..100 {
            let p = img.get_pixel(x, y);
            let pix = [p[0], p[1], p[2]];
            image.put_pixel((x) as u32, (y) as u32, Rgb(pix));
        }
    }

    let mut img = image::open("out2.png").unwrap();
    for y in 0..100 {
        for x in 0..100 {
            let p = img.get_pixel(x, y);
            let pix = [p[0], p[1], p[2]];
            image.put_pixel((x+100) as u32, (y) as u32, Rgb(pix));
        }
    }

    let mut img = image::open("out3.png").unwrap();
    for y in 0..100 {
        for x in 0..100 {
            let p = img.get_pixel(x, y);
            let pix = [p[0], p[1], p[2]];
            image.put_pixel((x+200) as u32, (y) as u32, Rgb(pix));
        }
    }

    let mut img = image::open("out4.png").unwrap();
    for y in 0..100 {
        for x in 0..100 {
            let p = img.get_pixel(x, y);
            let pix = [p[0], p[1], p[2]];
            image.put_pixel((x+300) as u32, (y) as u32, Rgb(pix));
        }
    }

    let mut img = image::open("out5.png").unwrap();
    for y in 0..100 {
        for x in 0..100 {
            let p = img.get_pixel(x, y);
            let pix = [p[0], p[1], p[2]];
            image.put_pixel((x) as u32, (y+100) as u32, Rgb(pix));
        }
    }

    let mut img = image::open("out6.png").unwrap();
    for y in 0..100 {
        for x in 0..100 {
            let p = img.get_pixel(x, y);
            let pix = [p[0], p[1], p[2]];
            image.put_pixel((x+100) as u32, (y+100) as u32, Rgb(pix));
        }
    }

    let mut img = image::open("out7.png").unwrap();
    for y in 0..100 {
        for x in 0..100 {
            let p = img.get_pixel(x, y);
            let pix = [p[0], p[1], p[2]];
            image.put_pixel((x+200) as u32, (y+100) as u32, Rgb(pix));
        }
    }

    let mut img = image::open("out8.png").unwrap();
    for y in 0..100 {
        for x in 0..100 {
            let p = img.get_pixel(x, y);
            let pix = [p[0], p[1], p[2]];
            image.put_pixel((x+300) as u32, (y+100) as u32, Rgb(pix));
        }
    }

    let mut img = image::open("out9.png").unwrap();
    for y in 0..100 {
        for x in 0..100 {
            let p = img.get_pixel(x, y);
            let pix = [p[0], p[1], p[2]];
            image.put_pixel((x) as u32, (y+200) as u32, Rgb(pix));
        }
    }

    let mut img = image::open("out10.png").unwrap();
    for y in 0..100 {
        for x in 0..100 {
            let p = img.get_pixel(x, y);
            let pix = [p[0], p[1], p[2]];
            image.put_pixel((x+100) as u32, (y+200) as u32, Rgb(pix));
        }
    }

    let mut img = image::open("out11.png").unwrap();
    for y in 0..100 {
        for x in 0..100 {
            let p = img.get_pixel(x, y);
            let pix = [p[0], p[1], p[2]];
            image.put_pixel((x+200) as u32, (y+200) as u32, Rgb(pix));
        }
    }

    let mut img = image::open("out12.png").unwrap();
    for y in 0..100 {
        for x in 0..100 {
            let p = img.get_pixel(x, y);
            let pix = [p[0], p[1], p[2]];
            image.put_pixel((x+300) as u32, (y+200) as u32, Rgb(pix));
        }
    }

    let mut img = image::open("out13.png").unwrap();
    for y in 0..100 {
        for x in 0..100 {
            let p = img.get_pixel(x, y);
            let pix = [p[0], p[1], p[2]];
            image.put_pixel((x) as u32, (y+300) as u32, Rgb(pix));
        }
    }

    let mut img = image::open("out14.png").unwrap();
    for y in 0..100 {
        for x in 0..100 {
            let p = img.get_pixel(x, y);
            let pix = [p[0], p[1], p[2]];
            image.put_pixel((x+100) as u32, (y+300) as u32, Rgb(pix));
        }
    }

    let mut img = image::open("out15.png").unwrap();
    for y in 0..100 {
        for x in 0..100 {
            let p = img.get_pixel(x, y);
            let pix = [p[0], p[1], p[2]];
            image.put_pixel((x+200) as u32, (y+300) as u32, Rgb(pix));
        }
    }

    let mut img = image::open("out16.png").unwrap();
    for y in 0..100 {
        for x in 0..100 {
            let p = img.get_pixel(x, y);
            let pix = [p[0], p[1], p[2]];
            image.put_pixel((x+300) as u32, (y+300) as u32, Rgb(pix));
        }
    }
    image.save("finalmente.png");

}
