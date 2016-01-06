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
    let mut depth = 0;
    let mut total_col = Color::zero();
    let mut total_ref = Color::all(1.0);
    let mut ray = ray;

    loop {
        let (t, id) = match intersect(ray, spheres) {
            Some((t, id)) => (t, id),
            None => return total_col,
        };

        let obj = spheres[id];
        let mut base = obj.mat.base;
        let max_refl = base.r.max(base.g.max(base.b));

        total_col = total_col + total_ref * obj.mat.emis;

        if depth > 5 {
            if random::<f64>() < max_refl {
                base = base / max_refl;
            } else {
                return total_col;
            }
        }

        total_ref = total_ref * base;

        let point = ray.at(t);
        let normal = (point - obj.pos).norm();
        let new_norm = if normal.dot(ray.dir) < 0.0 { normal } else { normal * -1.0 };

        match obj.mat.refl {
            MaterialType::DIFFUSE => {
                let r1 = 2.0 * PI * random::<f64>();
                let r2 = random::<f64>();
                let r2s = r2.sqrt();
                let w = new_norm;
                let u = if w.x.abs() > 0.1 {
                    Vector::new(0.0, 1.0, 0.0)
                } else {
                    Vector::new(1.0, 0.0, 0.0)
                }.cross(w).norm();
                let v = w.cross(u);
                let d = (u * r1.cos() * r2s + v * r1.sin() * r2s + w * (1.0 - r2).sqrt()).norm();

                ray = Ray::new(point, d);
            },
            MaterialType::MIRROR => {
                ray = Ray::new(point, ray.dir - normal * 2.0 * normal.dot(ray.dir));
            },
            MaterialType::GLASS => {
                let refl_ray = Ray::new(point, ray.dir - normal * 2.0 * normal.dot(ray.dir));
                let nc = 1.0;
                let nt = 1.5;
                let into = normal.dot(new_norm) > 0.0;
                let nnt = if into { nc / nt } else { nt / nc };
                let ddn = ray.dir.dot(new_norm);
                let cos2t = 1.0 - nnt * nnt * (1.0 - ddn * ddn);

                if cos2t < 0.0 {
                    ray = refl_ray;
                }

                let if_into = if into { 1.0 } else { -1.0 };
                let t_dir = (ray.dir * nnt - normal * (if_into * (ddn * nnt + cos2t.sqrt()))).norm();
                let a = nt - nc;
                let b = nt + nc;
                let r0 = a * a / (b * b);
                let if_into = if into { -ddn } else { t_dir.dot(normal) };
                let c = 1.0 - if_into;
                let re = r0 + (1.0 - r0) * c * c * c * c * c;
                let tr = 1.0 - re;
                let p = 0.25 + 0.5 * re;
                let rp = re / p;
                let tp = tr / (1.0 - p);


                if random::<f64>() < p {
                    total_ref = total_ref * rp;
                    ray = refl_ray;
                } else {
                    total_ref = total_ref * tp;
                    ray = Ray::new(point, t_dir);
                }
            },
        }

        depth += 1;
    }
}
