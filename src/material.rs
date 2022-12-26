use crate::color::Color;
use crate::bmp::Bmp;
use std::cmp::Ordering;

#[derive(Debug, Clone, PartialEq)]
pub struct Material {
    pub color: Color,
    pub absorb_color: Color,
    pub diff: f64,
    pub spec: f64,
    pub refl: f64,
    pub refr: f64,
    pub rindex: f64,
    pub m_texture: Bmp,
    pub use_m_texture: bool,
    // m_texture_func - do after texture funcs implemented
    // use_m_texture_func - do after texture funcs implemented
}

impl Material {
    pub fn hasTexture(self) -> bool {
        self.use_m_texture
    }

    pub fn setTexture(&mut self, texture: Bmp) -> () {
        self.use_m_texture = true;
        self.m_texture = texture;
    }

    pub fn getTextureColor(&self, u: f64, v:f64) -> Color {
        if self.use_m_texture {
            return self.m_texture.getColor(u, v);
        }else{
            return Color {r: 1.0, g: 1.0, b: 1.0};
        }
    }

    pub fn compare(self, other: Material) -> std::cmp::Ordering {
        if self.refl + 0.000001 < other.refl || ((self.refl - other.refl).abs() < 0.000001 && self.refr + 0.000001 < other.refr) {
            return Ordering::Greater;
        }else {
            return Ordering::Less;
        }
    }

}
