extern crate num;

use std::{fmt, ops};

use num::traits::Inv;
use rand::distributions::{Distribution, Standard};
use rand::Rng;

#[derive(Clone, Copy, PartialEq, Debug)]
pub struct Vec3(pub f32, pub f32, pub f32);

impl Vec3 {
    pub fn x(&self) -> f32 {
        self.0
    }
    pub fn y(&self) -> f32 {
        self.1
    }
    pub fn z(&self) -> f32 {
        self.2
    }

    pub fn dot(u: Vec3, v: Vec3) -> f32 {
        (u * v).accumulate()
    }

    pub fn cross(u: Vec3, v: Vec3) -> Vec3 {
        Vec3((u.y() * v.z()) - (u.z() * v.y()),
             -(u.x() * v.z()) + (u.z() * v.x()),
             (u.x() * v.y()) - (u.y() * v.x()))
    }

    pub fn len_sqr(&self) -> f32 {
        Vec3::dot(*self, *self)
    }

    pub fn len(&self) -> f32 {
        self.len_sqr().sqrt()
    }

    pub fn unit(&self) -> Vec3 {
        *self / self.len()
    }

    pub fn random(min: f32, max: f32) -> Vec3 {
        let mut rng = rand::thread_rng();

        Vec3(rng.gen_range(min, max), rng.gen_range(min, max), rng.gen_range(min, max))
    }

    pub fn random_unit_disk() -> Vec3 {
        let mut rng = rand::thread_rng();

        loop {
            let p = Vec3(rng.gen_range(-1.0, 1.0), rng.gen_range(-1.0, 1.0), 0.0);
            if p.len_sqr() <= 1.0 { return p; }
        }
    }

    pub fn random_unit_sphere() -> Vec3 {
        loop {
            let p = Vec3::random(-1.0, 1.0);
            if p.len_sqr() <= 1.0 { return p; }
        }
    }

    pub fn random_unit_vector() -> Vec3 {
        let mut rng = rand::thread_rng();

        let a = rng.gen_range(0.0, 2.0 * std::f32::consts::PI);
        let z = rng.gen_range(-1.0f32, 1.0f32);
        let r = (1.0 - (z * z)).sqrt();

        Vec3(r * a.cos(), r * a.sin(), z)
    }

    pub fn r(&self, samples: usize) -> u8 {
        let r = self.x();
        let scale = 1.0 / samples as f32;
        let r = (r * scale).sqrt();
        (256.0 * num::clamp(r, 0.0, 0.999)) as u8
    }

    pub fn g(&self, samples: usize) -> u8 {
        let g = self.y();
        let scale = 1.0 / samples as f32;
        let g = (g * scale).sqrt();
        (256.0 * num::clamp(g, 0.0, 0.999)) as u8
    }

    pub fn b(&self, samples: usize) -> u8 {
        let b = self.z();
        let scale = 1.0 / samples as f32;
        let b = (b * scale).sqrt();
        (256.0 * num::clamp(b, 0.0, 0.999)) as u8
    }

    pub fn accumulate(&self) -> f32 {
        self.0 + self.1 + self.2
    }
}

impl Distribution<Vec3> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Vec3 {
        Vec3(rng.gen(), rng.gen(), rng.gen())
    }
}

impl ops::Add for Vec3 {
    type Output = Vec3;

    fn add(self, rhs: Vec3) -> Vec3 {
        Vec3(self.x() + rhs.x(), self.y() + rhs.y(), self.z() + rhs.z())
    }
}

impl ops::Sub for Vec3 {
    type Output = Vec3;

    fn sub(self, rhs: Vec3) -> Vec3 {
        Vec3(self.x() - rhs.x(), self.y() - rhs.y(), self.z() - rhs.z())
    }
}


impl ops::Neg for Vec3 {
    type Output = Vec3;

    fn neg(self) -> Self::Output {
        Vec3(-self.x(), -self.y(), -self.z())
    }
}

impl ops::Mul<f32> for Vec3 {
    type Output = Self;

    fn mul(self, rhs: f32) -> Self::Output {
        Vec3(self.x() * rhs as f32,
             self.y() * rhs as f32,
             self.z() * rhs as f32, )
    }
}


impl ops::Mul<Vec3> for f32 {
    type Output = Vec3;

    fn mul(self, rhs: Vec3) -> Self::Output {
        Vec3(self * rhs.x(),
             self * rhs.y() as f32,
             self * rhs.z() as f32, )
    }
}

impl ops::Mul<Vec3> for Vec3 {
    type Output = Vec3;

    fn mul(self, rhs: Vec3) -> Self::Output {
        Vec3(self.x() * rhs.x() as f32,
             self.y() * rhs.y() as f32,
             self.z() * rhs.z() as f32, )
    }
}

impl ops::Div<f32> for Vec3 {
    type Output = Vec3;

    fn div(self, rhs: f32) -> Self::Output {
        self * rhs.inv()
    }
}

impl fmt::Display for Vec3 {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{0} {1} {2}", self.x(), self.y(), self.z())
    }
}

pub struct Ray {
    pub origin: Vec3,
    pub direction: Vec3,
}

impl Ray {
    pub fn at(&self, t: f32) -> Vec3 {
        self.origin + (t * self.direction)
    }
}


impl From<Vec3> for [f32; 3] {
    fn from(source: Vec3) -> Self {
        [source.0, source.1, source.2]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const VEC1: Vec3 = Vec3(1.0, 2.0, 3.0);
    const VEC2: Vec3 = Vec3(3.0, 2.0, 1.0);

    #[test]
    fn test_addition() {
        assert_eq!(VEC1 + VEC2, Vec3(4.0, 4.0, 4.0));
    }

    #[test]
    fn test_subtraction() {
        assert_eq!(VEC2 - VEC2, Vec3(0.0, 0.0, 0.0));
    }

    #[test]
    fn test_vector_multiplication() {
        assert_eq!(VEC1 * VEC2, Vec3(3.0, 4.0, 3.0));
    }

    #[test]
    fn test_negation() {
        assert_eq!(-VEC1, Vec3(-1.0, -2.0, -3.0));
    }

    #[test]
    fn test_float_multiplication() {
        assert_eq!(2.0 * VEC1, Vec3(2.0, 4.0, 6.0));
    }

    #[test]
    fn test_float_division() {
        assert_eq!(VEC1 / 2.0, Vec3(0.5, 1.0, 1.5));
    }

    #[test]
    fn test_dot_product() {
        assert_eq!(Vec3::dot(VEC1, VEC2), 10.0);
    }

    #[test]
    fn test_cross_product() {
        assert_eq!(Vec3::cross(VEC1, VEC2), Vec3(-4.0, 8.0, -4.0));
    }

    #[test]
    fn test_len() {
        assert_eq!(VEC1.len(), 14.0_f32.sqrt())
    }
}
