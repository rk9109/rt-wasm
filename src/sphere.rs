use crate::intersect::{IntersectEvent, IntersectRecord};
use crate::material::Material;
use crate::ray::Ray;
use crate::vec::Vec3;

pub struct Sphere<M: Material> {
    pub center: Vec3,
    pub radius: f32,
    pub material: M,
}

impl<M: Material> Sphere<M> {
    pub fn new(center: Vec3, radius: f32, material: M) -> Sphere<M> {
        Sphere {
            center,
            radius,
            material,
        }
    }
}

impl<M: Material> IntersectEvent for Sphere<M> {
    fn intersect(&self, r: &Ray, t_min: f32, t_max: f32) -> Option<IntersectRecord> {
        // solution to the quadratic equation:
        //   t*t*dot(b, b)+ 2*t*dot(b, a-c) + dot(a-c, a-c) - r*r = 0
        let oc = r.origin - self.center;
        let a = r.direction.dot(r.direction);
        let b = oc.dot(r.direction);
        let c = oc.dot(oc) - self.radius * self.radius;
        let discriminant = b * b - a * c;
        if discriminant > 0.0 {
            // check both solutions to the quadratic equation
            let t = (-b - discriminant.sqrt()) / a;
            if t_min < t && t < t_max {
                let p = r.point_at_parameter(t);
                let normal = (p - self.center) / self.radius;
                return Some(IntersectRecord::new(t, p, normal, &self.material));
            }
            let t = (-b + discriminant.sqrt()) / a;
            if t_min < t && t < t_max {
                let p = r.point_at_parameter(t);
                let normal = (p - self.center) / self.radius;
                return Some(IntersectRecord::new(t, p, normal, &self.material));
            }
        }
        None
    }
}
