mod vector;
mod models;
mod material;
mod camera;
mod utils;
mod renderer;

use rand::Rng;
use camera::Camera;
use material::{Dielectric, Lambertian, Metal};
use models::{Model, Sphere};
use vector::Vec3;
use crate::renderer::render_par;

fn generate_world() -> Box<dyn Model + Sync> {
    let mut rng = rand::thread_rng();

    // World
    let mut world: Vec<Box<dyn Model + Sync>> = Vec::new();

    // Ground
    world.push(Box::new(Sphere {
        center: Vec3(0.0, -1000.0, 0.0),
        radius: 1000.0,
        material: Box::new(Lambertian {
            albedo: Vec3(0.5, 0.5, 0.5),
        }),
    }));

    for a in -11..11 {
        for b in -11..11 {
            let random_mat = rng.gen::<f64>();

            let center = Vec3(
                a as f64 + 0.9 * rng.gen::<f64>(),
                0.2,
                b as f64 + 0.9 * rng.gen::<f64>(),
            );

            if (center - Vec3(4.0, 0.2, 0.0)).len() > 0.9 {
                if random_mat < 0.8 {
                    // Diffuse
                    let albedo = rng.gen::<Vec3>();
                    world.push(Box::new(Sphere {
                        center,
                        radius: 0.2,
                        material: Box::new(Lambertian { albedo }),
                    }));
                } else if random_mat < 0.95 {
                    // Metal
                    let albedo = Vec3::random(0.5, 1.0);
                    let fuzz = rng.gen_range(0.0, 0.5);
                    world.push(Box::new(Sphere {
                        center,
                        radius: 0.2,
                        material: Box::new(Metal { albedo, fuzz }),
                    }));
                } else {
                    world.push(Box::new(Sphere {
                        center,
                        radius: 0.2,
                        material: Box::new(Dielectric {
                            refractive_index: 1.5,
                        }),
                    }));
                }
            }
        }
    }
    world.push(Box::new(Sphere {
        center: Vec3(0.0, 1.0, 0.0),
        radius: 1.0,
        material: Box::new(Dielectric {
            refractive_index: 1.5,
        }),
    }));

    world.push(Box::new(Sphere {
        center: Vec3(-4.0, 1.0, 0.0),
        radius: 1.0,
        material: Box::new(Lambertian {
            albedo: Vec3(0.4, 0.2, 0.1),
        }),
    }));

    world.push(Box::new(Sphere {
        center: Vec3(4.0, 1.0, 0.0),
        radius: 1.0,
        material: Box::new(Metal {
            albedo: Vec3(0.7, 0.6, 0.5),
            fuzz: 0.0,
        }),
    }));

    Box::new(world)
}


fn glass_test() -> Box<dyn Model + Sync> {
    // World
    let mut world: Vec<Box<dyn Model + Sync>> = Vec::new();

    // Ground
    world.push(Box::new(Sphere {
        center: Vec3(0.0, -100.5, -1.0),
        radius: 100.0,
        material: Box::new(Lambertian {
            albedo: Vec3(0.8, 0.8, 0.0),
        }),
    }));

    world.push(Box::new(Sphere {
        center: Vec3(-1.0, 0.0, -1.0),
        radius: 0.5,
        material: Box::new(Dielectric {
            refractive_index: 1.5,
        }),
    }));

    world.push(Box::new(Sphere {
        center: Vec3(0.0, 0.0, -1.0),
        radius: 0.5,
        material: Box::new(Dielectric {
            refractive_index: 1.5,
        }),
    }));

    world.push(Box::new(Sphere {
        center: Vec3(1.0, 0.0, -1.0),
        radius: 0.5,
        material: Box::new(Metal {
            albedo: Vec3(0.7, 0.6, 0.5),
            fuzz: 0.0,
        }),
    }));

    Box::new(world)
}

fn main() {
    const IMAGE_WIDTH: usize = 900;
    const IMAGE_HEIGHT: usize = (IMAGE_WIDTH as f64 / ASPECT_RATIO) as usize;
    const ASPECT_RATIO: f64 = 3.0 / 2.0;
    const SAMPLES_PER_PIXEL: usize = 500;

    let world = generate_world();

    let camera = Camera::new(
        Vec3(11.0, 3.0, 11.0),
        Vec3(0.0, 1.0, 0.0),
        Vec3(0.0, 1.0, 0.0),
        20.0,
        ASPECT_RATIO,
        0.1,
        16.0,
    );

    let image = render_par(world, Box::from(camera), IMAGE_WIDTH, IMAGE_HEIGHT, SAMPLES_PER_PIXEL);

    eprintln!("Saving image...");
    println!("P3\n{0} {1}\n255\n", IMAGE_WIDTH, IMAGE_HEIGHT);
    for line in &*image.lock().unwrap() {
        for pixel in line {
            println!("{0} {1} {2}", pixel.0, pixel.1, pixel.2);
        }
    }
}
