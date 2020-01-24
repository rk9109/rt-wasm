use rand::Rng;
use rand_pcg;

use crate::intersect::IntersectRecord;
use crate::ray::Ray;
use crate::vec::Vec3;

pub fn random_point_in_sphere(rng: &mut rand_pcg::Pcg64) -> Vec3 {
    // return a random point contained inside the unit sphere
    let unit = Vec3::new(1.0, 1.0, 1.0);
    loop {
        // select random points inside the unit cube until a selected point
        // is contained inside the unit sphere
        let p = Vec3::new(rng.gen::<f32>(), rng.gen::<f32>(), rng.gen::<f32>()) * 2.0 - unit;
        if p.squared_length() < 1.0 {
            return p;
        }
    }
}

pub fn reflect(v: Vec3, n: Vec3) -> Vec3 {
    // reflect ray direction `v` across normal `n`
    v - n * v.dot(n) * 2.0
}

pub fn refract(v: Vec3, n: Vec3, ni_over_nt: f32) -> Option<Vec3> {
    // refract ray direction `v` across normal `n`
    // `ni_over_nt` is the ratio of material refractive indices
    let uv = v.unit();
    let dt = uv.dot(n);
    let discriminant = 1.0 - ni_over_nt * ni_over_nt * (1.0 - dt * dt);
    if discriminant > 0.0 {
        let refracted = (uv - n * dt) * ni_over_nt - n * discriminant.sqrt();
        Some(refracted)
    } else {
        None
    }
}

pub fn schlick(cosine: f32, refractive_idx: f32) -> f32 {
    // schlick's approximation for the fresnel equations
    let r0 = ((1.0 - refractive_idx) / (1.0 + refractive_idx)).powi(2);
    r0 + (1.0 - r0) * (1.0 - cosine).powi(5)
}

pub trait Material {
    fn scatter(
        &self,
        r: &Ray,
        record: &IntersectRecord,
        rng: &mut rand_pcg::Pcg64,
    ) -> Option<(Ray, Vec3)>;
}

#[derive(Debug, Copy, Clone)]
pub struct Lambertian {
    pub albedo: Vec3,
}

impl Lambertian {
    pub fn new(albedo: Vec3) -> Lambertian {
        Lambertian { albedo }
    }
}

impl Material for Lambertian {
    fn scatter(
        &self,
        _r: &Ray,
        record: &IntersectRecord,
        rng: &mut rand_pcg::Pcg64,
    ) -> Option<(Ray, Vec3)> {
        // scattering on a lambertian surface
        let target = record.p + record.normal + random_point_in_sphere(rng);
        let scattered = Ray::new(record.p, target - record.p);
        let attenuation = self.albedo;
        Some((scattered, attenuation))
    }
}

#[derive(Debug, Copy, Clone)]
pub struct Metal {
    pub fuzz: f32,
    pub albedo: Vec3,
}

impl Metal {
    pub fn new(fuzz: f32, albedo: Vec3) -> Metal {
        Metal {
            fuzz: if fuzz < 1.0 { fuzz } else { 1.0 },
            albedo,
        }
    }
}

impl Material for Metal {
    fn scatter(
        &self,
        r: &Ray,
        record: &IntersectRecord,
        rng: &mut rand_pcg::Pcg64,
    ) -> Option<(Ray, Vec3)> {
        // scattering on a metal surface
        let reflected = reflect(r.direction.unit(), record.normal);
        let scattered = Ray::new(
            record.p,
            reflected + random_point_in_sphere(rng) * self.fuzz,
        );
        let attenuation = self.albedo;
        if scattered.direction.dot(record.normal) > 0.0 {
            return Some((scattered, attenuation));
        }
        None
    }
}

#[derive(Debug, Copy, Clone)]
pub struct Dielectric {
    pub ri: f32,
    pub fuzz: f32,
    pub albedo: Vec3,
}

impl Dielectric {
    pub fn new(ri: f32, fuzz: f32, albedo: Vec3) -> Dielectric {
        Dielectric { ri, fuzz, albedo }
    }
}

impl Material for Dielectric {
    fn scatter(
        &self,
        r: &Ray,
        record: &IntersectRecord,
        rng: &mut rand_pcg::Pcg64,
    ) -> Option<(Ray, Vec3)> {
        // scattering on a dielectric surface
        let attenuation = self.albedo;
        let cos_theta = r.direction.dot(record.normal);

        let (outward_normal, ni_over_nt, cos) = if cos_theta > 0.0 {
            // light passes from dielectric to vacuum
            let outward_normal = -record.normal;
            let ni_over_nt = self.ri;
            let cos = self.ri * cos_theta / r.direction.length();
            (outward_normal, ni_over_nt, cos)
        } else {
            // light passes from vacuum to dielectric
            let outward_normal = record.normal;
            let ni_over_nt = 1.0 / self.ri;
            let cos = -cos_theta / r.direction.length();
            (outward_normal, ni_over_nt, cos)
        };

        // both reflection and refraction occur in a dielectric material; return reflected
        // or refracted light based on `reflect_prob`
        if let Some(refracted) = refract(r.direction, outward_normal, ni_over_nt) {
            let reflect_prob = schlick(cos, self.ri);
            if rng.gen::<f32>() >= reflect_prob {
                let scattered = Ray::new(
                    record.p,
                    refracted + random_point_in_sphere(rng) * self.fuzz,
                );
                return Some((scattered, attenuation));
            }
        }
        let reflected = reflect(r.direction.unit(), record.normal);
        let scattered = Ray::new(
            record.p,
            reflected + random_point_in_sphere(rng) * self.fuzz,
        );
        Some((scattered, attenuation))
    }
}
