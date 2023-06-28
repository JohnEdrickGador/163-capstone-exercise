use super::*;

#[derive(Clone, Debug, Copy)]
pub struct Camera {
    pub eye: Vector3,
    pub center: Vector3,
    pub up: Vector3,

    pub fovy: f64,
}

impl Camera {
    pub fn new() -> Self {
        Self {
            eye: Vector3::new_empty(),
            center: Vector3::new_empty(),
            up: Vector3::new_empty(),
            fovy: 0.0,
        }
    }
}