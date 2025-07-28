use crate::{
    hittable::{HitRecord, Hittable},
    point::Point,
    ray::Ray,
};

pub(crate) struct Sphere {
    pub(crate) center: Point,
    pub(crate) radius: f32,
}

impl Sphere {
    pub(crate) fn new(center: Point, radius: f32) -> Sphere {
        Sphere {
            center,
            radius: radius.max(0.0),
        }
    }
}

impl Hittable for Sphere {
    fn hit(&self, ray: Ray, ray_tmin: f32, ray_tmax: f32, hit_record: &mut HitRecord) -> bool {
        let ray_to_sphere = self.center - ray.origin;
        let a = ray.direction.length_squared();
        let h = ray.direction.dot(ray_to_sphere);
        let c = ray_to_sphere.length_squared() - self.radius * self.radius;
        let discriminant = h * h - a * c;

        if discriminant < 0.0 {
            return false;
        }
        let sqrt_d = discriminant.sqrt();

        // Find nearest root that lies in the acceptable range of ray_tmin..ray_tmax
        let root = (h - sqrt_d) / a;
        if root <= ray_tmin || ray_tmax <= root {
            let root = (h + sqrt_d) / a;
            if root <= ray_tmin || ray_tmax <= root {
                return false;
            }
        }
        hit_record.t = root;
        hit_record.position = ray.at(hit_record.t);
        let outward_normal = (hit_record.position - self.center) / self.radius;
        hit_record.set_face_normal(ray, outward_normal);

        true
    }
}
