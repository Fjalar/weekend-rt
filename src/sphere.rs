use std::rc::Rc;

use crate::{
    aabb::AABB,
    hittable::{HitRecord, Hittable},
    interval::Interval,
    material::Material,
    point::Point,
    ray::Ray,
    vec3::Vec3,
};

#[derive(Debug)]
pub(crate) struct Sphere {
    pub(crate) center: Point,
    pub(crate) radius: f32,
    pub(crate) material: Rc<dyn Material>,
    pub(crate) aabb: AABB,
}

impl Sphere {
    pub(crate) fn new(center: Point, radius: f32, material: Rc<dyn Material>) -> Sphere {
        let radius = radius.max(0.0);
        let radius_vector = Vec3::new(radius, radius, radius);
        Sphere {
            center,
            radius,
            material,
            aabb: AABB::new_between(center - radius_vector, center + radius_vector),
        }
    }
}

impl Hittable for Sphere {
    fn hit(&self, ray: Ray, ray_inteval: Interval) -> Option<HitRecord> {
        let ray_to_sphere = self.center - ray.origin;
        let a = ray.direction.length_squared();
        let h = ray.direction.dot(ray_to_sphere);
        let c = ray_to_sphere.length_squared() - self.radius * self.radius;
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
        let outward_normal = (position - self.center) / self.radius;

        Some(HitRecord::new(
            ray,
            root,
            outward_normal,
            self.material.clone(),
        ))
    }

    fn bounding_box(&self) -> &AABB {
        &self.aabb
    }
}
