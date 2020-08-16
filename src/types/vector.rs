extern crate num;

// Allow us to use any numerical type
use self::num::traits::Inv;
use std::{fmt, ops};
use rand::Rng;

#[derive(Clone, Copy)]
pub struct Vec3 {
    v: [f64; 3],
}

impl Vec3 {
    // Take in a tuple with any numeric type
    pub fn new(val0: f64, val1: f64, val2: f64) -> Vec3 {
        Vec3 {
            v: [val0, val1, val2], // Set values from tuple
        }
    }

    pub fn x(&self) -> f64 {
        self[0]
    }
    pub fn y(&self) -> f64 {
        self[1]
    }
    pub fn z(&self) -> f64 {
        self[2]
    }

    pub fn len_sqr(&self) -> f64 {
        Vec3::dot(self, self)
    }

    pub fn len(&self) -> f64 {
        self.len_sqr().sqrt()
    }

    pub fn dot(u: &Vec3, v: &Vec3) -> f64 {
        u.x() * v.x() + u.y() * v.y() + u.z() * v.z()
    }

    pub fn cross(u: &Vec3, v: &Vec3) -> Vec3 {
        Vec3 {
            v: [
                (u.y() * v.z()) - (u.z() * v.y()),
                -(u.x() * v.z()) + (u.z() * v.x()),
                (u.x() * v.y()) - (u.y() * v.x()),
            ]
        }
    }

    pub fn unit(&self) -> Vec3 {
        *self / self.len()
    }

    pub fn random() -> Vec3 {
        let mut rng = rand::thread_rng();

        Vec3 {
            v: [rng.gen::<f64>(), rng.gen::<f64>(), rng.gen::<f64>()]
        }
    }

    pub fn random_clamped(min: f64, max: f64) -> Vec3 {
        let mut rng = rand::thread_rng();

        Vec3 {
            v: [rng.gen_range(min, max), rng.gen_range(min, max), rng.gen_range(min, max)]
        }
    }

    pub fn random_unit_disk() -> Vec3 {
        let mut rng = rand::thread_rng();

        loop {
            let p = Vec3::new(rng.gen_range(-1.0, 1.0), rng.gen_range(-1.0, 1.0), 0.0);
            if p.len_sqr() <= 1.0 { return p; }
        }
    }

    pub fn random_unit_sphere() -> Vec3 {
        loop {
            let p = Vec3::random_clamped(-1.0, 1.0);
            if p.len_sqr() <= 1.0 { return p; }
        }
    }
    pub fn random_unit_vector() -> Vec3 {
        let mut rng = rand::thread_rng();

        let a = rng.gen_range(0.0, 2.0 * std::f64::consts::PI);
        let z = rng.gen_range(-1.0f64, 1.0f64);
        let r = (1.0 - (z * z)).sqrt();

        Vec3::new(r * a.cos(), r * a.sin(), z)
    }

    pub fn reflect(incident: Vec3, normal: Vec3) -> Vec3 {
        incident - 2.0 * (Vec3::dot(&incident, &normal) * normal)
    }

    pub fn refract(uv: Vec3, normal: Vec3, eta_over_etaprime: f64) -> Vec3 {
        let cos_theta = Vec3::dot(&-uv, &normal);
        let r_out_perpendicular = eta_over_etaprime * (uv + (cos_theta * normal));
        let r_out_parallel = -((1.0 - r_out_perpendicular.len_sqr()).abs().sqrt()) * normal;
        r_out_perpendicular + r_out_parallel
    }

    pub fn as_color(&self, samples: i64) -> String {
        let r = self.x();
        let g = self.y();
        let b = self.z();

        let scale = 1.0 / samples as f64;

        let r = (r * scale).sqrt();
        let g = (g * scale).sqrt();
        let b = (b * scale).sqrt();

        format!(
            "{0} {1} {2}",
            (256.0 * num::clamp(r, 0.0, 0.999)) as i64,
            (256.0 * num::clamp(g, 0.0, 0.999)) as i64,
            (256.0 * num::clamp(b, 0.0, 0.999)) as i64
        )
    }
}

impl ops::Index<usize> for Vec3 {
    type Output = f64;

    fn index(&self, index: usize) -> &Self::Output {
        assert!(index < 3);
        &self.v[index]
    }
}

impl ops::Add for Vec3 {
    type Output = Vec3;

    fn add(self, rhs: Vec3) -> Vec3 {
        Vec3 {
            v: [self.x() + rhs.x(), self.y() + rhs.y(), self.z() + rhs.z()],
        }
    }
}

impl ops::Sub for Vec3 {
    type Output = Vec3;

    fn sub(self, rhs: Vec3) -> Vec3 {
        Vec3 {
            v: [self.x() - rhs.x(), self.y() - rhs.y(), self.z() - rhs.z()],
        }
    }
}

impl ops::Neg for Vec3 {
    type Output = Vec3;

    fn neg(self) -> Self::Output {
        Vec3 {
            v: [-self.x(), self.y(), self.z()],
        }
    }
}

impl ops::Mul<f64> for Vec3 {
    type Output = Self;

    fn mul(self, rhs: f64) -> Self::Output {
        Vec3 {
            v: [
                self.x() * rhs as f64,
                self.y() * rhs as f64,
                self.z() * rhs as f64,
            ],
        }
    }
}

impl ops::Mul<Vec3> for f64 {
    type Output = Vec3;

    fn mul(self, rhs: Vec3) -> Self::Output {
        rhs * self
    }
}

impl ops::Mul<Vec3> for Vec3 {
    type Output = Vec3;

    fn mul(self, rhs: Vec3) -> Self::Output {
        Vec3 {
            v: [
                self.x() * rhs.x() as f64,
                self.y() * rhs.y() as f64,
                self.z() * rhs.z() as f64,
            ],
        }
    }
}

impl ops::Div<f64> for Vec3 {
    type Output = Vec3;

    fn div(self, rhs: f64) -> Self::Output {
        self * rhs.inv()
    }
}

impl fmt::Display for Vec3 {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{0} {1} {2}", self.x(), self.y(), self.z())
    }
}

// impl From<Vec3> for Color {
//     fn from(item: Vec3) -> Self {
//         Color(item)
//     }
// }
//
// impl From<Vec3> for Point3 {
//     fn from(item: Vec3) -> Self {
//         Point3(item)
//     }
// }
//
// impl fmt::Display for Color {
//     fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
//         write!(f, "{0} {1} {2}",
//                (self.x() * 255.999) as u32,
//                (self.y() * 255.999) as u32,
//                (self.z() * 255.999) as u32)
//     }
// }
