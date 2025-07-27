use std::ops;

use crate::vec3::Vec3;

#[derive(Clone, Copy, Debug)]
pub(crate) struct Color {
    pub(crate) r: f32,
    pub(crate) g: f32,
    pub(crate) b: f32,
}

impl std::fmt::Display for Color {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let rbyte = (255.999 * self.r) as u8;
        let gbyte = (255.999 * self.g) as u8;
        let bbyte = (255.999 * self.b) as u8;

        write!(f, "{rbyte} {gbyte} {bbyte}",)
    }
}

impl Color {
    pub(crate) const fn new(r: f32, g: f32, b: f32) -> Self {
        Color { r, g, b }
    }
}

impl ops::Add<Color> for Color {
    type Output = Color;

    fn add(self, rhs: Color) -> Self::Output {
        Color::new(self.r + rhs.r, self.g + rhs.g, self.b + rhs.b)
    }
}

impl ops::Sub<Color> for Color {
    type Output = Color;

    fn sub(self, rhs: Color) -> Self::Output {
        Color::new(self.r - rhs.r, self.g - rhs.g, self.b - rhs.b)
    }
}

impl<T> ops::Mul<T> for Color
where
    T: Into<f32> + Copy,
{
    type Output = Color;

    fn mul(self, rhs: T) -> Self::Output {
        Color::new(
            self.r * rhs.into(),
            self.g * rhs.into(),
            self.b * rhs.into(),
        )
    }
}

impl ops::Mul<Color> for f32 {
    type Output = Color;

    fn mul(self, rhs: Color) -> Self::Output {
        Color {
            r: rhs.r * self,
            g: rhs.g * self,
            b: rhs.b * self,
        }
    }
}

impl<T> ops::MulAssign<T> for Color
where
    T: Into<f32> + Copy,
{
    fn mul_assign(&mut self, rhs: T) {
        self.r *= rhs.into();
        self.g *= rhs.into();
        self.b *= rhs.into();
    }
}

impl<T> ops::Div<T> for Color
where
    T: Into<f32> + Copy,
{
    type Output = Color;

    fn div(self, rhs: T) -> Self::Output {
        Color::new(
            self.r / rhs.into(),
            self.g / rhs.into(),
            self.b / rhs.into(),
        )
    }
}

impl<T> ops::DivAssign<T> for Color
where
    T: Into<f32> + Copy,
{
    fn div_assign(&mut self, rhs: T) {
        self.r /= rhs.into();
        self.g /= rhs.into();
        self.b /= rhs.into();
    }
}

impl ops::Div<Color> for f32 {
    type Output = Color;

    fn div(self, rhs: Color) -> Self::Output {
        Color {
            r: rhs.r / self,
            g: rhs.g / self,
            b: rhs.b / self,
        }
    }
}

impl From<Vec3> for Color {
    fn from(value: Vec3) -> Self {
        Color::new(value.x, value.y, value.z)
    }
}
