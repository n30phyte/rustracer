use rand::Rng;

use super::models::Hit;
use super::vector::{Ray, Vec3};

pub struct Scatter {
    pub attenuation: Vec3,
    pub ray: Ray,
}

pub trait Material: Send + Sync {
    fn scatter(&self, r_in: &Ray, hit: &Hit) -> Option<Scatter>;
}

pub struct Lambertian {
    pub albedo: Vec3,
}

impl Material for Lambertian {
    fn scatter(&self, _r_in: &Ray, hit: &Hit) -> Option<Scatter> {
        Some(Scatter {
            attenuation: self.albedo,
            ray: Ray {
                origin: hit.point,
                direction: hit.normal + Vec3::random_unit_vector(),
            },
        })
    }
}

pub fn reflect(incident: Vec3, normal: Vec3) -> Vec3 {
    incident - 2.0 * (Vec3::dot(incident, normal) * normal)
}

pub struct Metal {
    pub albedo: Vec3,
    pub fuzz: f32,
}

impl Material for Metal {
    fn scatter(&self, r_in: &Ray, hit: &Hit) -> Option<Scatter> {
        let reflected = reflect(r_in.direction, hit.normal);
        let scattered = Ray {
            origin: hit.point,
            direction: reflected + (self.fuzz * Vec3::random_unit_sphere()),
        };

        if Vec3::dot(scattered.direction, hit.normal) > 0.0 {
            Some(Scatter {
                attenuation: self.albedo,
                ray: scattered,
            })
        } else {
            None
        }
    }
}

pub fn refract(uv: Vec3, normal: Vec3, eta_over_etaprime: f32) -> Vec3 {
    let cos_theta = Vec3::dot(-uv, normal);
    let r_out_perpendicular = eta_over_etaprime * (uv + (cos_theta * normal));
    let r_out_parallel = -((1.0 - r_out_perpendicular.len_sqr()).abs().sqrt()) * normal;
    r_out_perpendicular + r_out_parallel
}

pub struct Dielectric {
    pub refractive_index: f32,
}

impl Dielectric {
    fn schlick(cosine: f32, refractive_index: f32) -> f32 {
        let r0 = (1.0 - refractive_index) / (1.0 + refractive_index);
        let r0 = r0 * r0;

        r0 + (1.0 - r0) * (1.0 - cosine).powf(5.0)
    }
}

impl Material for Dielectric {
    fn scatter(&self, r_in: &Ray, hit: &Hit) -> Option<Scatter> {
        let attenuation = Vec3(1.0, 1.0, 1.0);

        let etai_over_etat = if hit.front_face {
            1.0 / self.refractive_index
        } else {
            self.refractive_index
        };

        let unit_direction = r_in.direction.unit();

        let cos_theta = Vec3::dot(-unit_direction, hit.normal).min(1.0);
        let sin_theta = (1.0 - (cos_theta * cos_theta)).sqrt();

        let reflected = reflect(unit_direction, hit.normal);
        let reflect_scatter = Some(Scatter {
            attenuation,
            ray: Ray {
                origin: hit.point,
                direction: reflected,
            },
        });

        if (etai_over_etat * sin_theta) > 1.0 {
            return reflect_scatter;
        }

        let mut rng = rand::thread_rng();

        if rng.gen::<f32>() < Dielectric::schlick(cos_theta, etai_over_etat) {
            return reflect_scatter;
        }

        let refracted = refract(unit_direction, hit.normal, etai_over_etat);
        Some(Scatter {
            attenuation,
            ray: Ray {
                origin: hit.point,
                direction: refracted,
            },
        })
    }
}
