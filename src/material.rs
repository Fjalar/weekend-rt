use rand::rngs::ThreadRng;

use crate::{color::Color, ray::Ray, vec3::Vec3};

pub(crate) trait Material {
    fn scatter(
        &self,
        rng: &mut ThreadRng,
        ray: Ray,
        t: f32,
        normal: Vec3,
        front_face: bool,
    ) -> (Ray, Color);
}

pub(crate) struct Lambertian {
    pub(crate) albedo: Color,
}

impl Material for Lambertian {
    fn scatter(
        &self,
        rng: &mut ThreadRng,
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

pub(crate) struct Metal {
    pub(crate) albedo: Color,
    pub(crate) fuzz: f32,
}

impl Material for Metal {
    fn scatter(
        &self,
        rng: &mut ThreadRng,
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

pub(crate) struct Dielectric {
    pub(crate) refraction_index: f32,
}

impl Material for Dielectric {
    fn scatter(
        &self,
        _: &mut ThreadRng,
        ray: Ray,
        t: f32,
        normal: Vec3,
        front_face: bool,
    ) -> (Ray, Color) {
        let color = Color::new(1.0, 1.0, 1.0);

        // Inversion here is different from tutorial, why??
        let refraction_index = if !front_face {
            1.0 / self.refraction_index
        } else {
            self.refraction_index
        };

        let refracted = (ray.direction).refract(normal, refraction_index);

        let scattered = Ray::new(ray.at(t), refracted);

        (scattered, color)
    }
}
