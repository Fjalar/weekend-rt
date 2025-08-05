use std::{rc::Rc, sync::Arc};

use crate::{
    axis::Axis,
    color::Color,
    hittable::{HitRecord, Hittable},
    interval::Interval,
    material::Lambertian,
    point::Point,
    vec3::Vec3,
};

#[derive(Clone, Copy, Debug)]
pub(crate) struct AABB {
    pub(crate) x: Interval,
    pub(crate) y: Interval,
    pub(crate) z: Interval,
}

impl AABB {
    pub(crate) fn new(x: Interval, y: Interval, z: Interval) -> Self {
        AABB { x, y, z }
    }

    pub(crate) fn empty() -> Self {
        AABB {
            x: Interval::empty(),
            y: Interval::empty(),
            z: Interval::empty(),
        }
    }

    pub(crate) fn new_from_point(p: Point) -> AABB {
        AABB {
            x: Interval::new(p.x, p.x),
            y: Interval::new(p.y, p.y),
            z: Interval::new(p.z, p.z),
        }
    }

    pub(crate) fn expand(&mut self, box1: &AABB) {
        self.x = Interval::tight(self.x, box1.x);
        self.y = Interval::tight(self.y, box1.y);
        self.z = Interval::tight(self.z, box1.z);
    }

    pub(crate) fn axis_interval(&self, axis: Axis) -> Interval {
        match axis {
            Axis::X => self.x,
            Axis::Y => self.y,
            Axis::Z => self.z,
        }
    }

    pub(crate) fn new_between(a: Point, b: Point) -> Self {
        AABB {
            x: if a.x <= b.x {
                Interval::new(a.x, b.x)
            } else {
                Interval::new(b.x, a.x)
            },

            y: if a.y <= b.y {
                Interval::new(a.y, b.y)
            } else {
                Interval::new(b.y, a.y)
            },

            z: if a.z <= b.z {
                Interval::new(a.z, b.z)
            } else {
                Interval::new(b.z, a.z)
            },
        }
    }

    pub(crate) fn longest_axis(&self) -> Axis {
        let xy = self.x.size() > self.y.size();
        let yz = self.y.size() > self.z.size();
        let zx = self.z.size() > self.x.size();

        if xy {
            if zx {
                return Axis::Z;
            } else {
                return Axis::X;
            }
        } else if yz {
            return Axis::Y;
        } else {
            return Axis::Z;
        }
    }
}

impl Hittable for AABB {
    fn hit(&self, ray: crate::ray::Ray, ray_interval: Interval) -> Option<HitRecord> {
        let mut ray_interval_min = ray_interval.min;
        let mut ray_interval_max = ray_interval.max;
        for &axis in Axis::iter() {
            let ray_direction_axis = ray.direction.axis(axis);
            let ray_origin_axis = ray.origin.axis(axis);

            let axis_interval = self.axis_interval(axis);
            let ad_inv = 1.0 / ray_direction_axis;

            let t0 = (axis_interval.min - ray_origin_axis) * ad_inv;
            let t1 = (axis_interval.max - ray_origin_axis) * ad_inv;

            if t0 < t1 {
                if t0 > ray_interval_min {
                    ray_interval_min = t0;
                }
                if t1 < ray_interval_max {
                    ray_interval_max = t1;
                }
            } else {
                if t1 > ray_interval_min {
                    ray_interval_min = t1;
                }
                if t0 < ray_interval_max {
                    ray_interval_max = t0;
                }
            }

            if ray_interval_max <= ray_interval_min {
                return None;
            }
        }

        // Placeholder Hitrecord, just need to know if it hit or not really.
        // Warrants restructuring
        Some(HitRecord {
            position: Point::new(0.0, 0.0, 0.0),
            normal: Vec3::new(0.0, 0.0, 0.0),
            material: Arc::new(Lambertian {
                albedo: Color::new(0.0, 0.0, 0.0),
            }),
            t: ray_interval_max,
            front_face: true,
        })
    }

    fn bounding_box(&self) -> &AABB {
        self
    }
}
