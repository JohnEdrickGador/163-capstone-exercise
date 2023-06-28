use crate::primitives::*;
use super::*;

pub enum LightType {
    Directional(DirectionalLight),
    Point(PointLight),
}

pub struct DirectionalLight {
    pub direction: Vector3,
    pub color: RGBColor,
}

pub struct PointLight {
    pub position: Point3,
    pub color: RGBColor,
}

pub struct LightStack {
    pub attenuation: [f64; 3],
    pub lights: Vec <LightType>,
}

impl LightStack {
    pub fn new() -> Self {
        Self {
            attenuation: [1.0, 0.0, 0.0],
            lights: Vec::new(),
        }
    }
}
