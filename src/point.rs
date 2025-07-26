use crate::vec3::Vec3;

pub(crate) struct Point(pub(crate) Vec3);

impl std::ops::Deref for Point {
    type Target = Vec3;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl std::ops::DerefMut for Point {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
