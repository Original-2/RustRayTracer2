use std::ops::{Add, AddAssign, Div, Mul, MulAssign, Sub, SubAssign};

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Color {
    pub r: f64,
    pub g: f64,
    pub b: f64,
}

impl Add for Color {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self { r: self.r + other.r, g: self.g + other.g, b: self.b + other.b }
    }
}

impl Sub for Color {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        Self { r: self.r - other.r, g: self.g - other.g, b: self.b - other.b }
    }
}

impl Mul for Color {
    type Output = Self;

    fn mul(self, other: Self) -> Self {
        Self { r: self.r * other.r, g: self.g * other.g, b: self.b * other.b }
    }
}

impl Mul<f64> for Color {
    type Output = Self;

    fn mul(self, other: f64) -> Self {
        Self { r: self.r * other, g: self.g * other, b: self.b * other }
    }
}

impl Mul<i32> for Color {
    type Output = Self;

    fn mul(self, other: i32) -> Self {
        Self { r: self.r * (other as f64), g: self.g * (other as f64), b: self.b * (other as f64) }
    }
}

impl Div<f64> for Color {
    type Output = Self;

    fn div(self, other: f64) -> Self {
        Self { r: self.r / other, g: self.g / other, b: self.b / other }
    }
}

impl MulAssign<f64> for Color {
    fn mul_assign(&mut self, other: f64) {
        self.r = self.r * other;
        self.g = self.g * other;
        self.b = self.b * other;
    }
}

impl AddAssign<f64> for Color {
    fn add_assign(&mut self, other: f64) {
        self.r = self.r + other;
        self.g = self.g + other;
        self.b = self.b + other;
    }
}

impl AddAssign for Color {
    fn add_assign(&mut self, other: Self) {
        self.r = self.r + other.r;
        self.g = self.g + other.g;
        self.b = self.b + other.b;
    }
}

impl SubAssign<f64> for Color {
    fn sub_assign(&mut self, other: f64) {
        self.r = self.r - other;
        self.g = self.g - other;
        self.b = self.b - other;
    }
}


impl Color {
    pub fn exp(&self) -> Color {
        Color { r: self.r.exp(), g: self.g.exp(), b: self.b.exp() }
    }

    pub fn clamp(&self) -> Color {
        Color {
            r: self.r.max(0.0).min(1.0),
            g: self.g.max(0.0).min(1.0),
            b: self.b.max(0.0).min(1.0),
        }
    }
}
