use std::rc::Rc;

use crate::{interval::Interval, material::Material, point::Point, ray::Ray, vec3::Vec3};

#[derive(Clone)]
pub(crate) struct HitRecord {
    pub(crate) position: Point,
    pub(crate) normal: Vec3,
    pub(crate) material: Rc<dyn Material>,
    pub(crate) t: f32,
    pub(crate) front_face: bool,
}

impl HitRecord {
    pub(crate) fn new(ray: Ray, t: f32, outward_normal: Vec3, material: Rc<dyn Material>) -> Self {
        let position = ray.at(t);

        let front_face = ray.direction.dot(outward_normal) < 0.0;
        let normal = if front_face {
            outward_normal
        } else {
            -outward_normal
        };

        HitRecord {
            position,
            normal,
            material,
            t,
            front_face,
        }
    }
}

pub(crate) trait Hittable {
    fn hit(&self, ray: Ray, ray_interval: Interval) -> Option<HitRecord>;
}

pub(crate) struct HittableList(pub(crate) Vec<Rc<dyn Hittable>>);

impl HittableList {
    pub(crate) fn new() -> HittableList {
        HittableList(Vec::new())
    }

    #[allow(dead_code)]
    pub(crate) fn clear(&mut self) {
        self.0.clear();
    }

    pub(crate) fn add(&mut self, object: Rc<dyn Hittable>) {
        self.0.push(object);
    }
}

impl Hittable for HittableList {
    fn hit(&self, ray: Ray, ray_interval: Interval) -> Option<HitRecord> {
        let mut potential_hit: Option<HitRecord> = None;
        let mut closest_so_far = ray_interval.max;

        for object in self.0.iter() {
            if let Some(hit) = object.hit(ray, Interval::new(ray_interval.min, closest_so_far)) {
                closest_so_far = hit.t;
                potential_hit = Some(hit);
            }
        }

        potential_hit
    }
}
