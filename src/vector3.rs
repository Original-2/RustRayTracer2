use std::ops::{Add, Sub, Div, Neg, Mul};

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Vector3 {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

impl Add for Vector3 {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self {x: self.x + other.x, y: self.y + other.y, z: self.z + other.z}
    }
}

impl Sub for Vector3 {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        Self {x: self.x - other.x, y: self.y - other.y, z: self.z - other.z}
    }
}

impl Mul for Vector3 {
    type Output = Self;

    fn mul(self, other: Self) -> Self {
        Self {x: self.y * other.z - self.z * other.y,
            y: self.z * other.x - self.x * other.z,
            z: self.x * other.y - self.y * other.x}
    }
}

impl Mul<f64> for Vector3 {
    type Output = Self;

    fn mul(self, other: f64) -> Self {
        Self {x: self.x * other, y: self.y * other, z: self.z * other}
    }

}

impl Div<f64> for Vector3 {
    type Output = Self;

    fn div(self, other: f64) -> Self {
        Self {x: self.x / other, y: self.y / other, z: self.z / other}
    }

}

impl Neg for Vector3 {
    type Output = Self;

    fn neg(self) -> Self {
        Self {x: -self.x, y: -self.y, z: -self.z}
    }
}

impl Vector3 {
    pub fn dot(&self, a: Vector3) -> f64 {
        a.x * self.x + a.y * self.y + a.z * self.z
    }

    pub fn mod1(&self) -> f64 {
        (self.x * self.x + self.y * self.y + self.z * self.z).sqrt()
    }

    pub fn mod2(&self) -> f64 {
        self.x * self.x + self.y * self.y + self.z * self.z
    }

    pub fn unitise(&self) -> Vector3{
        let len = &self.mod1();

        if len < &0.000001 {
            return *self;
        }else {
            return *self / *len;
        }
    }

    pub fn reflect(&self, n: Vector3) -> Vector3 {
        *self - n * (2.0 * self.dot(n))
    }

    pub fn refract (&self, n: Vector3, rindex: f64) -> Vector3{
        let ni = 1.0 / rindex;
        let cosi = self.dot(n);
        let cosT2 = 1.0 - ni*ni * (1.0 - cosi*cosi);
        if cosT2 < 0.000001{
            return Vector3 {x:0.0, y:0.0, z:0.0};
        } else {
            return *self * ni - n * (cosT2.sqrt() + cosi * ni);
        }
    }
}
