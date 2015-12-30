use math::{Point, Vector};

#[derive(Clone, Copy, Debug)]
pub struct Ray {
    pub ori: Point,
    pub dir: Vector,
}

impl Ray {
    pub fn new(ori: Point, dir: Vector) -> Ray {
        Ray { ori: ori, dir: dir }
    }

    pub fn at(self, t: f64) -> Point {
        self.ori + self.dir * t
    }
}
