use super::aabb::AABB;
use super::material::Material;
use super::vector::{Ray, Vec3};
use std::sync::Arc;

// Minimum t to reduce acne
pub(crate) const T_MIN: f32 = 0.0001;

#[derive(Clone, Copy)]
pub struct Hit<'a> {
    pub point: Vec3,
    pub normal: Vec3,
    pub t: f32,
    pub front_face: bool,
    pub material: &'a dyn Material,
}

pub fn get_face_normal(r: &Ray, outward_normal: Vec3) -> (bool, Vec3) {
    let front_face = Vec3::dot(r.direction, outward_normal) < 0.0;
    let normal = if front_face {
        outward_normal
    } else {
        -outward_normal
    };

    (front_face, normal)
}

pub trait Model: Send + Sync {
    fn hit(&self, r: &Ray) -> Option<Hit>;
    fn bounding_box(&self, t_0: f32, t_1: f32) -> Option<AABB>;
}

pub struct Sphere {
    pub center: Vec3,
    pub radius: f32,
    pub material: Box<dyn Material>,
}

impl Model for Sphere {
    fn hit(&self, r: &Ray) -> Option<Hit> {
        let oc = r.origin - self.center;
        let a = r.direction.len_sqr();
        let hf_b = Vec3::dot(oc, r.direction);
        let c = oc.len_sqr() - self.radius.powi(2);
        let discriminant: f32 = hf_b.powi(2) - (a * c);

        if discriminant > 0.0 {
            let root = discriminant.sqrt();

            let t = ((-hf_b - root) / a, (-hf_b + root) / a);

            return match t {
                (x, _) if x >= T_MIN => {
                    let normals = get_face_normal(r, (r.at(x) - self.center) / self.radius);
                    Some(Hit {
                        t: x,
                        point: r.at(x),
                        normal: normals.1,
                        front_face: normals.0,
                        material: self.material.as_ref(),
                    })
                }
                (_, y) if y >= T_MIN => {
                    let normals = get_face_normal(r, (r.at(y) - self.center) / self.radius);
                    Some(Hit {
                        t: y,
                        point: r.at(y),
                        normal: normals.1,
                        front_face: normals.0,
                        material: self.material.as_ref(),
                    })
                }
                _ => None,
            };
        }

        None
    }

    fn bounding_box(&self, _t_0: f32, _t_1: f32) -> Option<AABB> {
        Some(AABB {
            min: self.center - Vec3(self.radius, self.radius, self.radius),
            max: self.center + Vec3(self.radius, self.radius, self.radius),
        })
    }
}

impl Model for Vec<Arc<dyn Model>> {
    fn hit(&self, r: &Ray) -> Option<Hit> {
        let mut closest_so_far: Option<Hit> = None;
        for item in self {
            if let Some(hit) = item.hit(r) {
                match closest_so_far {
                    None => closest_so_far = Some(hit),
                    Some(old) => {
                        if hit.t < old.t {
                            closest_so_far = Some(hit);
                        }
                    }
                }
            }
        }

        closest_so_far
    }

    fn bounding_box(&self, t_0: f32, t_1: f32) -> Option<AABB> {
        if self.is_empty() {
            return None;
        }

        let mut first_box = true;
        let mut output_box: AABB = AABB {
            min: Vec3(0.0, 0.0, 0.0),
            max: Vec3(0.0, 0.0, 0.0),
        };
        for item in self {
            if let Some(bounding_box) = item.bounding_box(t_0, t_1) {
                output_box = if first_box {
                    first_box = false;
                    bounding_box
                } else {
                    AABB::surrounding_box(output_box, bounding_box)
                }
            } else {
                return None;
            }
        }
        Some(output_box)
    }
}
