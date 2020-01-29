mod intersect;
mod material;
mod ray;
mod sphere;
mod vec;

pub mod camera;
pub mod scenes;

use std::f32;

use rand::Rng;

use camera::Camera;
use intersect::{IntersectEvent, IntersectList};
use ray::Ray;
use scenes::Params;
use vec::Vec3;

fn color(r: &Ray, world: &IntersectList, depth: u16, rng: &mut rand_pcg::Pcg64) -> Vec3 {
    // recursively trace the path of `r` as it intersects objects in `IntersectList`
    if let Some(record) = world.intersect(r, 0.001, f32::MAX) {
        if depth < 50 {
            if let Some((scattered, attenuation)) = record.material.scatter(&r, &record, rng) {
                return color(&scattered, &world, depth + 1, rng) * attenuation;
            }
        }
        return Vec3::new(0.0, 0.0, 0.0);
    } else {
        // linear interpolation of blue and white based on y-coordinate
        let unit_direction = r.direction.unit();
        let t = 0.5 * (unit_direction.y + 1.0);
        return Vec3::new(1.0, 1.0, 1.0) * (1.0 - t) + Vec3::new(0.5, 0.7, 1.0) * t;
    }
}

pub fn cast(
    params: &Params,
    world: &IntersectList,
    cam: &Camera,
    rng: &mut rand_pcg::Pcg64,
    create_image: bool,
    create_pb: bool,
) {
    // initialize image
    let mut image = Vec::with_capacity((3 * params.nx * params.ny) as usize);

    // initialize progress bar
    let pb = indicatif::ProgressBar::new((params.nx * params.ny) as u64);

    for j in (0..params.ny).rev() {
        for i in 0..params.nx {
            if create_pb {
                pb.inc(1);
            }
            let mut pixel = Vec3::new(0.0, 0.0, 0.0);
            for _ in 0..params.ns {
                let u = (i as f32 + rng.gen::<f32>()) / params.nx as f32;
                let v = (j as f32 + rng.gen::<f32>()) / params.ny as f32;
                let r = cam.point(u, v, rng);
                pixel += color(&r, &world, 0, rng);
            }
            pixel /= params.ns as f32;
            if create_image {
                image.push((255.99 * pixel.x.sqrt()) as u8);
                image.push((255.99 * pixel.y.sqrt()) as u8);
                image.push((255.99 * pixel.z.sqrt()) as u8);
            }
        }
    }
    if create_image {
        image::save_buffer(
            params.output.clone(),
            &image,
            params.nx,
            params.ny,
            image::RGB(8),
        )
        .expect("error saving image");
    }
    if create_pb {
        pb.finish_and_clear();
    }
}
