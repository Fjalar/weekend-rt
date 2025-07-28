use std::ops;

use crate::vec3::Vec3;

#[derive(Clone, Copy, Debug, Default)]
pub(crate) struct Point {
    pub(crate) x: f32,
    pub(crate) y: f32,
    pub(crate) z: f32,
}

#[allow(dead_code)]
impl Point {
    pub(crate) const fn new(x: f32, y: f32, z: f32) -> Self {
        Point { x, y, z }
    }
}

impl ops::Add<Vec3> for Point {
    type Output = Point;

    fn add(self, rhs: Vec3) -> Self::Output {
        Point {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
        }
    }
}

impl ops::Sub<Vec3> for Point {
    type Output = Point;

    fn sub(self, rhs: Vec3) -> Self::Output {
        Point {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z,
        }
    }
}

impl ops::Sub<Point> for Point {
    type Output = Vec3;

    fn sub(self, rhs: Point) -> Self::Output {
        Vec3 {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z,
        }
    }
}

impl From<Vec3> for Point {
    fn from(value: Vec3) -> Self {
        Point {
            x: value.x,
            y: value.y,
            z: value.z,
        }
    }
}
