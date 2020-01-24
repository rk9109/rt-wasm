use std::f32;

use rand::Rng;
use rand_pcg;

use crate::ray::Ray;
use crate::vec::Vec3;

pub fn random_point_in_disk(rng: &mut rand_pcg::Pcg64) -> Vec3 {
    // return a random point inside the unit circle
    let unit = Vec3::new(1.0, 1.0, 0.0);
    loop {
        // select random points inside the unit square until a selected point
        // is contained inside the unit circle
        let p = Vec3::new(rng.gen::<f32>(), rng.gen::<f32>(), 0.0) * 2.0 - unit;
        if p.squared_length() < 1.0 {
            return p;
        }
    }
}

pub struct Camera {
    pub origin: Vec3,
    pub corner: Vec3,
    pub horizontal: Vec3,
    pub vertical: Vec3,
    pub u: Vec3,
    pub v: Vec3,
    pub radius: f32,
}

impl Camera {
    pub fn new(
        look_from: Vec3,
        look_to: Vec3,
        vup: Vec3,
        vfov: f32,
        aspect: f32,
        aperture: f32,
        focus_dist: f32,
    ) -> Camera {
        // construct camera:
        //   :look_from:  location of the camera
        //   :look_to:    location the camera points to
        //   :vup:        orientation vector
        //   :vfov:       FOV (in degrees)
        //   :aspect:     aspect ratio (horizontal / vertical)
        //   :aperture:   aperture size
        //   :focus_dist:
        let theta = vfov * f32::consts::PI / 180.0;
        let half_height = (theta / 2.0).tan();
        let half_width = aspect * half_height;
        let origin = look_from;
        let w = (look_from - look_to).unit();
        let u = vup.cross(w).unit();
        let v = w.cross(u);
        Camera {
            origin: origin,
            corner: origin
                - v * half_height * focus_dist
                - u * half_width * focus_dist
                - w * focus_dist,
            horizontal: u * half_width * focus_dist * 2.0,
            vertical: v * half_height * focus_dist * 2.0,
            u,
            v,
            radius: aperture / 2.0,
        }
    }

    pub fn point(&self, s: f32, t: f32, rng: &mut rand_pcg::Pcg64) -> Ray {
        // return ray from the origin to coordinate (s, t)
        let rd = random_point_in_disk(rng) * self.radius;
        let offset = self.u * rd.x + self.v * rd.y;
        Ray::new(
            self.origin + offset,
            self.corner + self.horizontal * s + self.vertical * t - self.origin - offset,
        )
    }
}
