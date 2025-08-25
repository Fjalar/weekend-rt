use std::sync::Arc;

use crate::{
    aabb::Aabb, axis::Axis, hittable::HitRecord, interval::Interval, primitive::Primitive, ray::Ray,
};

#[derive(Debug)]
pub(crate) struct Bvh {
    nodes: Vec<BvhNode>,
}

impl Bvh {
    pub(crate) fn new(objects: &mut Vec<Primitive>) -> Bvh {
        let mut nodes = Vec::<BvhNode>::with_capacity(2 * objects.len() - 1);

        let mut root_aabb = Aabb::empty();
        objects
            .iter()
            .for_each(|prim| root_aabb.expand(prim.bounding_box()));

        let root_node = BvhNode::new(root_aabb, 0, objects.len());
        nodes.push(root_node);

        let mut bvh = Bvh { nodes };

        bvh.subdivide(0, objects);

        bvh
    }

    pub(crate) fn subdivide(&mut self, idx_of_target: usize, objects: &mut Vec<Primitive>) {
        let target = &self.nodes[idx_of_target];

        let parent_cost = target.aabb.half_area() * target.primitive_count as f32;

        if true {
            let mut best_axis = Axis::X;
            let mut best_idx = 0;
            let mut best_cost = f32::MAX;
            for axis in Axis::iter() {
                for idx in 0..(target.primitive_count as usize) {
                    let cost = Self::calculate_sah(
                        idx,
                        &objects[(target.left as usize)
                            ..((target.left + target.primitive_count) as usize)],
                    );
                    if cost < best_cost {
                        best_cost = cost;
                        best_axis = *axis;
                        best_idx = idx;
                    }
                }
            }

            if best_cost > parent_cost {
                return;
            }

            let key_lambda = |a: &Primitive, b: &Primitive| {
                (a.bounding_box().axis_interval(best_axis).middle())
                    .total_cmp(&b.bounding_box().axis_interval(best_axis).middle())
            };

            let start = target.left as usize;
            let mid = (target.left + best_idx as u32) as usize;
            let end = (target.left + target.primitive_count) as usize;

            objects.as_mut_slice()[start..end].sort_by(key_lambda);

            let mut left_aabb = Aabb::empty();

            objects[start..=mid]
                .iter()
                .for_each(|prim| left_aabb.expand(prim.bounding_box()));

            let mut right_aabb = Aabb::empty();

            objects[mid..end]
                .iter()
                .for_each(|prim| right_aabb.expand(prim.bounding_box()));

            let idx_of_next = self.nodes.len();

            self.nodes[idx_of_target] = BvhNode {
                aabb: target.aabb,
                left: idx_of_next as u32,
                primitive_count: 0,
            };

            self.nodes.push(BvhNode::new(left_aabb, start, mid - start));
            self.nodes.push(BvhNode::new(right_aabb, mid, end - mid));

            self.subdivide(idx_of_next, objects);
            self.subdivide(idx_of_next + 1, objects);
        }
    }

    fn calculate_sah(idx: usize, prims: &[Primitive]) -> f32 {
        let mut left_box = Aabb::empty();
        prims[0..idx]
            .iter()
            .for_each(|prim| left_box.expand(prim.bounding_box()));

        let mut right_box = Aabb::empty();
        prims[idx..]
            .iter()
            .for_each(|prim| right_box.expand(prim.bounding_box()));

        left_box.half_area() * idx as f32 + right_box.half_area() * (prims.len() - idx) as f32
    }

    pub(crate) fn hit(
        &self,
        node_idx: usize,
        ray: Ray,
        ray_interval: Interval,
        world: &Arc<Vec<Primitive>>,
    ) -> Option<HitRecord> {
        let current = &self.nodes[node_idx];
        if current.aabb.hit(ray, ray_interval) {
            if current.primitive_count != 0 {
                let left_idx = current.left as usize;
                let right_idx = left_idx + current.primitive_count as usize;

                let mut potential_hit: Option<HitRecord> = None;
                let mut closest_so_far = ray_interval.max;

                for prim in world[left_idx..right_idx].iter() {
                    if let Some(hit) =
                        prim.hit(ray, Interval::new(ray_interval.min, closest_so_far))
                    {
                        // return Some(HitRecord::new(
                        //     ray,
                        //     hit.t,
                        //     1.0,
                        //     1.0,
                        //     Vec3::new(1.0, 1.0, 1.0),
                        //     Arc::new(crate::material::Material::Lambertian(Arc::new(
                        //         crate::texture::Texture::Color(Color::new(1.0, 0.0, 1.0)),
                        //     ))),
                        // ));
                        closest_so_far = hit.t;
                        potential_hit = Some(hit);
                    }
                }

                potential_hit
            } else {
                let left_idx = current.left as usize;
                let right_idx = (current.left + 1) as usize;

                let left_hit = self.hit(left_idx, ray, ray_interval, world);

                let right_hit = self.hit(right_idx, ray, ray_interval, world);

                match (left_hit.is_some(), right_hit.is_some()) {
                    (true, true) => {
                        if let (Some(l), Some(r)) = (left_hit, right_hit) {
                            if l.t < r.t { Some(l) } else { Some(r) }
                        } else {
                            None
                        }
                    }
                    (true, false) => left_hit,
                    (false, true) => right_hit,
                    (false, false) => None,
                }
            }
        } else {
            None
        }
    }

    // fn get_leaf(
    //     &self,
    //     node_idx: usize,
    //     ray: crate::ray::Ray,
    //     ray_interval: crate::interval::Interval,
    // ) -> Option<usize> {
    // }
}

#[derive(Debug)]
pub(crate) struct BvhNode {
    aabb: Aabb,
    left: u32,
    primitive_count: u32,
}

impl BvhNode {
    pub(crate) fn new(aabb: Aabb, left: usize, primitive_count: usize) -> Self {
        BvhNode {
            aabb,
            left: left as u32,
            primitive_count: primitive_count as u32,
        }
    }
}
