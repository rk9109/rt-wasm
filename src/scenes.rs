use rand::Rng;
use rand_pcg;

use crate::camera::Camera;
use crate::intersect::{IntersectEvent, IntersectList};
use crate::material::{Dielectric, Lambertian, Metal};
use crate::sphere::Sphere;
use crate::vec::Vec3;

#[allow(dead_code)]
pub fn custom_scene(nx: u32, ny: u32) -> (IntersectList, Camera) {
    // scene used in the README
    //
    let mut list: Vec<Box<dyn IntersectEvent>> = Vec::with_capacity(10);

    // material options
    let ground = Lambertian::new(Vec3::new(0.35, 0.35, 0.45));
    let pink = Lambertian::new(Vec3::new(0.8, 0.4, 0.4));
    let gold = Metal::new(0.0, Vec3::new(1.0, 0.8, 0.4));
    let gold_rough = Metal::new(0.25, Vec3::new(1.0, 0.8, 0.4));
    let silver = Metal::new(0.0, Vec3::new(0.8, 0.8, 0.8));
    let silver_rough = Metal::new(0.25, Vec3::new(0.8, 0.8, 0.8));
    let glass = Dielectric::new(1.5, 0.0, Vec3::new(0.8, 0.8, 0.8));
    let glass_rough = Dielectric::new(1.5, 0.15, Vec3::new(0.8, 0.8, 0.8));

    // create base
    list.push(Box::new(Sphere::new(
        Vec3::new(0.0, -1000.0, 0.0),
        1000.0,
        ground.clone(),
    )));

    // create large spheres
    list.push(Box::new(Sphere::new(
        Vec3::new(4.0, 0.5, 1.0),
        0.5,
        pink.clone(),
    )));
    list.push(Box::new(Sphere::new(
        Vec3::new(3.0, 0.5, 0.25),
        0.5,
        silver.clone(),
    )));
    list.push(Box::new(Sphere::new(
        Vec3::new(2.0, 0.5, -0.5),
        0.5,
        glass.clone(),
    )));
    list.push(Box::new(Sphere::new(
        Vec3::new(4.0, 0.35, -1.15),
        0.35,
        gold.clone(),
    )));

    // create small spheres
    list.push(Box::new(Sphere::new(
        Vec3::new(5.0, 0.20, -0.8),
        0.20,
        glass_rough.clone(),
    )));
    list.push(Box::new(Sphere::new(
        Vec3::new(4.2, 0.20, -0.6),
        0.20,
        glass_rough.clone(),
    )));
    list.push(Box::new(Sphere::new(
        Vec3::new(5.4, 0.20, 0.55),
        0.20,
        gold_rough.clone(),
    )));
    list.push(Box::new(Sphere::new(
        Vec3::new(5.0, 0.20, 0.25),
        0.20,
        silver_rough.clone(),
    )));

    // camera options
    let cam = Camera::new(
        Vec3::new(10.0, 1.0, 0.0),
        Vec3::new(0.0, 0.0, 0.0),
        Vec3::new(0.0, 1.0, 0.0),
        17.5,
        nx as f32 / ny as f32,
        0.1,
        Vec3::new(5.5, 1.0, 0.0).length(),
    );

    (IntersectList::new(list), cam)
}

#[allow(dead_code)]
pub fn rtiow_scene(nx: u32, ny: u32, rng: &mut rand_pcg::Pcg64) -> (IntersectList, Camera) {
    // scene used in `Ray Tracing in One Weekend`
    //
    let mut list: Vec<Box<dyn IntersectEvent>> = Vec::with_capacity(500);

    // create base
    list.push(Box::new(Sphere::new(
        Vec3::new(0.0, -1000.0, 0.0),
        1000.0,
        Lambertian::new(Vec3::new(0.5, 0.5, 0.5)),
    )));

    // create random small spheres
    for x in -11..11 {
        for z in -11..11 {
            let center = Vec3::new(
                x as f32 + 0.9 * rng.gen::<f32>(),
                0.2,
                z as f32 + 0.9 * rng.gen::<f32>(),
            );
            let material_prob = rng.gen::<f32>();
            if material_prob < 0.8 {
                list.push(Box::new(Sphere::new(
                    center,
                    0.2,
                    Lambertian::new(Vec3::new(
                        rng.gen::<f32>() * rng.gen::<f32>(),
                        rng.gen::<f32>() * rng.gen::<f32>(),
                        rng.gen::<f32>() * rng.gen::<f32>(),
                    )),
                )));
            } else if material_prob < 0.95 {
                list.push(Box::new(Sphere::new(
                    center,
                    0.2,
                    Metal::new(
                        0.5 * rng.gen::<f32>(),
                        Vec3::new(
                            0.5 * (1.0 + rng.gen::<f32>()),
                            0.5 * (1.0 + rng.gen::<f32>()),
                            0.5 * (1.0 + rng.gen::<f32>()),
                        ),
                    ),
                )));
            } else {
                list.push(Box::new(Sphere::new(
                    center,
                    0.2,
                    Dielectric::new(1.5, 0.0, Vec3::new(0.8, 0.8, 0.8)),
                )));
            }
        }
    }

    // create large spheres
    list.push(Box::new(Sphere::new(
        Vec3::new(0.0, 1.0, 0.0),
        1.0,
        Dielectric::new(1.5, 0.0, Vec3::new(0.8, 0.8, 0.8)),
    )));
    list.push(Box::new(Sphere::new(
        Vec3::new(-4.0, 1.0, 0.0),
        1.0,
        Lambertian::new(Vec3::new(0.4, 0.2, 0.1)),
    )));
    list.push(Box::new(Sphere::new(
        Vec3::new(4.0, 1.0, 0.0),
        1.0,
        Metal::new(0.0, Vec3::new(0.6, 0.6, 0.6)),
    )));

    // camera options
    let cam = Camera::new(
        Vec3::new(12.0, 2.0, 2.0),
        Vec3::new(0.0, 0.0, 0.0),
        Vec3::new(0.0, 1.0, 0.0),
        20.0,
        nx as f32 / ny as f32,
        0.1,
        10.0,
    );

    (IntersectList::new(list), cam)
}
