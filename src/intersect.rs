use crate::material::Material;
use crate::ray::Ray;
use crate::vec::Vec3;

pub struct IntersectRecord<'a> {
    pub t: f32,
    pub p: Vec3,
    pub normal: Vec3,
    pub material: &'a dyn Material,
}

impl<'a> IntersectRecord<'a> {
    pub fn new(t: f32, p: Vec3, normal: Vec3, material: &'a dyn Material) -> IntersectRecord {
        // construct intersection record
        //   :t:        timestep of intersection
        //   :p:        intersection point
        //   :normal:   surface normal at intersection point
        //   :material: surface material
        IntersectRecord {
            t,
            p,
            normal,
            material,
        }
    }
}

pub trait IntersectEvent {
    fn intersect(&self, r: &Ray, t_min: f32, t_max: f32) -> Option<IntersectRecord>;
}

pub struct IntersectList {
    list: Vec<Box<dyn IntersectEvent>>,
}

impl IntersectList {
    pub fn new(list: Vec<Box<dyn IntersectEvent>>) -> IntersectList {
        // construct a list of intersectable objects
        IntersectList { list }
    }
}

impl IntersectEvent for IntersectList {
    fn intersect(&self, r: &Ray, t_min: f32, t_max: f32) -> Option<IntersectRecord> {
        // return an `IntersectRecord` recording the first intersection of `r`
        // into an intersectable object in `IntersectList`
        let mut t_nearest = t_max;
        let mut intersect: Option<IntersectRecord> = None;
        for list_item in self.list.iter() {
            // determine if `r` intersects each object in `IntersectList`
            if let Some(record) = list_item.intersect(r, t_min, t_nearest) {
                // record the nearest object intersected
                t_nearest = record.t;
                intersect = Some(record);
            }
        }
        // return nearest object
        intersect
    }
}
