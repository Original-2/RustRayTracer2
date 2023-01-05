use std::cmp::Ordering;

use crate::bmp::Bmp;
use crate::color::Color;

#[derive(Debug, Clone, PartialEq)]
pub struct Material {
    pub color: Color,
    pub absorb_color: Color,
    pub diff: f64,
    pub spec: f64,
    pub reflection: f64,
    pub refraction: f64,
    pub r_index: f64,
    pub m_texture: Option<Bmp>,
    pub use_m_texture: bool,
    // m_texture_func - do after texture funcs implemented
    // use_m_texture_func - do after texture funcs implemented
}

impl Material {

    pub fn has_texture(&self) -> bool {
        self.use_m_texture
    }

    pub fn set_texture(&mut self, texture: Bmp) -> () {
        self.use_m_texture = true;
        self.m_texture = Some(texture);
    }

    pub fn get_texture_color(&self, u: f64, v: f64) -> Color {
        return if self.use_m_texture {
            self.m_texture.as_ref().unwrap().get_color(u, v)
        } else {
            self.color
        }
    }

    pub fn compare(&self, other: Material) -> Ordering {
        return if self.reflection + 0.000001 < other.reflection || ((self.reflection - other.reflection).abs() < 0.000001 && self.refraction + 0.000001 < other.refraction) {
            Ordering::Greater
        } else {
            Ordering::Less
        }
    }

}
