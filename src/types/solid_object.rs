use super::{ray::Ray, vector::Vec3, material::Material};
use num::traits::Pow;
use std::sync::Arc;

pub enum HitRecResult {
    Hit(HitRecord),
    Miss,
}

#[derive(Clone)]
pub struct HitRecord {
    pub point: Vec3,
    pub normal: Vec3,
    pub t: f64,
    pub front_face: bool,
    pub material: Arc<dyn Material>,
}

impl HitRecord {
    pub fn set_face_normal(&mut self, r: &Ray, outward_normal: Vec3) {
        self.front_face = Vec3::dot(&r.direction, &outward_normal) < 0.0;
        self.normal = if self.front_face {
            outward_normal
        } else {
            -outward_normal
        };
    }
}

pub trait SolidObject {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> HitRecResult;
}

pub struct Sphere {
    pub center: Vec3,
    pub radius: f64,
    pub material: Arc<dyn Material>,
}

impl SolidObject for Sphere {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> HitRecResult {
        let oc = r.origin - self.center;
        let a = r.direction.len_sqr();
        let hf_b = Vec3::dot(&oc, &r.direction);
        let c = oc.len_sqr() - self.radius.pow(2);
        let discriminant: f64 = (hf_b * hf_b) - (a * c);

        if discriminant > 0.0 {
            let root = discriminant.sqrt();

            let pair = ((-hf_b - root) / a, (-hf_b + root) / a);

            let temp = match pair {
                (x, _) if x < t_max && x > t_min => x,
                (_, y) if y < t_max && y > t_min => y,
                _ => return HitRecResult::Miss,
            };

            let point = r.at(temp);
            let outward_normal = (point - self.center) / self.radius;
            let mut output = HitRecord {
                t: temp,
                point: r.at(temp),
                normal: Vec3::new(0.0, 0.0, 0.0),
                front_face: false,
                material: self.material.clone(),
            };
            output.set_face_normal(r, outward_normal);

            return HitRecResult::Hit(output);
        }

        HitRecResult::Miss
    }
}

impl SolidObject for Vec<Box<dyn SolidObject>> {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> HitRecResult {
        let mut hre = HitRecResult::Miss;
        let mut closest_so_far = t_max;
        for item in self {
            if let HitRecResult::Hit(res) = item.hit(r, t_min, closest_so_far) {
                closest_so_far = res.t;
                hre = HitRecResult::Hit(res);
            }
        }

        hre
    }
}
