use std::ops::{Add, Div, Mul, Sub};

#[derive(Clone, Copy, Debug)]
pub struct Vector {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

impl Vector {
    pub fn new(x: f64, y: f64, z: f64) -> Vector {
        Vector { x: x, y: y, z: z }
    }

    pub fn all(v: f64) -> Vector {
        Vector { x: v, y: v, z: v }
    }

    pub fn zero() -> Vector {
        Vector { x: 0.0, y: 0.0, z: 0.0 }
    }

    pub fn cross(self, ov: Vector) -> Vector {
        Vector {
            x: self.y * ov.z - self.z * ov.y,
            y: self.z * ov.x - self.x * ov.z,
            z: self.x * ov.y - self.y * ov.x,
        }
    }

    pub fn dot(self, ov: Vector) -> f64 {
        self.x * ov.x + self.y * ov.y + self.z * ov.z
    }

    pub fn mag(self) -> f64 {
        (self.x * self.x + self.y * self.y + self.z * self.z).sqrt()
    }

    pub fn mag2(self) -> f64 {
        self.x * self.x + self.y * self.y + self.z * self.z
    }

    pub fn norm(self) -> Vector {
        self / self.mag()
    }
}

impl Add for Vector {
    type Output = Vector;

    fn add(self, ov: Vector) -> Vector {
        Vector {
            x: self.x + ov.x,
            y: self.y + ov.y,
            z: self.z + ov.z,
        }
    }
}

impl Div<f64> for Vector {
    type Output = Vector;

    fn div(self, k: f64) -> Vector {
        Vector {
            x: self.x / k,
            y: self.y / k,
            z: self.z / k,
        }
    }
}

impl Mul<f64> for Vector {
    type Output = Vector;

    fn mul(self, k: f64) -> Vector {
        Vector {
            x: self.x * k,
            y: self.y * k,
            z: self.z * k,
        }
    }
}

impl Sub for Vector {
    type Output = Vector;

    fn sub(self, ov: Vector) -> Vector {
        Vector {
            x: self.x - ov.x,
            y: self.y - ov.y,
            z: self.z - ov.z,
        }
    }
}
