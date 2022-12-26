use crate::color::Color;
use crate::bmp::Bmp;
use crate::vector3::Vector3;

#[derive(Clone)]
pub struct Camera {
    pub m_eye: Vector3,
    pub m_dir: Vector3,
    pub m_up: Vector3,
    pub m_w: i32,
    pub m_h: i32,
    pub m_fovy: f64,
    pub m_dist: f64,
    pub m_dw: Vector3, // m_dw = (m_dh * m_dir).unitize() * tan(m_fovy) * m_dist * (1.0 * m_w / m_h);
    pub m_dh: Vector3, // m_dh = m_up * tan(m_fovy) * -m_dist;
    pub m_film: Bmp, // m_film = new Bmp(m_w, m_h);
}

impl Camera {
    pub fn getW(&self) -> i32 {
        self.m_w
    }
    pub fn getH(&self) -> i32 {
        self.m_h
    }
    pub fn getEye(&self) -> Vector3 {
        self.m_eye
    }

    pub fn emit(&self, x: i32, y: i32) -> Vector3 {
        self.m_dir * self.m_dist
        + self.m_dw * (2.0 * (x as f64) / (self.m_w as f64) - 1.0)
        + self.m_dh * (2.0 * (y as f64) / (self.m_h as f64) - 1.0)
    }

    pub fn setColor(&mut self, x: i32, y: i32, color: Color) -> () {
        self.m_film.setColor(x, y, color);
    }

    pub fn print(&self) -> () {
        self.clone().m_film.save(String::from("hi.bmp"));
    }
}
