use math::Vector;
use std::ops::{Add, Div, Mul, Sub};

#[derive(Clone, Copy, Debug)]
pub struct Point {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

impl Point {
    pub fn new(x: f64, y: f64, z: f64) -> Point {
        Point { x: x, y: y, z: z }
    }

    pub fn all(v: f64) -> Point {
        Point { x: v, y: v, z: v }
    }

    pub fn origin() -> Point {
        Point { x: 0.0, y: 0.0, z: 0.0 }
    }
}

impl Add for Point {
    type Output = Point;

    fn add(self, op: Point) -> Point {
        Point {
            x: self.x + op.x,
            y: self.y + op.y,
            z: self.z + op.z,
        }
    }
}

impl Add<Vector> for Point {
    type Output = Point;

    fn add(self, ov: Vector) -> Point {
        Point {
            x: self.x + ov.x,
            y: self.y + ov.y,
            z: self.z + ov.z,
        }
    }
}

impl Div<f64> for Point {
    type Output = Point;

    fn div(self, k: f64) -> Point {
        Point {
            x: self.x / k,
            y: self.y / k,
            z: self.z / k,
        }
    }
}

impl Mul<f64> for Point {
    type Output = Point;

    fn mul(self, k: f64) -> Point {
        Point {
            x: self.x * k,
            y: self.y * k,
            z: self.z * k,
        }
    }
}

impl Sub for Point {
    type Output = Vector;

    fn sub(self, op: Point) -> Vector {
        Vector {
            x: self.x - op.x,
            y: self.y - op.y,
            z: self.z - op.z,
        }
    }
}

impl Sub<Vector> for Point {
    type Output = Point;

    fn sub(self, ov: Vector) -> Point {
        Point {
            x: self.x - ov.x,
            y: self.y - ov.y,
            z: self.z - ov.z,
        }
    }
}
