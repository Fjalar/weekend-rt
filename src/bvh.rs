use std::sync::Arc;

use crate::{aabb::Aabb, hittable::HitRecord, interval::Interval, primitive::Primitive};

#[derive(Debug)]
pub(crate) struct BVHNode {
    left: Option<Arc<BVHNode>>,
    right: Option<Arc<BVHNode>>,
    start_idx: usize,
    primitive_count: usize,
    aabb: Aabb,
}

impl BVHNode {
    pub(crate) fn new(objects: &mut Vec<Primitive>, start: usize, end: usize) -> Arc<Self> {
        let mut bounding_box = Aabb::empty();

        for idx in start..end {
            if let Some(obj) = objects.get(idx) {
                bounding_box.expand(obj.bounding_box());
            }
        }

        let longest_axis = bounding_box.longest_axis();

        let key_lambda = |a: &Primitive, b: &Primitive| {
            a.bounding_box()
                .axis_interval(longest_axis)
                .min
                .total_cmp(&b.bounding_box().axis_interval(longest_axis).min)
        };

        let object_span = end - start;

        let (left, right, start_idx, primitive_count) = if object_span == 1 {
            (None, None, start, 1usize)
        } else if object_span == 2 {
            (None, None, start, 2usize)
        } else {
            objects.as_mut_slice()[start..end].sort_by(key_lambda);

            let mid = start + object_span / 2;

            let local_left = BVHNode::new(objects, start, mid);
            let local_right = BVHNode::new(objects, mid, end);

            bounding_box.expand(local_left.bounding_box());
            bounding_box.expand(local_right.bounding_box());

            (Some(local_left), Some(local_right), 0usize, 0usize)
        };

        // let mut aabb = *left.bounding_box();
        // aabb.expand(right.bounding_box());

        Arc::new(BVHNode {
            left,
            right,
            start_idx,
            primitive_count,
            aabb: bounding_box,
        })
    }

    pub(crate) fn hit(
        &self,
        ray: crate::ray::Ray,
        ray_interval: crate::interval::Interval,
        world: &Arc<Vec<Primitive>>,
    ) -> Option<crate::hittable::HitRecord> {
        if self.aabb.hit(ray, ray_interval) {
            if let (Some(left), Some(right)) = (&self.left, &self.right) {
                let hit_left = left.hit(ray, ray_interval, world);
                let hit_right = right.hit(ray, ray_interval, world);

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
            } else {
                let world_slice =
                    &world.as_slice()[self.start_idx..(self.start_idx + self.primitive_count)];

                let mut potential_hit: Option<HitRecord> = None;
                let mut closest_so_far = ray_interval.max;

                for object in world_slice.iter() {
                    if let Some(hit) =
                        object.hit(ray, Interval::new(ray_interval.min, closest_so_far))
                    {
                        closest_so_far = hit.t;
                        potential_hit = Some(hit);
                    }
                }

                return potential_hit;
            }
        }
        None
    }

    fn bounding_box(&self) -> &crate::aabb::Aabb {
        &self.aabb
    }
}
