extern crate image;
extern crate rand;

mod geometry;
mod material;
mod math;

use geometry::*;
use material::*;
use math::*;
use rand::random;
use std::path::Path;
use std::sync::mpsc;
use std::thread;

const NUM_THREADS: usize = 4;

fn main() {
    let w = 512;
    let h = 512;
    let w_f = w as f64;
    let h_f = h as f64;
    let samps = 20;
    let chunk_h = h / NUM_THREADS;
    let cam = Ray::new(Point::new(50.0, 52.0, 295.6), Vector::new(0.0, -0.042612, -1.0).norm());
    let cx = Vector::new(w as f64 * 0.5135 / h as f64, 0.0, 0.0);
    let cy = cx.cross(cam.dir).norm() * 0.5135;
    let (tx, rx) = mpsc::channel();
    let mut out = Vec::new();

    let materials = [
        Material::new(Color::zero(),    Color::new(0.75, 0.25, 0.25), MaterialType::DIFFUSE),
        Material::new(Color::zero(),    Color::new(0.25, 0.25, 0.75), MaterialType::DIFFUSE),
        Material::new(Color::zero(),    Color::all(0.75),             MaterialType::DIFFUSE),
        Material::new(Color::zero(),    Color::zero(),                MaterialType::DIFFUSE),
        Material::new(Color::zero(),    Color::all(0.999),            MaterialType::MIRROR),
        Material::new(Color::zero(),    Color::all(0.999),            MaterialType::GLASS),
        Material::new(Color::all(12.0), Color::zero(),                MaterialType::DIFFUSE),
    ];

    let spheres = [
        Sphere::new(100_000.0, Point::new(100_001.0, 40.8, 81.6), materials[0]),
        Sphere::new(100_000.0, Point::new(-99_901.0, 40.8, 81.6), materials[1]),
        Sphere::new(100_000.0, Point::new(50.0, 40.8, 100_000.0), materials[2]),
        Sphere::new(100_000.0, Point::new(50.0, 40.8, -99_830.0), materials[3]),
        Sphere::new(100_000.0, Point::new(50.0, 100_000.0, 81.6), materials[2]),
        Sphere::new(100_000.0, Point::new(50.0, -99_918.4, 81.6), materials[2]),
        Sphere::new(16.5,      Point::new(27.0, 16.5, 47.0),      materials[4]),
        Sphere::new(16.5,      Point::new(73.0, 16.5, 78.0),      materials[5]),
        Sphere::new(600.0,     Point::new(50.0, 681.33, 81.6),    materials[6]),
    ];

    for t in 0..NUM_THREADS {
        let tx = tx.clone();

        thread::spawn(move || {
            let mut data = Vec::with_capacity(3 * w * chunk_h);
            let bot = chunk_h * t;
            let top = chunk_h * (t + 1);
            for y in bot..top {
                let y_f = y as f64;

                for x in 0..w {
                    let x_f = x as f64;
                    let mut accum = Color::zero();

                    for sy in 0..2 {
                        for sx in 0..2 {
                            let sx = sx as f64;
                            let sy = sy as f64;
                            for _ in 0..samps {
                                let r1 = 2.0 * random::<f64>();
                                let r2 = 2.0 * random::<f64>();
                                let dx = if r1 < 1.0 { r1.sqrt() - 1.0 } else { 1.0 - (2.0 - r1).sqrt() };
                                let dy = if r2 < 1.0 { r2.sqrt() - 1.0 } else { 1.0 - (2.0 - r2).sqrt() };
                                let d = cx * (((sx + 0.5 + dx) / 2.0 + x_f) / w_f - 0.5) +
                                        cy * (((sy + 0.5 + dy) / 2.0 + y_f) / h_f - 0.5) + cam.dir;
                                let ray = Ray::new(cam.ori + d * 140.0, d.norm());
                                accum = accum + radiance(ray, 0, &spheres);
                            }
                        }
                    }

                    accum = accum * (0.25 / samps as f64);
                    data.push(to_int(accum.r));
                    data.push(to_int(accum.g));
                    data.push(to_int(accum.b));
                }
            }

            let _ = tx.send((bot * w * 3, data));
        });
    }

    for _ in 0..NUM_THREADS {
        let (s_idx, data) = rx.recv().unwrap();

        println!("{}", s_idx);

        for (idx, val) in data.iter().enumerate() {
            out.insert(s_idx + idx - 1, *val);
        }
    }

    let w = w as u32;
    let h = h as u32;
    let _ = image::save_buffer(&Path::new("smallpt.png"), &out, w, h, image::RGB(8));
}
