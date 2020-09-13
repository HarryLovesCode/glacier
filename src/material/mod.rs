use geometry::{intersect, Sphere};
use math::{Ray, Vector};
use rand::random;
use std::f64::consts::PI;

pub use self::color::Color;
pub use self::material::Material;

mod color;
mod material;

#[derive(Clone, Copy, Debug)]
pub enum MaterialType {
    DIFFUSE,
    MIRROR,
    GLASS,
}

pub fn radiance(ray: Ray, spheres: &[Sphere]) -> Color {
    let total_col = Color::zero();
    let total_ref = Color::all(1.0);
    let ray = ray;

    let (t, id) = match intersect(ray, spheres) {
        Some((t, id)) => (t, id),
        None => return total_col,
    };

    let obj = spheres[id];

    let point = ray.at(t);
    let normal = (point - obj.pos).norm();
    let new_norm = if normal.dot(ray.dir) < 0.0 {
        normal
    } else {
        normal * -1.0
    };

    return Color::new(new_norm.x, new_norm.y,new_norm.z);
}
