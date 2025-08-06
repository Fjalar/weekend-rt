use std::sync::Arc;

use crate::{
    aabb::AABB, hittable::HitRecord, interval::Interval, material::Material, point::Point,
    ray::Ray, vec3::Vec3,
};

#[derive(Debug)]
pub(crate) struct SphereParams {
    pub(crate) center: Point,
    pub(crate) radius: f32,
    pub(crate) material: Arc<Material>,
    pub(crate) aabb: AABB,
}

#[derive(Debug)]
pub(crate) enum Primitive {
    Sphere(SphereParams),
}

impl Primitive {
    pub(crate) fn hit(&self, ray: Ray, ray_inteval: Interval) -> Option<HitRecord> {
        match self {
            Primitive::Sphere(params) => {
                let ray_to_sphere = params.center - ray.origin;
                let a = ray.direction.length_squared();
                let h = ray.direction.dot(ray_to_sphere);
                let c = ray_to_sphere.length_squared() - params.radius * params.radius;
                let discriminant = h * h - a * c;

                if discriminant < 0.0 {
                    return None;
                }
                let sqrt_d = discriminant.sqrt();

                // Find nearest root that lies in the acceptable range of ray_tmin..ray_tmax
                let mut root = (h - sqrt_d) / a;
                if !ray_inteval.surrounds(root) {
                    root = (h + sqrt_d) / a;
                    if !ray_inteval.surrounds(root) {
                        return None;
                    }
                }

                let position = ray.at(root);
                let outward_normal = (position - params.center) / params.radius;

                Some(HitRecord::new(
                    ray,
                    root,
                    outward_normal,
                    params.material.clone(),
                ))
            }
        }
    }

    pub(crate) fn bounding_box(&self) -> &AABB {
        match self {
            Primitive::Sphere(params) => params.bounding_box(),
        }
    }
}

impl SphereParams {
    pub(crate) fn new(center: Point, radius: f32, material: Arc<Material>) -> SphereParams {
        let radius = radius.max(0.0);
        let radius_vector = Vec3::new(radius, radius, radius);
        SphereParams {
            center,
            radius,
            material,
            aabb: AABB::new_between(center - radius_vector, center + radius_vector),
        }
    }

    fn bounding_box(&self) -> &AABB {
        &self.aabb
    }
}

// impl Hittable for Sphere {
//     fn hit(&self, ray: Ray, ray_inteval: Interval) -> Option<HitRecord> {
//         let ray_to_sphere = self.center - ray.origin;
//         let a = ray.direction.length_squared();
//         let h = ray.direction.dot(ray_to_sphere);
//         let c = ray_to_sphere.length_squared() - self.radius * self.radius;
//         let discriminant = h * h - a * c;

//         if discriminant < 0.0 {
//             return None;
//         }
//         let sqrt_d = discriminant.sqrt();

//         // Find nearest root that lies in the acceptable range of ray_tmin..ray_tmax
//         let mut root = (h - sqrt_d) / a;
//         if !ray_inteval.surrounds(root) {
//             root = (h + sqrt_d) / a;
//             if !ray_inteval.surrounds(root) {
//                 return None;
//             }
//         }

//         let position = ray.at(root);
//         let outward_normal = (position - self.center) / self.radius;

//         Some(HitRecord::new(
//             ray,
//             root,
//             outward_normal,
//             self.material.clone(),
//         ))
//     }
// }
