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
mod scenes;
mod sphere;
mod vec;

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

fn parse_args() -> Option<Params> {
    // parse command-line arguments
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

    Some(Params::new(nx, ny, ns, random_seed, output))
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

fn main() {
    // parse command-line arguments
    let params = match parse_args() {
        Some(params) => params,
        None => return,
    };

    // initialize rng
    let mut rng = rand_pcg::Pcg64::seed_from_u64(params.random_seed);

    // initialize world and camera
    let (world, cam) = scenes::custom_scene(params.nx, params.ny);

    // initialize timer
    let start = time::Instant::now();

    cast(&params, &world, &cam, &mut rng, true, true);

    // print elapsed time
    let end = time::Instant::now();
    let time_secs = (end - start).as_secs();
    let time_millis = (end - start).subsec_millis();
    println!(
        "elapsed time: {}",
        time_secs as f32 + time_millis as f32 / 1000.0
    );
}
