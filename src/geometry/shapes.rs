use crate::primitives::*;
use super::material::Material;

#[derive(Debug, Clone, Copy)]
pub enum ShapeType {
    Triangle(Triangle),
    Sphere(Sphere),
    None,
}

#[derive(Debug, Clone, Copy)]
pub struct Triangle {
    pub vertices: [Point3; 3],
    pub transform: Matrix4,
    pub material: Material,
}

#[derive(Debug, Clone, Copy)]
pub struct Sphere {
    pub radius: f64,
    pub center: Point3,
    pub transform: Matrix4,
    pub material: Material,
}

#[derive(Debug, Clone)]
pub struct VertexStack(pub Vec <Point3>);

#[derive(Debug, Clone)]
pub struct Shapes(pub Vec <ShapeType>);

impl VertexStack {
    pub fn new() -> Self {
        Self(Vec::new())
    }
}

impl Shapes {
    pub fn new() -> Self {
        Self(Vec::new())
    }
}

impl Sphere {
    pub fn new() -> Self {
        Self {
            radius: 0.0,
            center: Point3::new_empty(),
            transform: Matrix4::new_on_diag(1.0),
            material: Material::new(),
        }
    }
}

impl Triangle {
    pub fn new() -> Self {
        Self {
            vertices: [Point3::new_empty(), Point3::new_empty(), Point3::new_empty()],
            transform: Matrix4::new_on_diag(1.0),
            material: Material::new(),
        }
    }
}