extern crate byteorder;

use std::fs;
use std::io::{Cursor, Read};

use byteorder::{LittleEndian, ReadBytesExt};

use crate::color::Color;


const ENABLE_TEXTURE_FILTERING: bool = true;

#[derive(Debug, Copy, Clone, PartialEq)]
struct BITMAPFILEHEADER
{
    bfType: u16,
    bfSize: u32,
    bfReserved1: u16,
    bfReserved2: u16,
    bfOffBits: u32,
}

#[derive(Debug, Copy, Clone, PartialEq)]
struct BITMAPINFOHEADER
{
    biSize: u32,
    biWidth: u32,
    biHeight: u32,
    biPlanes: u16,
    biBitCount: u16,
    biCompression: u32,
    biSizeImage: u32,
    biXPelsPerMeter: u32,
    biYPelsPerMeter: u32,
    biClrUsed: u32,
    biClrImportant: u32,
}

#[derive(Debug, Copy, Clone, PartialEq)]
struct BmpColor
{
    r: u8,
    g: u8,
    b: u8,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Bmp {
    m_w: i32,
    m_h: i32,
    m_bf: BITMAPFILEHEADER,
    m_bi: BITMAPINFOHEADER,
    m_data: Vec<Vec<BmpColor>>,
}

pub fn BmpNew(w: i32, h: i32, background: Color) -> Bmp {

    let mut bg_full: Vec<Vec<BmpColor>> = Vec::new();

    for _i in 0..w {
        let mut bg_row: Vec<BmpColor> = Vec::new();
        for _j in 0..h {
            bg_row.push(BmpColor {
                r: (background.r * 255.0 + 0.5) as u8,
                g: (background.g * 255.0 + 0.5) as u8,
                b: (background.r * 255.0 + 0.5) as u8,
            })
        }
        bg_full.push(bg_row);
    }

    return Bmp {
        m_w: w,
        m_h: h,
        m_bf: BITMAPFILEHEADER {
            bfType: 0x4D42,
            bfSize: ((w * 3 + w % 4) * h + 54) as u32,
            bfOffBits: 54,
            bfReserved1: 0,
            bfReserved2: 0,
        },
        m_bi: BITMAPINFOHEADER {
            biSize: 40,
            biWidth: w as u32,
            biHeight: h as u32,
            biPlanes: 1,
            biBitCount: 24,
            biCompression: 0,
            biSizeImage: 0,
            biXPelsPerMeter: 0,
            biYPelsPerMeter: 0,
            biClrUsed: 0,
            biClrImportant: 0,
        },
        m_data: bg_full,
    };
}

pub fn bmp_from_file(filename: String) -> Bmp {
    let mut f = fs::File::open(filename).unwrap();

    let mut bytes = Vec::new();
    f.read_to_end(&mut bytes);

    let mut bmp_data = Cursor::new(bytes);

    let m_bf_from_str = BITMAPFILEHEADER {
        bfType: bmp_data.read_u16::<LittleEndian>().unwrap(),
        bfSize: bmp_data.read_u32::<LittleEndian>().unwrap(),
        bfOffBits: bmp_data.read_u32::<LittleEndian>().unwrap(),
        bfReserved1: bmp_data.read_u16::<LittleEndian>().unwrap(),
        bfReserved2: bmp_data.read_u16::<LittleEndian>().unwrap(),
    };

    let m_bi_from_str = BITMAPINFOHEADER {
        biSize: bmp_data.read_u32::<LittleEndian>().unwrap(),
        biWidth: bmp_data.read_u32::<LittleEndian>().unwrap(),
        biHeight: bmp_data.read_u32::<LittleEndian>().unwrap(),
        biPlanes: bmp_data.read_u16::<LittleEndian>().unwrap(),
        biBitCount: bmp_data.read_u16::<LittleEndian>().unwrap(),
        biCompression: bmp_data.read_u32::<LittleEndian>().unwrap(),
        biSizeImage: bmp_data.read_u32::<LittleEndian>().unwrap(),
        biXPelsPerMeter: bmp_data.read_u32::<LittleEndian>().unwrap(),
        biYPelsPerMeter: bmp_data.read_u32::<LittleEndian>().unwrap(),
        biClrUsed: bmp_data.read_u32::<LittleEndian>().unwrap(),
        biClrImportant: bmp_data.read_u32::<LittleEndian>().unwrap(),
    };

    let w = m_bi_from_str.biWidth;
    let h = m_bi_from_str.biHeight;

    let mut bg_full: Vec<Vec<BmpColor>> = Vec::new();

    for _i in 0..w {
        let mut bg_row: Vec<BmpColor> = Vec::new();
        for _j in 0..h {
            let mut px = vec![0, 0, 0];
            bmp_data.read(&mut px).expect("Unable to read pixel");

            bg_row.push(BmpColor {
                r: px[2] as u8,
                g: px[1] as u8,
                b: px[0] as u8,
            })
        }
        bg_full.push(bg_row);
    }

    return Bmp {
        m_w: w as i32,
        m_h: h as i32,
        m_bf: m_bf_from_str,
        m_bi: m_bi_from_str,
        m_data: bg_full,
    };
}


impl Bmp {
    fn get_w(self) -> i32 {
        return self.m_w;
    }
    fn get_h(self) -> i32 {
        return self.m_h;
    }
    fn get_color_int(&self, x: i32, y: i32) -> Color {
        return Color {
            r: self.m_data[x as usize][y as usize].r as f64 / 255.0,
            g: self.m_data[x as usize][y as usize].g as f64 / 255.0,
            b: self.m_data[x as usize][y as usize].b as f64 / 255.0,
        };
    }

    pub fn get_color(&self, uinp: f64, vinp: f64) -> Color {
        let u = uinp * (self.m_w - 1) as f64;
        let v = vinp * (self.m_h - 1) as f64;


        if ENABLE_TEXTURE_FILTERING {
            let mut u1: i32 = (u + 0.000001).floor() as i32;
            let mut v1: i32 = (u + 0.000001).floor() as i32;
            let mut u2: i32 = u1 + 1;
            let mut v2: i32 = v1 + 1;


            let ru: i32 = u2 - (u as i32);
            let rv: i32 = v2 - (v as i32);

            if u1 < 0 {
                u1 = self.m_w - 1;
            }

            if v1 < 0 {
                v1 = self.m_h - 1;
            }

            if u2 == self.m_w {
                u2 = 0;
            }

            if v2 == self.m_h {
                v2 = 0;
            }

            return self.get_color_int(u1, v1) * (ru * rv)
                + self.get_color_int(u1, v2) * (ru * (1 - rv))
                + self.get_color_int(u2, v1) * ((1 - ru) * rv)
                + self.get_color_int(u2, v2) * ((1 - ru) * (1 - rv));
        } else {
            return self.get_color_int((u + 0.5) as i32, (v + 0.5) as i32);
        }
    }

    pub fn set_color(&mut self, x: i32, y: i32, color: Color) -> () {
        self.m_data[x as usize][y as usize] = BmpColor {
            r: (color.r * 255.0 + 0.5) as u8,
            g: (color.g * 255.0 + 0.5) as u8,
            b: (color.r * 255.0 + 0.5) as u8,
        }
    }
}
