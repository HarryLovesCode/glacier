use std::ops::{Add, Div, Mul, Neg, Sub};

/// Vector is a standard vector in 3 dimensional space
#[derive(Clone, Copy, Debug)]
pub struct Vector {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

impl Vector {
    /// Initialize Vector and set values for { x, y, z } components
    pub fn new(x: f64, y: f64, z: f64) -> Vector {
        Vector { x: x, y: y, z: z }
    }

    /// Compute length of Vector
    pub fn len(self) -> f64 {
        self.len_sqr().sqrt()
    }

    /// Compute length of Vector squared
    pub fn len_sqr(self) -> f64 {
        self.x * self.x + self.y * self.y + self.z * self.z
    }

    /// Compute normalized copy of this
    pub fn norm(self) -> Vector {
        self / self.len()
    }

    /// Compute vector cross product of two vectors
    pub fn cross(self, ov: Vector) -> Vector {
        Vector {
            x: self.y * ov.z - self.z * ov.y,
            y: self.z * ov.x - self.x * ov.z,
            z: self.x * ov.y - self.y * ov.x,
        }
    }

    /// Compute dot product of two vectors
    pub fn dot(self, ov: Vector) -> f64 {
        self.x * ov.x + self.y * ov.y + self.z * ov.z
    }
}

impl Add for Vector {
    type Output = Vector;

    /// Compute sum of two vectors
    fn add(self, r: Vector) -> Vector {
        Vector {
            x: self.x + r.x,
            y: self.y + r.y,
            z: self.z + r.z,
        }
    }
}

impl Div<f64> for Vector {
    type Output = Vector;

    /// Compute this Vector scaled by 1 / k
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

    /// Compute this Vector scaled by k
    fn mul(self, k: f64) -> Vector {
        Vector {
            x: self.x * k,
            y: self.y * k,
            z: self.z * k,
        }
    }
}

impl Neg for Vector {
    type Output = Vector;

    /// Compute opposite of Vector
    fn neg(self) -> Vector {
        Vector {
            x: -self.x,
            y: -self.y,
            z: -self.z,
        }
    }
}

impl Sub for Vector {
    type Output = Vector;

    /// Compute difference of two vectors
    fn sub(self, r: Vector) -> Vector {
        Vector {
            x: self.x - r.x,
            y: self.y - r.y,
            z: self.z - r.z,
        }
    }
}

#[test]
fn test_add() {
    let v0 = Vector::new(1.0, 2.0, 3.0);
    let v1 = Vector::new(1.0, 2.0, 3.0);
    let s = v0 + v1;

    assert!(s.x == 2.0);
    assert!(s.y == 4.0);
    assert!(s.z == 6.0);
}

#[test]
fn test_div() {
    let v = Vector::new(2.0, 4.0, 6.0);
    let q = v / 2.0;

    assert!(q.x == 1.0);
    assert!(q.y == 2.0);
    assert!(q.z == 3.0);
}

#[test]
fn test_len() {
    let v = Vector::new(1.0, 2.0, 3.0);

    assert!(v.len() == (14.0f64).sqrt());
}

#[test]
fn test_len_sqr() {
    let v = Vector::new(1.0, 2.0, 3.0);
    
    assert!(v.len_sqr() == 1.0 + 4.0 + 9.0);
}

#[test]
fn test_mul() {
    let v = Vector::new(1.0, 2.0, 3.0);
    let p = v * 2.0;

    assert!(p.x == 2.0);
    assert!(p.y == 4.0);
    assert!(p.z == 6.0);
}

#[test]
fn test_norm() {
    let v = Vector::new(1.0, 2.0, 3.0);

    assert!(v.norm().len().round() == 1.0);
}

#[test]
fn test_sub() {
    let v0 = Vector::new(3.0, 2.0, 1.0);
    let v1 = Vector::new(1.0, 2.0, 3.0);
    let d = v0 - v1;

    assert!(d.x == 2.0);
    assert!(d.y == 0.0);
    assert!(d.z == -2.0);
}
