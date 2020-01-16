use crate::ray::Ray;
use crate::vec::Vec3;

pub struct Camera {
    pub corner: Vec3,
    pub horizontal: Vec3,
    pub vertical: Vec3,
    pub origin: Vec3,
}

impl Camera {
    pub fn new(corner: Vec3, horizontal: Vec3, vertical: Vec3, origin: Vec3) -> Camera {
        Camera {
            corner,
            horizontal,
            vertical,
            origin,
        }
    }

    pub fn point(&self, u: f64, v: f64) -> Ray {
        Ray::new(
            self.origin,
            self.corner + self.horizontal * u + self.vertical * v - self.origin,
        )
    }
}
