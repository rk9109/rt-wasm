use std::env;
use std::f32;
use std::time;

use getopts::Options;
use image;
use indicatif;
use rand::{Rng, SeedableRng};
use rand_pcg;

mod camera;
mod intersect;
mod material;
mod ray;
mod sphere;
mod vec;

use camera::Camera;
use intersect::{IntersectEvent, IntersectList};
use material::{Dielectric, Lambertian, Metal};
use ray::Ray;
use sphere::Sphere;
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

fn parse_args() -> Option<(u32, u32, u32, u64, String)> {
    // parse command-line arguments
    // returns:
    //   :nx:     x-dimension of image
    //   :ny:     y-dimension of image
    //   :ns:     number of samples per pixel
    //   :output: output image filename
    let args: Vec<String> = env::args().collect();

    let mut opts = Options::new();
    opts.optflag("h", "help", "");
    opts.optopt("x", "", "x resolution", "INT");
    opts.optopt("y", "", "y resolution", "INT");
    opts.optopt("s", "samples", "samples per pixel", "INT");
    opts.optopt("r", "random", "random seed for RNG", "INT");
    opts.optopt("o", "output", "output filename", "FILE");

    let matches = match opts.parse(&args[1..]) {
        Ok(matches) => matches,
        Err(error) => {
            println!("{}", error);
            return None;
        }
    };

    // process --help option
    if matches.opt_present("h") {
        let message = format!("Usage: {} [options]", args[0]);
        println!("{}", opts.usage(&message));
        return None;
    }

    // initialize default values
    let mut nx = 200;
    let mut ny = 100;
    let mut ns = 100;
    let mut random_seed = 0;
    let mut output = String::from("output.png");

    // parse optional options
    if matches.opt_present("x") {
        nx = matches.opt_str("x").unwrap().parse().unwrap();
    }
    if matches.opt_present("y") {
        ny = matches.opt_str("y").unwrap().parse().unwrap();
    }
    if matches.opt_present("s") {
        ns = matches.opt_str("s").unwrap().parse().unwrap();
    }
    if matches.opt_present("r") {
        random_seed = matches.opt_str("r").unwrap().parse().unwrap();
    }
    if matches.opt_present("o") {
        output = matches.opt_str("o").unwrap();
    }

    Some((nx, ny, ns, random_seed, output))
}

fn create_scene() -> IntersectList {
    let mut list: Vec<Box<dyn IntersectEvent>> = vec![];

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

    IntersectList::new(list)
}

fn main() {
    // parse command-line arguments
    let (nx, ny, ns, random_seed, output) = match parse_args() {
        Some((nx, ny, ns, random_seed, output)) => (nx, ny, ns, random_seed, output),
        None => return,
    };

    // initialize image
    let mut image = Vec::with_capacity((3 * nx * ny) as usize);

    // initialize rng
    let mut rng = rand_pcg::Pcg64::seed_from_u64(random_seed);

    // initialize world
    let world = create_scene();

    // initialize the camera
    let cam = Camera::new(
        Vec3::new(10.0, 1.0, 0.0),
        Vec3::new(0.0, 0.0, 0.0),
        Vec3::new(0.0, 1.0, 0.0),
        17.5,
        nx as f32 / ny as f32,
        0.1,
        Vec3::new(5.5, 1.0, 0.0).length(),
    );

    // initialize progress bar
    let pb = indicatif::ProgressBar::new((nx * ny) as u64);

    // initialize timer
    let start = time::Instant::now();

    for j in (0..ny).rev() {
        for i in 0..nx {
            pb.inc(1);
            let mut pixel = Vec3::new(0.0, 0.0, 0.0);
            for _ in 0..ns {
                let u = (i as f32 + rng.gen::<f32>()) / nx as f32;
                let v = (j as f32 + rng.gen::<f32>()) / ny as f32;
                let r = cam.point(u, v, &mut rng);
                pixel += color(&r, &world, 0, &mut rng);
            }
            pixel /= ns as f32;
            image.push((255.99 * pixel.x.sqrt()) as u8);
            image.push((255.99 * pixel.y.sqrt()) as u8);
            image.push((255.99 * pixel.z.sqrt()) as u8);
        }
    }
    image::save_buffer(output, &image, nx, ny, image::RGB(8)).expect("error saving image");

    pb.finish_and_clear();

    // print elapsed time
    let end = time::Instant::now();
    let time_secs = (end - start).as_secs();
    let time_millis = (end - start).subsec_millis();
    println!(
        "elapsed time: {}",
        time_secs as f32 + time_millis as f32 / 1000.0
    );
}
