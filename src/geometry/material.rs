use crate::primitives::*;

pub type RGBColor = Vector3;

#[derive(Debug, Clone, Copy)]
pub struct Material {
    pub ambient: RGBColor,
    pub diffuse: RGBColor,
    pub specular: RGBColor,
    pub emission: RGBColor,

    pub shininess: f64,
}

impl Material {
    pub fn new() -> Self {
        Self {
            ambient: Vector3::new_with_value(0.2),
            diffuse: Vector3::new_empty(),
            specular: Vector3::new_empty(),
            emission: Vector3::new_empty(),
            shininess: 0.0,
        }
    }
}