use std::ops::{Add, Div, Mul, Sub};

#[derive(Clone, Copy, Debug)]
pub struct Color {
    pub r: f64,
    pub g: f64,
    pub b: f64,
}

impl Color {
    pub fn new(r: f64, g: f64, b: f64) -> Color {
        Color { r: r, g: g, b: b }
    }

    pub fn all(v: f64) -> Color {
        Color { r: v, g: v, b: v }
    }

    pub fn zero() -> Color {
        Color { r: 0.0, g: 0.0, b: 0.0 }
    }
}

impl Add for Color {
    type Output = Color;

    fn add(self, oc: Color) -> Color {
        Color {
            r: self.r + oc.r,
            g: self.g + oc.g,
            b: self.b + oc.b,
        }
    }
}

impl Div for Color {
    type Output = Color;

    fn div(self, oc: Color) -> Color {
        Color {
            r: self.r / oc.r,
            g: self.g / oc.g,
            b: self.b / oc.b,
        }
    }
}

impl Div<f64> for Color {
    type Output = Color;

    fn div(self, k: f64) -> Color {
        Color {
            r: self.r / k,
            g: self.g / k,
            b: self.b / k,
        }
    }
}

impl Mul for Color {
    type Output = Color;

    fn mul(self, oc: Color) -> Color {
        Color {
            r: self.r * oc.r,
            g: self.g * oc.g,
            b: self.b * oc.b,
        }
    }
}

impl Mul<f64> for Color {
    type Output = Color;

    fn mul(self, k: f64) -> Color {
        Color {
            r: self.r * k,
            g: self.g * k,
            b: self.b * k,
        }
    }
}

impl Sub for Color {
    type Output = Color;

    fn sub(self, oc: Color) -> Color {
        Color {
            r: self.r - oc.r,
            g: self.g - oc.g,
            b: self.b - oc.b,
        }
    }
}
