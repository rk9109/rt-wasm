use std::f64;

use crate::ray::Ray;
use crate::vec::Vec3;

pub struct Camera {
    pub origin: Vec3,
    pub corner: Vec3,
    pub horizontal: Vec3,
    pub vertical: Vec3,
}

impl Camera {
    pub fn new(look_from: Vec3, look_to: Vec3, vup: Vec3, vfov: f64, aspect: f64) -> Camera {
        // construct camera:
        //   :look_from: location of the camera
        //   :look_to:   location the camera points to
        //   :vup:       orientation vector
        //   :vfov:      FOV (in degrees)
        //   :aspect:    aspect ratio (horizontal / vertical)
        let theta = vfov * f64::consts::PI / 180.0;
        let half_height = (theta / 2.0).tan();
        let half_width = aspect * half_height;
        let origin = look_from;
        let w = (look_from - look_to).unit();
        let u = vup.cross(w).unit();
        let v = w.cross(u);
        Camera {
            origin: origin,
            corner: origin - v * half_height - u * half_width - w,
            horizontal: u * half_width * 2.0,
            vertical: v * half_height * 2.0,
        }
    }

    pub fn point(&self, s: f64, t: f64) -> Ray {
        // return ray from the origin to coordinate (s, t)
        Ray::new(
            self.origin,
            self.corner + self.horizontal * s + self.vertical * t - self.origin,
        )
    }
}
