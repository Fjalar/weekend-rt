use crate::{axis::Axis, interval::Interval, point::Point};

#[derive(Clone, Copy, Debug)]
pub(crate) struct Aabb {
    pub(crate) x: Interval,
    pub(crate) y: Interval,
    pub(crate) z: Interval,
}

impl Aabb {
    #[allow(dead_code)]
    pub(crate) fn new(x: Interval, y: Interval, z: Interval) -> Self {
        Aabb { x, y, z }
    }

    pub(crate) fn empty() -> Self {
        Aabb {
            x: Interval::empty(),
            y: Interval::empty(),
            z: Interval::empty(),
        }
    }

    #[allow(dead_code)]
    pub(crate) fn new_from_point(p: Point) -> Aabb {
        Aabb {
            x: Interval::new(p.x, p.x),
            y: Interval::new(p.y, p.y),
            z: Interval::new(p.z, p.z),
        }
    }

    pub(crate) fn expand(&mut self, box1: &Aabb) {
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
        Aabb {
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

    #[allow(dead_code)]
    pub(crate) fn longest_axis(&self) -> Axis {
        let xy = self.x.size() > self.y.size();
        let yz = self.y.size() > self.z.size();
        let zx = self.z.size() > self.x.size();

        if xy {
            if zx { Axis::Z } else { Axis::X }
        } else if yz {
            Axis::Y
        } else {
            Axis::Z
        }
    }

    pub(crate) fn half_area(&self) -> f32 {
        let (x, y, z) = (self.x.size(), self.y.size(), self.z.size());
        x * y + y * z + z * x
    }

    pub(crate) fn hit(&self, ray: crate::ray::Ray, ray_interval: Interval) -> bool {
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
                return false;
            }
        }

        true
    }
}
