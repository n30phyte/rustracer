use rand::prelude::*;
use rayon::prelude::*;
use super::{camera::Camera, models::Model, vector::Ray, vector::Vec3};
use std::sync::{Mutex, Arc};
use num::traits::real::Real;

#[derive(Copy, Clone)]
pub struct Pixel(pub u8, pub u8, pub u8);

fn ray_color(r: Ray, world: &dyn Model, depth: usize) -> Vec3 {
    if depth == 0 {
        return Vec3(0.0, 0.0, 0.0);
    }

    match world.hit(&r) {
        Some(hit) => match hit.material.scatter(&r, &hit) {
            Some(scatter) => {
                scatter.attenuation * ray_color(scatter.ray, world, depth - 1)
            }
            _ => Vec3(0.0, 0.0, 0.0),
        },
        _ => {
            let unit_direction = r.direction.unit();
            let t = 0.5 * (unit_direction.y() + 1.0);
            (1.0 - t) * Vec3(1.0, 1.0, 1.0) + t * Vec3(0.5, 0.7, 1.0)
        }
    }
}

pub fn render(world: Box<dyn Model + Sync>, camera: Box<Camera>, width: usize, height: usize, samples: usize) -> Vec<Vec<Pixel>> {
    let mut frame = vec![Vec::with_capacity(5); height];

    for j in (0..height).rev() {
        let mut rng = rand::thread_rng();

        let mut line = Vec::with_capacity(width);
        for i in 0..width {
            let mut pixel_color = Vec3(0.0, 0.0, 0.0);

            for _ in 0..samples {
                let u = (i as f64 + rng.gen::<f64>()) / (width - 1) as f64;
                let v = (j as f64 + rng.gen::<f64>()) / (height - 1) as f64;
                let r = camera.get_ray(u, v);
                pixel_color = pixel_color + ray_color(r, world.as_ref(), 50);
            }

            let pixel = Pixel(pixel_color.r(samples), pixel_color.g(samples), pixel_color.b(samples));
            line.push(pixel);
        }
        frame[j] = line;
        eprintln!("Done rendering line {0}", j);
    }

    frame
}

pub fn render_par(world: Box<dyn Model + Sync>, camera: Box<Camera>, width: usize, height: usize, samples: usize) -> Arc<Mutex<Vec<Vec<Pixel>>>> {
    let frame = Arc::new(Mutex::new(vec![Vec::with_capacity(5); height]));

    (0..height).into_par_iter().for_each(|j| {
        eprintln!("Working on line {0}", j);
        let mut rng = rand::thread_rng();

        let mut line = Vec::with_capacity(width);
        for i in 0..width {
            let mut pixel_color = Vec3(0.0, 0.0, 0.0);

            for _ in 0..samples {
                let u = (i as f64 + rng.gen::<f64>()) / (width - 1) as f64;
                let v = (j as f64 + rng.gen::<f64>()) / (height - 1) as f64;
                let r = camera.get_ray(u, v);
                pixel_color = pixel_color + ray_color(r, world.as_ref(), 50);
            }

            let pixel = Pixel(pixel_color.r(samples), pixel_color.g(samples), pixel_color.b(samples));
            line.push(pixel);
        }
        frame.lock().unwrap()[height - j - 1] = line;
    });

    frame
}