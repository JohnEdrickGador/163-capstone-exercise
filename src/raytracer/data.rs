use crate::primitives::*;
use crate::geometry::*;

#[derive(Debug, Clone, Copy)]
pub struct IntersectData {
    pub index: usize,
    pub coords: Point3,
    pub shape: ShapeType,
}

pub struct Scene {
    pub img_width: usize,
    pub img_height: usize,
    pub max_recurse_depth: usize,

    pub camera: Camera,
    pub shapes: Shapes,
    pub vertices: VertexStack,
    pub lights: LightStack,
}

impl IntersectData {
    pub fn new() -> Self {
        Self {
            index: 0,
            coords: Point3::new_empty(),
            shape: ShapeType::None,
        }
    }
}

impl Scene {
    pub fn new() -> Self {
        Self {
            img_width: 0,
            img_height: 0,
            max_recurse_depth: 5,
            camera: Camera::new(),
            shapes: Shapes::new(),
            vertices: VertexStack::new(),
            lights: LightStack::new(),
        }
    }
}