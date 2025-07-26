use std::fmt::Display;

use crate::vec3::Vec3;

pub(crate) struct Color(pub(crate) Vec3);

impl Display for Color {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let rbyte = (255.999 * self.0.x) as u8;
        let gbyte = (255.999 * self.0.y) as u8;
        let bbyte = (255.999 * self.0.z) as u8;

        write!(f, "{rbyte} {gbyte} {bbyte}",)
    }
}
