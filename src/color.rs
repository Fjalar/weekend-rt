use std::ops;

use crate::{interval::Interval, vec3::Vec3};

#[derive(Clone, Copy, Debug)]
pub(crate) struct Color {
    pub(crate) r: f32,
    pub(crate) g: f32,
    pub(crate) b: f32,
}

impl std::fmt::Display for Color {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        // Display outputs in decimal without gamma correction

        // Clamp to 0..255
        const INTENSITY: Interval = Interval::new(0.0, 0.999);

        let rbyte = (256.0 * INTENSITY.clamp(self.r)) as u8;
        let gbyte = (256.0 * INTENSITY.clamp(self.g)) as u8;
        let bbyte = (256.0 * INTENSITY.clamp(self.b)) as u8;

        write!(f, "Color({rbyte}, {gbyte}, {bbyte})")
    }
}

impl Color {
    pub(crate) const fn new(r: f32, g: f32, b: f32) -> Self {
        Color { r, g, b }
    }

    fn linear_to_gamma(linear_component: f32) -> f32 {
        if linear_component > 0.0 {
            linear_component.sqrt()
        } else {
            0.0
        }
    }

    pub(crate) fn bytes(&self) -> [u8; 3] {
        // Gamma correction
        let r_corrected = Self::linear_to_gamma(self.r);
        let g_corrected = Self::linear_to_gamma(self.g);
        let b_corrected = Self::linear_to_gamma(self.b);

        // Clamp to 0..255
        const INTENSITY: Interval = Interval::new(0.0, 0.999);
        let rbyte = (256.0 * INTENSITY.clamp(r_corrected)) as u8;
        let gbyte = (256.0 * INTENSITY.clamp(g_corrected)) as u8;
        let bbyte = (256.0 * INTENSITY.clamp(b_corrected)) as u8;
        [rbyte, gbyte, bbyte]
    }
}

impl ops::Add<Color> for Color {
    type Output = Color;

    fn add(self, rhs: Color) -> Self::Output {
        Color::new(self.r + rhs.r, self.g + rhs.g, self.b + rhs.b)
    }
}

impl ops::AddAssign for Color {
    fn add_assign(&mut self, rhs: Self) {
        self.r += rhs.r;
        self.g += rhs.g;
        self.b += rhs.b;
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

impl ops::Mul<Color> for Color {
    type Output = Color;

    fn mul(self, rhs: Color) -> Self::Output {
        Color::new(self.r * rhs.r, self.g * rhs.g, self.b * rhs.b)
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
