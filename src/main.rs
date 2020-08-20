mod vector;
mod aabb;
mod models;
mod material;
mod camera;
mod bvh;
mod renderer;

use std::path::Path;
use std::fs::File;
use std::io::Write;

use rand::Rng;

use camera::Camera;
use material::{Dielectric, Lambertian, Metal};
use models::{Model, Sphere};
use vector::Vec3;
use renderer::render_par;
use std::sync::Arc;
use crate::bvh::BvhNode;


fn generate_world() -> Box<dyn Model> {
    let mut rng = rand::thread_rng();

    // World
    let mut world: Vec<Arc<dyn Model>> = Vec::new();

    // Ground
    world.push(Arc::new(Sphere {
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
                    world.push(Arc::new(Sphere {
                        center,
                        radius: 0.2,
                        material: Box::new(Lambertian { albedo }),
                    }));
                } else if random_mat < 0.95 {
                    // Metal
                    let albedo = Vec3::random(0.5, 1.0);
                    let fuzz = rng.gen_range(0.0, 0.5);
                    world.push(Arc::new(Sphere {
                        center,
                        radius: 0.2,
                        material: Box::new(Metal { albedo, fuzz }),
                    }));
                } else {
                    world.push(Arc::new(Sphere {
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
    world.push(Arc::new(Sphere {
        center: Vec3(0.0, 1.0, 0.0),
        radius: 1.0,
        material: Box::new(Dielectric {
            refractive_index: 1.5,
        }),
    }));

    world.push(Arc::new(Sphere {
        center: Vec3(-4.0, 1.0, 0.0),
        radius: 1.0,
        material: Box::new(Lambertian {
            albedo: Vec3(0.4, 0.2, 0.1),
        }),
    }));

    world.push(Arc::new(Sphere {
        center: Vec3(4.0, 1.0, 0.0),
        radius: 1.0,
        material: Box::new(Metal {
            albedo: Vec3(0.7, 0.6, 0.5),
            fuzz: 0.0,
        }),
    }));

    Box::new(world)
}


fn glass_test() -> Box<dyn Model> {
    // World
    let mut world: Vec<Arc<dyn Model>> = Vec::new();

    // Ground
    world.push(Arc::new(Sphere {
        center: Vec3(0.0, -100.5, -1.0),
        radius: 100.0,
        material: Box::new(Lambertian {
            albedo: Vec3(0.8, 0.8, 0.0),
        }),
    }));

    world.push(Arc::new(Sphere {
        center: Vec3(-1.0, 0.0, -1.0),
        radius: 0.5,
        material: Box::new(Dielectric {
            refractive_index: 1.5,
        }),
    }));

    world.push(Arc::new(Sphere {
        center: Vec3(0.0, 0.0, -1.0),
        radius: 0.5,
        material: Box::new(Dielectric {
            refractive_index: 1.5,
        }),
    }));

    world.push(Arc::new(Sphere {
        center: Vec3(1.0, 0.0, -1.0),
        radius: 0.5,
        material: Box::new(Metal {
            albedo: Vec3(0.7, 0.6, 0.5),
            fuzz: 0.0,
        }),
    }));

    let size = world.len();
    Box::new(BvhNode::new(&mut world, 0, size, 0.0, 0.0))
}

fn main() {
    const IMAGE_WIDTH: usize = 800;
    const IMAGE_HEIGHT: usize = (IMAGE_WIDTH as f64 / ASPECT_RATIO) as usize;
    const ASPECT_RATIO: f64 = 3.0 / 2.0;
    const SAMPLES_PER_PIXEL: usize = 500;

    let world = glass_test();

    let camera = Camera::new(
        Vec3(0.0, 0.0, 0.0),
        Vec3(0.0, 0.0, -1.0),
        Vec3(0.0, 1.0, 0.0),
        90.0,
        ASPECT_RATIO,
        0.1,
        2.0,
    );

    let image = render_par(world, Box::from(camera), IMAGE_WIDTH, IMAGE_HEIGHT, SAMPLES_PER_PIXEL);

    let path: &Path = Path::new(r"render.png");
    let out_file: File = File::create(path).unwrap();

    let mut encoder = png::Encoder::new(out_file, IMAGE_WIDTH as u32, IMAGE_HEIGHT as u32);

    encoder.set_color(png::ColorType::RGB);
    encoder.set_depth(png::BitDepth::Eight);

    let writer = encoder.write_header().unwrap();
    let mut streamwriter = writer.into_stream_writer();


    eprintln!("Saving image...");
    for line in &*image.lock().unwrap() {
        for pixel in line {
            streamwriter.write(&[pixel.0, pixel.1, pixel.2]).unwrap();
        }
    }

    streamwriter.finish().unwrap();
}
