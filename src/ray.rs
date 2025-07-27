use crate::point::Point;
use crate::vec3::Vec3;

#[derive(Clone, Copy, Debug)]
pub(crate) struct Ray {
    pub(crate) origin: Point,
    pub(crate) direction: Vec3,
}

impl Ray {
    pub(crate) fn new(origin: Point, direction: Vec3) -> Ray {
        Ray { origin, direction }
    }

    pub(crate) fn at(&self, distance: f32) -> Point {
        self.origin + self.direction * distance
    }
}
