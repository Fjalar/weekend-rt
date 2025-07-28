use std::rc::Rc;

use crate::{interval::Interval, point::Point, ray::Ray, vec3::Vec3};

#[derive(Clone, Copy, Default)]
pub(crate) struct HitRecord {
    pub(crate) position: Point,
    pub(crate) normal: Vec3,
    pub(crate) t: f32,
    pub(crate) front_face: bool,
}

impl HitRecord {
    // Parameter outward_normal is assumed to be unit length
    pub(crate) fn set_face_normal(&mut self, ray: Ray, outward_normal: Vec3) {
        self.front_face = ray.direction.dot(outward_normal) < 0.0;
        self.normal = if self.front_face {
            outward_normal
        } else {
            -1.0 * outward_normal
        }
    }
}

pub(crate) trait Hittable {
    fn hit(&self, ray: Ray, ray_interval: Interval, hit_record: &mut HitRecord) -> bool;
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
    fn hit(&self, ray: Ray, ray_interval: Interval, hit_record: &mut HitRecord) -> bool {
        let mut temp_record = HitRecord::default();
        let mut hit_anything = false;
        let mut closest_so_far = ray_interval.max;

        for object in self.0.iter() {
            if object.hit(
                ray,
                Interval::new(ray_interval.min, closest_so_far),
                &mut temp_record,
            ) {
                hit_anything = true;
                closest_so_far = temp_record.t;
                *hit_record = temp_record;
            }
        }

        hit_anything
    }
}
