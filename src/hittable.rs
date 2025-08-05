use std::rc::Rc;

use crate::{
    aabb::AABB, interval::Interval, material::Material, point::Point, ray::Ray, vec3::Vec3,
};

#[derive(Clone)]
pub(crate) struct HitRecord {
    #[allow(dead_code)]
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

pub(crate) trait Hittable: std::fmt::Debug {
    fn hit(&self, ray: Ray, ray_interval: Interval) -> Option<HitRecord>;

    fn bounding_box(&self) -> &AABB;
}

#[derive(Debug)]
pub(crate) struct HittableList {
    pub(crate) objects: Vec<Rc<dyn Hittable>>,
    pub(crate) aabb: AABB,
}

impl HittableList {
    pub(crate) fn new() -> HittableList {
        HittableList {
            objects: Vec::new(),
            aabb: AABB::new(Interval::empty(), Interval::empty(), Interval::empty()),
        }
    }

    #[allow(dead_code)]
    pub(crate) fn clear(&mut self) {
        self.objects.clear();
    }

    pub(crate) fn add(&mut self, object: Rc<dyn Hittable>) {
        self.aabb.expand(object.bounding_box());
        self.objects.push(object);
    }
}

impl Hittable for HittableList {
    fn hit(&self, ray: Ray, ray_interval: Interval) -> Option<HitRecord> {
        let mut potential_hit: Option<HitRecord> = None;
        let mut closest_so_far = ray_interval.max;

        for object in self.objects.iter() {
            if let Some(hit) = object.hit(ray, Interval::new(ray_interval.min, closest_so_far)) {
                closest_so_far = hit.t;
                potential_hit = Some(hit);
            }
        }

        potential_hit
    }

    fn bounding_box(&self) -> &AABB {
        &self.aabb
    }
}
