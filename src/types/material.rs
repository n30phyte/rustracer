use super::{ray::Ray, vector::Vec3, solid_object::HitRecord};
use std::cmp::min;
use rand::Rng;

pub enum ScatterRecResult {
    Hit(ScatterRec),
    Miss,
}

pub struct ScatterRec {
    pub attenuation: Vec3,
    pub ray: Ray,
}

pub trait Material {
    fn scatter(&self, r_in: &Ray, hr: &HitRecord) -> ScatterRecResult;
}

pub struct Lambertian {
    pub albedo: Vec3
}

impl Material for Lambertian {
    fn scatter(&self, _r_in: &Ray, hr: &HitRecord) -> ScatterRecResult {
        ScatterRecResult::Hit(ScatterRec {
            attenuation: self.albedo,
            ray: Ray {
                origin: hr.point,
                direction: hr.normal + Vec3::random_unit_vector(),
            },
        })
    }
}

pub struct Metal {
    pub albedo: Vec3,
    pub fuzz: f64,
}

impl Material for Metal {
    fn scatter(&self, r_in: &Ray, hr: &HitRecord) -> ScatterRecResult {
        let reflected = Vec3::reflect(r_in.direction.unit(), hr.normal);
        return if Vec3::dot(&reflected, &hr.normal) > 0.0 {
            ScatterRecResult::Hit(ScatterRec {
                attenuation: self.albedo,
                ray: Ray {
                    origin: hr.point,
                    direction: reflected + self.fuzz * Vec3::random_unit_sphere(),
                },
            })
        } else {
            ScatterRecResult::Miss
        };
    }
}

pub struct Dielectric {
    pub refractive_index: f64
}

impl Dielectric {
    fn schlick(cosine: f64, refractive_index: f64) -> f64 {
        let r0 = (1.0 - refractive_index) / (1.0 + refractive_index);
        let r0 = r0 * r0;

        r0 + (1.0 - r0) * (1.0 - cosine).powf(5.0)
    }
}

impl Material for Dielectric {
    fn scatter(&self, r_in: &Ray, hr: &HitRecord) -> ScatterRecResult {
        let mut rng = rand::thread_rng();

        let refractive_index = if hr.front_face { 1.0 / self.refractive_index } else { self.refractive_index };

        let attenuation = Vec3::new(1.0, 1.0, 1.0);

        let unit_direction = r_in.direction.unit();

        let cos_theta = Vec3::dot(&-unit_direction, &hr.normal).min(1.0);
        let sin_theta = (1.0 - (cos_theta * cos_theta)).sqrt();

        let reflect_prob = Dielectric::schlick(cos_theta, refractive_index);

        return if ((refractive_index * sin_theta) > 1.0) || rng.gen::<f64>() < reflect_prob {
            let reflected = Vec3::reflect(unit_direction, hr.normal);

            ScatterRecResult::Hit(ScatterRec {
                attenuation,
                ray: Ray {
                    origin: hr.point,
                    direction: reflected,
                },
            })
        } else {
            let refracted = Vec3::refract(unit_direction, hr.normal, refractive_index);

            ScatterRecResult::Hit(ScatterRec {
                attenuation,
                ray: Ray {
                    origin: hr.point,
                    direction: refracted,
                },
            })
        };
    }
}