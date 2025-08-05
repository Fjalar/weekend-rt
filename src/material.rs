use rand::Rng;
use rand_chacha::ChaCha8Rng;

use crate::{color::Color, ray::Ray, vec3::Vec3};

pub(crate) trait Material: std::fmt::Debug + Send + Sync {
    fn scatter(
        &self,
        rng: &mut ChaCha8Rng,
        ray: Ray,
        t: f32,
        normal: Vec3,
        front_face: bool,
    ) -> (Ray, Color);
}

#[derive(Debug)]
pub(crate) struct Lambertian {
    pub(crate) albedo: Color,
}

impl Material for Lambertian {
    fn scatter(
        &self,
        rng: &mut ChaCha8Rng,
        ray: Ray,
        t: f32,
        normal: Vec3,
        _: bool,
    ) -> (Ray, Color) {
        let mut scatter_direction = normal + Vec3::random_unit_vector(rng);

        if scatter_direction.near_zero() {
            scatter_direction = normal
        }

        (Ray::new(ray.at(t), scatter_direction), self.albedo)
    }
}

#[derive(Debug)]
pub(crate) struct Metal {
    pub(crate) albedo: Color,
    pub(crate) fuzz: f32,
}

impl Material for Metal {
    fn scatter(
        &self,
        rng: &mut ChaCha8Rng,
        ray: Ray,
        t: f32,
        normal: Vec3,
        _: bool,
    ) -> (Ray, Color) {
        let reflected = ray.direction.reflect(normal);
        let reflected_fuzzed = reflected.unit() + (self.fuzz * Vec3::random_unit_vector(rng));

        (Ray::new(ray.at(t), reflected_fuzzed), self.albedo)
    }
}

#[derive(Debug)]
pub(crate) struct Dielectric {
    pub(crate) refraction_index: f32,
}

impl Material for Dielectric {
    fn scatter(
        &self,
        rng: &mut ChaCha8Rng,
        ray: Ray,
        t: f32,
        normal: Vec3,
        front_face: bool,
    ) -> (Ray, Color) {
        let color = Color::new(1.0, 1.0, 1.0);

        let refraction_index = if front_face {
            1.0 / self.refraction_index
        } else {
            self.refraction_index
        };

        let unit_direction = ray.direction.unit();

        let cos_theta = f32::min((-unit_direction).dot(normal), 1.0);
        let sin_theta = (1.0 - cos_theta * cos_theta).sqrt();

        let cannot_refract = refraction_index * sin_theta > 1.0;

        // Schlick approximation
        let reflectance = {
            let r0 = (1.0 - refraction_index) / (1.0 + refraction_index);
            let r0 = r0 * r0;
            r0 + (1.0 - r0) * (1.0 - cos_theta).powi(10)
        };

        let direction = if cannot_refract || reflectance > rng.random_range(0.0..1.0) {
            unit_direction.reflect(normal)
        } else {
            unit_direction.refract(normal, refraction_index)
        };

        let scattered = Ray::new(ray.at(t), direction);

        (scattered, color)
    }
}
