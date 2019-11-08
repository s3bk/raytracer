use crate::math::Vector3;
use crate::scene::Object;

#[derive(Debug)]
pub struct Ray {
    pub start: Vector3,
    pub direction: Vector3,
}

#[derive(Debug)]
pub struct Hit<'a> {
    pub position: Vector3,
    pub normal: Vector3,
    pub object: &'a dyn Object,
}

impl Hit<'_> {
    pub fn out_angle(&self, in_angle: Vector3) -> Vector3 {
        in_angle - (2.0 * in_angle.dot(self.normal) * self.normal)
    }
}
