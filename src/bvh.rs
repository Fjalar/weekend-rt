use std::sync::Arc;

use crate::{aabb::AABB, hittable::HitRecord, primitive::Primitive};

#[derive(Debug)]
pub(crate) struct BVHNode {
    left: Arc<NodeOrPrim>,
    right: Arc<NodeOrPrim>,
    aabb: AABB,
}

#[derive(Debug)]
pub(crate) enum NodeOrPrim {
    Node(BVHNode),
    Prim(Primitive),
}

impl NodeOrPrim {
    pub(crate) fn hit(
        &self,
        ray: crate::ray::Ray,
        ray_interval: crate::interval::Interval,
    ) -> Option<HitRecord> {
        match self {
            NodeOrPrim::Node(node) => node.hit(ray, ray_interval),
            NodeOrPrim::Prim(prim) => prim.hit(ray, ray_interval),
        }
    }

    pub(crate) fn bounding_box(&self) -> &AABB {
        match self {
            NodeOrPrim::Node(node) => node.bounding_box(),
            NodeOrPrim::Prim(prim) => prim.bounding_box(),
        }
    }
}

impl BVHNode {
    pub(crate) fn new(objects: &mut Vec<Arc<NodeOrPrim>>, start: usize, end: usize) -> Self {
        let mut bounding_box = AABB::empty();

        for idx in start..end {
            if let Some(obj) = objects.get(idx) {
                bounding_box.expand(obj.bounding_box());
            }
        }

        let longest_axis = bounding_box.longest_axis();

        let key_lambda = |a: &Arc<NodeOrPrim>, b: &Arc<NodeOrPrim>| {
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

            let local_left: Arc<NodeOrPrim> =
                Arc::new(NodeOrPrim::Node(BVHNode::new(objects, start, mid)));
            let local_right: Arc<NodeOrPrim> =
                Arc::new(NodeOrPrim::Node(BVHNode::new(objects, mid, end)));

            (local_left, local_right)
        };

        let mut aabb = *left.bounding_box();
        aabb.expand(right.bounding_box());

        BVHNode { left, right, aabb }
    }

    pub(crate) fn hit(
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
