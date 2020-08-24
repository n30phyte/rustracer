use super::models::Model;
use crate::aabb::AABB;
use crate::models::Hit;
use crate::vector::{Ray, Vec3};
use rand;
use rand::Rng;
use std::sync::Arc;

pub struct BvhNode {
    left: Arc<dyn Model>,
    right: Arc<dyn Model>,
    bounding_box: AABB,
}

impl BvhNode {
    fn box_compare(a: Arc<dyn Model>, b: Arc<dyn Model>, c: u8) -> bool {
        let mut box_a = AABB {
            min: Vec3(0.0, 0.0, 0.0),
            max: Vec3(0.0, 0.0, 0.0),
        };

        let mut box_b = AABB {
            min: Vec3(0.0, 0.0, 0.0),
            max: Vec3(0.0, 0.0, 0.0),
        };

        if let Some(abox) = a.bounding_box(0.0, 0.0) {
            box_a = abox;
        }
        if let Some(bbox) = b.bounding_box(0.0, 0.0) {
            box_b = bbox;
        }
        return <[f32; 3]>::from(box_a.min)[c as usize] < <[f32; 3]>::from(box_b.min)[c as usize];
    }

    pub fn new(
        models: &mut Vec<Arc<dyn Model>>,
        start: usize,
        end: usize,
        time0: f32,
        time1: f32,
    ) -> BvhNode {
        let mut rng = rand::thread_rng();
        let axis: u8 = rng.gen_range(0, 2);
        let comparator = |a: Arc<dyn Model>, b: Arc<dyn Model>| BvhNode::box_compare(a, b, axis);

        let object_span = end - start;

        let left: Arc<dyn Model>;
        let right: Arc<dyn Model>;

        if object_span == 1 {
            left = models[start].clone();
            right = models[start].clone();
        } else if object_span == 2 {
            if comparator(models[start].clone(), models[start + 1].clone()) {
                left = models[start].clone();
                right = models[start + 1].clone();
            } else {
                left = models[start].clone();
                right = models[start + 1].clone();
            }
        } else {
            models.sort_unstable_by(|a, b| {
                if BvhNode::box_compare(a.clone(), b.clone(), axis) {
                    return std::cmp::Ordering::Less;
                } else {
                    return std::cmp::Ordering::Greater;
                }
            });
            let mid = start + object_span / 2;
            left = Arc::from(BvhNode::new(models, start, mid, time0, time1));
            right = Arc::from(BvhNode::new(models, mid, end, time0, time1));
        }

        let mut box_left = AABB {
            min: Vec3(0.0, 0.0, 0.0),
            max: Vec3(0.0, 0.0, 0.0),
        };

        let mut box_right = AABB {
            min: Vec3(0.0, 0.0, 0.0),
            max: Vec3(0.0, 0.0, 0.0),
        };

        if let Some(lbox) = left.bounding_box(time0, time1) {
            box_left = lbox;
        }
        if let Some(rbox) = right.bounding_box(time0, time1) {
            box_right = rbox;
        }

        BvhNode {
            left,
            right,
            bounding_box: AABB::surrounding_box(box_left, box_right),
        }
    }
}

impl Model for BvhNode {
    fn hit(&self, r: &Ray) -> Option<Hit> {
        if self.bounding_box.hit(r) {
            if let Some(hit_left) = self.left.hit(r) {
                return Some(hit_left);
            } else if let Some(hit_right) = self.right.hit(r) {
                return Some(hit_right);
            }
        }

        return None;
    }

    fn bounding_box(&self, t_0: f32, t_1: f32) -> Option<AABB> {
        Some(self.bounding_box.clone())
    }
}
