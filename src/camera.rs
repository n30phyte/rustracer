use super::vector::{Ray, Vec3};
use crate::utils::deg_to_rad;

pub struct Camera {
    origin: Vec3,
    lower_left_corner: Vec3,
    horizontal: Vec3,
    vertical: Vec3,
    u: Vec3,
    v: Vec3,
    w: Vec3,
    lens_radius: f64,
}

impl Camera {
    pub fn new(look_from: Vec3, look_at: Vec3, v_up: Vec3,
               vert_fov: f64, aspect_ratio: f64,
               aperture: f64, focus_distance: f64) -> Camera {
        // Camera Properties

        let theta = deg_to_rad(vert_fov);
        let h = (theta / 2.0).tan();

        let viewport_height = 2.0 * h;
        let viewport_width = aspect_ratio * viewport_height;

        let w = (look_from - look_at).unit();
        let u = (Vec3::cross(v_up, w)).unit();
        let v = Vec3::cross(w, u);

        let origin = look_from;
        let horizontal = focus_distance * viewport_width * u;
        let vertical = focus_distance * viewport_height * v;
        let lower_left_corner = origin - (horizontal / 2.0) - (vertical / 2.0) - focus_distance * w;
        Camera {
            origin,
            horizontal,
            vertical,
            lower_left_corner,
            u,
            v,
            w,
            lens_radius: aperture / 2.0,
        }
    }
    pub fn get_ray(&self, s: f64, t: f64) -> Ray {
        let rd = self.lens_radius * Vec3::random_unit_disk();
        let offset = self.u * rd.x() + self.v * rd.y();
        Ray {
            origin: self.origin + offset,
            direction: self.lower_left_corner + (s * self.horizontal) + (t * self.vertical) - self.origin - offset,
        }
    }
}
