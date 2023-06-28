use super::*;

#[derive(Clone, Debug, Copy)]
pub struct Ray {
    pub position: Point3,
    pub direction: Vector3,
}

impl Ray {
    pub fn new() -> Self {
        Self {
            position: Point3::new_empty(),
            direction: Vector3::new_empty(),
        }
    }

    pub fn from_origin(direction: &Vector3) -> Self {
        Self {
            position: Point3::new_empty(),
            direction: *direction,
        }
    }

    pub fn no_direction(position: &Point3) -> Self {
        Self {
            position: *position,
            direction: Vector3::new_empty(),
        }
    }
}