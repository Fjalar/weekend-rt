use std::ops;

use crate::vec3::Vec3;

pub(crate) struct Color(pub(crate) Vec3);

impl std::fmt::Display for Color {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let rbyte = (255.999 * self.0.x) as u8;
        let gbyte = (255.999 * self.0.y) as u8;
        let bbyte = (255.999 * self.0.z) as u8;

        write!(f, "{rbyte} {gbyte} {bbyte}",)
    }
}

impl std::ops::Deref for Color {
    type Target = Vec3;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl std::ops::DerefMut for Color {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

#[allow(dead_code)]
impl Color {
    pub(crate) const fn new(x: f32, y: f32, z: f32) -> Self {
        Color(Vec3::new(x, y, z))
    }
}

impl ops::Add<Color> for Color {
    type Output = Color;

    fn add(self, rhs: Color) -> Self::Output {
        Color::new(self.x + rhs.x, self.y + rhs.y, self.z + rhs.z)
    }
}

impl ops::Sub<Color> for Color {
    type Output = Color;

    fn sub(self, rhs: Color) -> Self::Output {
        Color::new(self.x - rhs.x, self.y - rhs.y, self.z - rhs.z)
    }
}

impl<T> ops::Mul<T> for Color
where
    T: Into<f32> + Copy,
{
    type Output = Color;

    fn mul(self, rhs: T) -> Self::Output {
        Color::new(
            self.x * rhs.into(),
            self.y * rhs.into(),
            self.z * rhs.into(),
        )
    }
}

impl<T> ops::MulAssign<T> for Color
where
    T: Into<f32> + Copy,
{
    fn mul_assign(&mut self, rhs: T) {
        self.x *= rhs.into();
        self.y *= rhs.into();
        self.z *= rhs.into();
    }
}

impl<T> ops::Div<T> for Color
where
    T: Into<f32> + Copy,
{
    type Output = Color;

    fn div(self, rhs: T) -> Self::Output {
        Color::new(
            self.x / rhs.into(),
            self.y / rhs.into(),
            self.z / rhs.into(),
        )
    }
}

impl<T> ops::DivAssign<T> for Color
where
    T: Into<f32> + Copy,
{
    fn div_assign(&mut self, rhs: T) {
        self.x /= rhs.into();
        self.y /= rhs.into();
        self.z /= rhs.into();
    }
}
