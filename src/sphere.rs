use crate::intersect::{IntersectEvent, IntersectRecord};
use crate::ray::Ray;
use crate::vec::Vec3;

pub struct Sphere {
    pub center: Vec3,
    pub radius: f64,
}

impl Sphere {
    pub fn new(center: Vec3, radius: f64) -> Sphere {
        Sphere { center, radius }
    }
}

impl IntersectEvent for Sphere {
    // TODO:
    // t*t*dot(b, b)+ 2*t*dot(b, a-c) + dot(a-c, a-c) - r*r = 0
    fn intersect(&self, r: &Ray, t_min: f64, t_max: f64) -> Option<IntersectRecord> {
        let oc = r.origin - self.center;
        let a = r.direction.dot(r.direction);
        let b = oc.dot(r.direction);
        let c = oc.dot(oc) - self.radius * self.radius;
        let discriminant = b * b - a * c;
        if discriminant > 0.0 {
            let t = (-b - discriminant.sqrt()) / a;
            if t_min < t && t < t_max {
                let p = r.point_at_parameter(t);
                let normal = (p - self.center) / self.radius;
                return Some(IntersectRecord::new(t, p, normal));
            }
            let t = (-b + discriminant.sqrt()) / a;
            if t_min < t && t < t_max {
                let p = r.point_at_parameter(t);
                let normal = (p - self.center) / self.radius;
                return Some(IntersectRecord::new(t, p, normal));
            }
        }
        None
    }
}
