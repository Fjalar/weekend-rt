use std::sync::Arc;

use rand::Rng;

use crate::{color::Color, ray::Ray, texture::Texture, vec3::Vec3};

#[derive(Debug)]
pub(crate) enum Material {
    // albedo
    Lambertian(Arc<Texture>),

    // albedo, fuzz
    Metal(Color, f32),

    // index of refraction relative to surrounding media (air or enclosing primitive)
    Dielectric(f32),
}

impl Material {
    pub(crate) fn scatter<T: Rng>(
        &self,
        rng: &mut T,
        ray: Ray,
        t: f32,
        normal: Vec3,
        front_face: bool,
    ) -> (Ray, Color) {
        match self {
            Material::Lambertian(tex) => {
                let mut scatter_direction = normal + Vec3::random_unit_vector(rng);

                if scatter_direction.near_zero() {
                    scatter_direction = normal
                }

                (
                    Ray::new(ray.at(t), scatter_direction),
                    tex.sample(0.0, 0.0, ray.at(t)),
                )
            }
            Material::Metal(albedo, fuzz) => {
                let reflected = ray.direction.reflect(normal);
                let reflected_fuzzed = reflected.unit() + (*fuzz * Vec3::random_unit_vector(rng));

                (Ray::new(ray.at(t), reflected_fuzzed), *albedo)
            }
            Material::Dielectric(ior) => {
                let color = Color::new(1.0, 1.0, 1.0);

                let refraction_index = if front_face { 1.0 / *ior } else { *ior };

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
    }
}
