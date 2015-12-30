use math::Ray;

pub use self::sphere::Sphere;

mod sphere;

pub const INFINITY: f64 = 1.0e20;
pub const EPSILON: f64 = 1.0e-4;

pub trait Geometry {
    fn intersect(self, ray: Ray) -> Option<f64>;
}

pub fn intersect(ray: Ray, spheres: &[Sphere]) -> Option<(f64, usize)> {
    let mut t_out = INFINITY;
    let mut id_out = 0;

    for (id, sphere) in spheres.iter().enumerate() {
        let t = sphere.intersect(ray);

        if t.is_some() {
            let t = t.unwrap();

            if t < t_out {
                t_out = t;
                id_out = id;
            }
        }
    }

    if t_out < INFINITY {
        Some((t_out, id_out))
    } else {
        None
    }
}
