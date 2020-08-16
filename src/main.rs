mod types;
mod utils;

use rand::Rng;
use types::{
    camera::Camera,
    ray::Ray,
    vector::Vec3,
    solid_object::{
        SolidObject, Sphere, HitRecResult,
    },
    material::{
        ScatterRecResult, Lambertian, Metal, Dielectric,
    },
};
use std::sync::Arc;
use std::f64::consts::PI;

fn ray_color(r: &Ray, world: &dyn SolidObject, depth: usize) -> Vec3 {
    if depth == 0 {
        return Vec3::new(0.0, 0.0, 0.0);
    }

    match world.hit(r, 0.001, f64::INFINITY) {
        HitRecResult::Hit(rec) => {
            match rec.material.scatter(r, &rec) {
                ScatterRecResult::Hit(rec) => {
                    return rec.attenuation * ray_color(&rec.ray, world, depth - 1);
                }
                ScatterRecResult::Miss => {
                    Vec3::new(0.0, 0.0, 0.0)
                }
            }
        }
        _ => {
            let unit_direction = r.direction.unit();
            let t = 0.5 * (unit_direction.y() + 1.0);
            (1.0 - t) * Vec3::new(1.0, 1.0, 1.0) + t * Vec3::new(0.5, 0.7, 1.0)
        }
    }
}

fn generate_world() -> Vec<Box<dyn SolidObject>> {
    let mut rng = rand::thread_rng();

    // World
    let mut world: Vec<Box<dyn SolidObject>> = Vec::new();

    // Ground
    world.push(Box::new(Sphere {
        center: Vec3::new(0.0, -1000.0, 0.0),
        radius: 1000.0,
        material: Arc::new(Lambertian { albedo: Vec3::new(0.5, 0.5, 0.5) }),
    }));

    for a in -11..11 {
        for b in -11..11 {
            let random_mat = rng.gen::<f64>();

            let center = Vec3::new(a as f64 + 0.9 * rng.gen::<f64>(), 0.2, b as f64 + 0.9 * rng.gen::<f64>());

            if (center - Vec3::new(4.0, 0.2, 0.0)).len() > 0.9 {
                if random_mat < 0.8 {
                    // Diffuse
                    let albedo = Vec3::random() * Vec3::random();
                    world.push(Box::new(Sphere {
                        center,
                        radius: 0.2,
                        material: Arc::new(Lambertian { albedo }),
                    }));
                } else if random_mat < 0.95 {
                    // Metal
                    let albedo = Vec3::random_clamped(0.5, 1.0);
                    let fuzz = rng.gen_range(0.0, 0.5);
                    world.push(Box::new(Sphere {
                        center,
                        radius: 0.2,
                        material: Arc::new(Metal { albedo, fuzz }),
                    }));
                } else {
                    world.push(Box::new(Sphere {
                        center,
                        radius: 0.2,
                        material: Arc::new(Dielectric { refractive_index: 1.5 }),
                    }));
                }
            }
        }
    }
    world.push(Box::new(Sphere {
        center: Vec3::new(0.0, 1.0, 0.0),
        radius: 1.0,
        material: Arc::new(Dielectric { refractive_index: 1.5 }),
    }));

    world.push(Box::new(Sphere {
        center: Vec3::new(-4.0, 1.0, 0.0),
        radius: 1.0,
        material: Arc::new(Lambertian { albedo: Vec3::new(0.4, 0.2, 0.1) }),
    }));

    world.push(Box::new(Sphere {
        center: Vec3::new(4.0, 1.0, 0.0),
        radius: 1.0,
        material: Arc::new(Metal { albedo: Vec3::new(0.7, 0.6, 0.5), fuzz: 0.0 }),
    }));


    world
}

fn main() {
    let mut rng = rand::thread_rng();

    // Image Properties
    const ASPECT_RATIO: f64 = 3.0 / 2.0;
    const IMAGE_WIDTH: i64 = 1200;
    const IMAGE_HEIGHT: i64 = (IMAGE_WIDTH as f64 / ASPECT_RATIO) as i64;
    const SAMPLES_PER_PIXEL: i64 = 500;

    let world = generate_world();

    let camera = Camera::new(
        Vec3::new(13.0, 2.0, 3.0),
        Vec3::new(0.0, 0.0, 0.0),
        Vec3::new(0.0, 1.0, 0.0),
        20.0,
        ASPECT_RATIO,
        0.1,
        10.0,
    );

    println!("P3\n{0} {1} \n255", IMAGE_WIDTH, IMAGE_HEIGHT);

    for j in (0..IMAGE_HEIGHT).rev() {
        eprintln!("Lines remaining: {0}", j);
        for i in 0..IMAGE_WIDTH {
            let mut pixel_color = Vec3::new(0.0, 0.0, 0.0);
            for _ in 0..SAMPLES_PER_PIXEL {
                let u = (i as f64 + rng.gen::<f64>()) / (IMAGE_WIDTH - 1) as f64;
                let v = (j as f64 + rng.gen::<f64>()) / (IMAGE_HEIGHT - 1) as f64;
                let r = camera.get_ray(u, v);
                pixel_color = pixel_color + ray_color(&r, &world, 50);
            }
            println!("{0}", pixel_color.as_color(SAMPLES_PER_PIXEL));
        }
    }
}
