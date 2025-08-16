use std::{f32::consts::PI, sync::Arc};

use crate::{
    aabb::Aabb, hittable::HitRecord, interval::Interval, material::Material, point::Point,
    ray::Ray, vec3::Vec3,
};

#[derive(Debug)]
pub(crate) struct SphereParams {
    pub(crate) center: Point,
    pub(crate) radius: f32,
    pub(crate) material: Arc<Material>,
    pub(crate) aabb: Aabb,
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

                let (u, v) = Self::get_sphere_uv(Point::from(outward_normal));

                Some(HitRecord::new(
                    ray,
                    root,
                    u,
                    v,
                    outward_normal,
                    params.material.clone(),
                ))
            }
        }
    }

    pub(crate) fn bounding_box(&self) -> &Aabb {
        match self {
            Primitive::Sphere(params) => params.bounding_box(),
        }
    }

    fn get_sphere_uv(p: Point) -> (f32, f32) {
        // p: a given point on the sphere of radius one, centered at the origin.
        // u: returned value [0,1] of angle around the Y axis from X=-1.
        // v: returned value [0,1] of angle from Y=-1 to Y=+1.
        //     <1 0 0> yields <0.50 0.50>       <-1  0  0> yields <0.00 0.50>
        //     <0 1 0> yields <0.50 1.00>       < 0 -1  0> yields <0.50 0.00>
        //     <0 0 1> yields <0.25 0.50>       < 0  0 -1> yields <0.75 0.50>

        let theta = f32::acos(-p.y);
        let phi = (-p.z).atan2(p.x) + PI;

        (phi / (2.0 * PI), theta / PI)
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
            aabb: Aabb::new_between(center - radius_vector, center + radius_vector),
        }
    }

    fn bounding_box(&self) -> &Aabb {
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
