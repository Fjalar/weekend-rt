use std::sync::Arc;

use crate::{aabb::AABB, hittable::Hittable};

#[derive(Debug)]
pub(crate) struct BVHNode {
    left: Arc<dyn Hittable>,
    right: Arc<dyn Hittable>,
    aabb: AABB,
}

impl BVHNode {
    pub(crate) fn new(objects: &mut Vec<Arc<dyn Hittable>>, start: usize, end: usize) -> Self {
        let mut bounding_box = AABB::empty();

        for idx in start..end {
            if let Some(obj) = objects.get(idx) {
                bounding_box.expand(obj.bounding_box());
            }
        }

        let longest_axis = bounding_box.longest_axis();

        let key_lambda = |a: &Arc<dyn Hittable>, b: &Arc<dyn Hittable>| {
            a.bounding_box()
                .axis_interval(longest_axis)
                .min
                .total_cmp(&b.bounding_box().axis_interval(longest_axis).min)
        };

        let object_span = end - start;

        let (left, right) = if object_span == 1 {
            (objects[start].clone(), objects[start].clone())
        } else if object_span == 2 {
            (objects[start].clone(), objects[start + 1].clone())
        } else {
            objects.as_mut_slice()[start..end].sort_by(key_lambda);

            let mid = start + object_span / 2;

            let local_left: Arc<dyn Hittable> = Arc::new(BVHNode::new(objects, start, mid));
            let local_right: Arc<dyn Hittable> = Arc::new(BVHNode::new(objects, mid, end));

            (local_left, local_right)
        };

        let mut aabb = *left.bounding_box();
        aabb.expand(right.bounding_box());

        BVHNode { left, right, aabb }
    }
}

impl Hittable for BVHNode {
    fn hit(
        &self,
        ray: crate::ray::Ray,
        ray_interval: crate::interval::Interval,
    ) -> Option<crate::hittable::HitRecord> {
        if self.aabb.hit(ray, ray_interval).is_some() {
            let hit_left = self.left.hit(ray, ray_interval);
            let hit_right = self.right.hit(ray, ray_interval);

            if hit_left.is_some() && hit_right.is_none() {
                return hit_left;
            } else if hit_right.is_some() && hit_left.is_none() {
                return hit_right;
            }

            if let (Some(l), Some(r)) = (hit_left, hit_right) {
                if l.t > r.t {
                    return Some(r);
                } else {
                    return Some(l);
                }
            }
        }
        None
    }

    fn bounding_box(&self) -> &crate::aabb::AABB {
        &self.aabb
    }
}
