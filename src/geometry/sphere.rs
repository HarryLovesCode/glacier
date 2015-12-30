use geometry::{EPSILON, Geometry};
use material::Material;
use math::{Point, Ray};

#[derive(Clone, Copy, Debug)]
pub struct Sphere {
    pub pos: Point,
    pub mat: Material,
    pub rad2: f64,
}

impl Sphere {
    pub fn new(rad: f64, pos: Point, mat: Material) -> Sphere {
        Sphere { rad2: rad * rad, pos: pos, mat: mat }
    }
}

impl Geometry for Sphere {
    fn intersect(self, ray: Ray) -> Option<f64> {
        let op = self.pos - ray.ori;
        let b = op.dot(ray.dir);
        let mut det = b * b - op.mag2() + self.rad2;

        if det < 0.0 {
            return None;
        }

        det = det.sqrt();
        let t0 = b - det;
        let t1 = b + det;

        if t0 > EPSILON {
            Some(t0)
        } else if t1 > EPSILON {
            Some(t1)
        } else {
            None
        }
    }
}
