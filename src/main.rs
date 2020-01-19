use std::env;
use std::f64;
use std::fs;

use getopts::Options;
use indicatif::ProgressBar;
use rand::prelude::*;

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

fn color(r: &Ray, world: &IntersectList, depth: u16) -> Vec3 {
    // recursively trace the path of `r` as it intersects objects in `IntersectList`
    if let Some(record) = world.intersect(r, 0.001, f64::MAX) {
        if depth < 50 {
            if let Some((scattered, attenuation)) = record.material.scatter(&r, &record) {
                return color(&scattered, &world, depth + 1) * attenuation;
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

fn parse_args() -> Option<(u64, u64, u64, String)> {
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
    let mut output = String::from("output.ppm");

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
    if matches.opt_present("o") {
        output = matches.opt_str("o").unwrap();
    }

    Some((nx, ny, ns, output))
}

fn main() {
    // parse command-line arguments
    let (nx, ny, ns, output) = match parse_args() {
        Some((nx, ny, ns, output)) => (nx, ny, ns, output),
        None => return,
    };

    // initialize ppm output
    let mut ppm = format!("P3\n{} {}\n{}\n", nx, ny, 255);

    // initialize rng
    let mut rng = rand::thread_rng();

    // initialize world
    let world = IntersectList::new(vec![
        Box::new(Sphere::new(
            Vec3::new(0.0, 0.0, -1.0),
            0.5,
            Lambertian::new(Vec3::new(0.2, 0.2, 0.6)),
        )),
        Box::new(Sphere::new(
            Vec3::new(0.0, -100.5, -1.0),
            100.0,
            Lambertian::new(Vec3::new(0.8, 0.8, 0.0)),
        )),
        Box::new(Sphere::new(
            Vec3::new(1.0, 0.0, -1.0),
            0.5,
            Metal::new(0.2, Vec3::new(0.8, 0.6, 0.2)),
        )),
        Box::new(Sphere::new(
            Vec3::new(-1.0, 0.0, -1.0),
            0.5,
            Dielectric::new(1.5, Vec3::new(1.0, 1.0, 1.0)),
        )),
        Box::new(Sphere::new(
            Vec3::new(-1.0, 0.0, -1.0),
            0.45,
            Dielectric::new(1.5, Vec3::new(1.0, 1.0, 1.0)),
        )),
    ]);

    // initialize the camera
    let cam = Camera::new(
        Vec3::new(-2.0, 2.0, 1.0),
        Vec3::new(0.0, 0.0, -1.0),
        Vec3::new(0.0, 1.0, 0.0),
        25.0,
        nx as f64 / ny as f64,
    );

    // initialize progress bar
    let pb = ProgressBar::new(nx * ny);

    for j in (0..ny).rev() {
        for i in 0..nx {
            pb.inc(1);
            let mut pixel = Vec3::new(0.0, 0.0, 0.0);
            for _ in 0..ns {
                let u = (i as f64 + rng.gen::<f64>()) / nx as f64;
                let v = (j as f64 + rng.gen::<f64>()) / ny as f64;
                let r = cam.point(u, v);
                pixel += color(&r, &world, 0);
            }
            pixel /= ns as f64;
            pixel = Vec3::new(pixel.x.sqrt(), pixel.y.sqrt(), pixel.z.sqrt());
            let ir = (255.99 * pixel.x) as i32;
            let ig = (255.99 * pixel.y) as i32;
            let ib = (255.99 * pixel.z) as i32;

            // ppm pixel
            ppm = format!("{}{} {} {}\n", ppm, ir, ig, ib);
        }
    }
    fs::write(output, &ppm).expect("ppm error");

    pb.finish_and_clear();
}
