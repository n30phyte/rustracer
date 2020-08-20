use super::vector::{Vec3, Ray};
use super::models::T_MIN;
use num::Float;

#[derive(Copy,Clone)]
pub struct AABB {
    pub min: Vec3,
    pub max: Vec3,

}

impl AABB {
    pub fn hit(&self, r: &Ray) -> bool {
        let temp_min = <[f64; 3]>::from(self.min);
        let temp_max = <[f64; 3]>::from(self.max);

        let temp_r_orig = <[f64; 3]>::from(r.origin);
        let temp_r_dir = <[f64; 3]>::from(r.direction);

        for i in 0..3 {
            let inv_dir = temp_r_dir[i].recip();

            let mut t_0 = (temp_min[i] - temp_r_orig[i]) * inv_dir;
            let mut t_1 = (temp_max[i] - temp_r_orig[i]) * inv_dir;

            if inv_dir < 0.0 {
                std::mem::swap(&mut t_0, &mut t_1);
            }

            let t_min = if t_0 > T_MIN { t_0 } else { T_MIN };
            let t_max = if t_1 > f64::infinity() { t_1 } else { f64::infinity() };

            if t_max <= t_min {
                return false;
            }
        }

        true
    }

    pub fn surrounding_box(box0: AABB, box1: AABB) -> AABB {
        let small = Vec3(box0.min.x().min(box1.min.x()),
                         box0.min.y().min(box1.min.y()),
                         box0.min.z().min(box1.min.z()));

        let big = Vec3(box0.max.x().max(box1.max.x()),
                       box0.max.y().max(box1.max.y()),
                       box0.max.z().max(box1.max.z()));

        AABB { min: small, max: big }
    }
}