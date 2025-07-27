use std::ops;

use crate::color::Color;

#[derive(Clone, Copy, Debug)]
pub(crate) struct Vec3 {
    pub(crate) x: f32,
    pub(crate) y: f32,
    pub(crate) z: f32,
}

#[allow(dead_code)]
impl Vec3 {
    pub(crate) const fn new(x: f32, y: f32, z: f32) -> Self {
        Vec3 { x, y, z }
    }

    pub(crate) const fn length_squared(&self) -> f32 {
        self.x * self.x + self.y * self.y + self.z * self.z
    }

    pub(crate) fn length(&self) -> f32 {
        self.length_squared().sqrt()
    }

    pub(crate) const fn dot(&self, rhs: Vec3) -> f32 {
        self.x * rhs.x + self.y * rhs.y + self.z * rhs.z
    }

    pub(crate) const fn cross(&self, rhs: Vec3) -> Vec3 {
        Vec3 {
            x: self.y * rhs.z - self.z * rhs.y,
            y: self.z * rhs.x - self.x * rhs.z,
            z: self.x * rhs.y - self.y * rhs.x,
        }
    }

    pub(crate) fn unit(&self) -> Vec3 {
        *self / self.length()
    }
}

impl ops::Add<Vec3> for Vec3 {
    type Output = Vec3;

    fn add(self, rhs: Vec3) -> Self::Output {
        Vec3 {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
        }
    }
}

impl ops::Sub<Vec3> for Vec3 {
    type Output = Vec3;

    fn sub(self, rhs: Vec3) -> Self::Output {
        Vec3 {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z,
        }
    }
}

impl<T> ops::Mul<T> for Vec3
where
    T: Into<f32> + Copy,
{
    type Output = Vec3;

    fn mul(self, rhs: T) -> Self::Output {
        Vec3 {
            x: self.x * rhs.into(),
            y: self.y * rhs.into(),
            z: self.z * rhs.into(),
        }
    }
}

impl ops::Mul<Vec3> for f32 {
    type Output = Vec3;

    fn mul(self, rhs: Vec3) -> Self::Output {
        Vec3 {
            x: rhs.x * self,
            y: rhs.y * self,
            z: rhs.z * self,
        }
    }
}

impl<T> ops::MulAssign<T> for Vec3
where
    T: Into<f32> + Copy,
{
    fn mul_assign(&mut self, rhs: T) {
        self.x *= rhs.into();
        self.y *= rhs.into();
        self.z *= rhs.into();
    }
}

impl<T> ops::Div<T> for Vec3
where
    T: Into<f32> + Copy,
{
    type Output = Vec3;

    fn div(self, rhs: T) -> Self::Output {
        Vec3 {
            x: self.x / rhs.into(),
            y: self.y / rhs.into(),
            z: self.z / rhs.into(),
        }
    }
}

impl ops::Div<Vec3> for f32 {
    type Output = Vec3;

    fn div(self, rhs: Vec3) -> Self::Output {
        Vec3 {
            x: rhs.x / self,
            y: rhs.y / self,
            z: rhs.z / self,
        }
    }
}

impl<T> ops::DivAssign<T> for Vec3
where
    T: Into<f32> + Copy,
{
    fn div_assign(&mut self, rhs: T) {
        self.x /= rhs.into();
        self.y /= rhs.into();
        self.z /= rhs.into();
    }
}

impl From<Color> for Vec3 {
    fn from(value: Color) -> Self {
        Vec3::new(value.r, value.g, value.b)
    }
}
