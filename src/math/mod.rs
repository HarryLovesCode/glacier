pub use self::point::Point;
pub use self::ray::Ray;
pub use self::vector::Vector;

mod point;
mod ray;
mod vector;

pub const TO_INT_POW: f64 = 1.0 / 2.2;

pub fn clamp(v: f64) -> f64 {
    if v < 0.0 {
        0.0
    } else if v > 1.0 {
        1.0
    } else {
        v
    }
}

pub fn to_int(v: f64) -> u8 {
    (clamp(v).powf(TO_INT_POW) * 255.0 + 0.5) as u8
}
