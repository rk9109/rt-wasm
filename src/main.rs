mod camera;
mod intersect;
mod ray;
mod sphere;
mod vec;

use rand::prelude::*;
use std::f64;
use std::fs;

use camera::Camera;
use intersect::{IntersectEvent, IntersectList};
use ray::Ray;
use sphere::Sphere;
use vec::Vec3;

fn color(r: &Ray, world: &IntersectList) -> Vec3 {
    if let Some(record) = world.intersect(r, 0.0, f64::MAX) {
        return (record.normal + Vec3::new(1.0, 1.0, 1.0)) * 0.5;
    } else {
        let unit_direction = r.direction.unit();
        let t = 0.5 * (unit_direction.y + 1.0);
        return Vec3::new(1.0, 1.0, 1.0) * (1.0 - t) + Vec3::new(0.5, 0.7, 1.0) * t;
    }
}

fn main() {
    let nx = 200;
    let ny = 100;
    let ns = 100;

    // ppm header
    let mut ppm = format!("P3\n{} {}\n{}\n", nx, ny, 255);

    // initialize rng
    let mut rng = rand::thread_rng();

    // initialize camera
    let cam = Camera::new(
        Vec3::new(-2.0, -1.0, -1.0),
        Vec3::new(4.0, 0.0, 0.0),
        Vec3::new(0.0, 2.0, 0.0),
        Vec3::new(0.0, 0.0, 0.0),
    );

    // initialize world
    let world = IntersectList::new(vec![
        Box::new(Sphere::new(Vec3::new(0.0, 0.0, -1.0), 0.5)),
        Box::new(Sphere::new(Vec3::new(0.0, -100.5, -1.0), 100.0)),
    ]);

    for j in (0..ny).rev() {
        for i in 0..nx {
            let mut pixel = Vec3::new(0.0, 0.0, 0.0);
            for _ in 0..ns {
                let u = (i as f64 + rng.gen::<f64>()) / nx as f64;
                let v = (j as f64 + rng.gen::<f64>()) / ny as f64;
                let r = cam.point(u, v);
                pixel += color(&r, &world);
            }
            pixel /= ns as f64;
            let ir = (255.99 * pixel.x) as i32;
            let ig = (255.99 * pixel.y) as i32;
            let ib = (255.99 * pixel.z) as i32;

            // ppm pixel
            ppm = format!("{}{} {} {}\n", ppm, ir, ig, ib);
        }
    }

    fs::write("output.ppm", &ppm).expect("ppm error");
}
