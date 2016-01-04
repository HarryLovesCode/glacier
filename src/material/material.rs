use material::{Color, MaterialType};

#[derive(Clone, Copy, Debug)]
pub struct Material {
    pub emis: Color,
    pub base: Color,
    pub refl: MaterialType,
}

impl Material {
    pub fn new(emis: Color, base: Color, refl: MaterialType) -> Material {
        Material { emis: emis, base: base, refl: refl }
    }
}
