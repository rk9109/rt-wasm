use crate::ray::Ray;
use crate::vec::Vec3;

pub struct IntersectRecord {
    pub t: f64,
    pub p: Vec3,
    pub normal: Vec3,
}

impl IntersectRecord {
    pub fn new(t: f64, p: Vec3, normal: Vec3) -> IntersectRecord {
        IntersectRecord { t, p, normal }
    }
}

pub trait IntersectEvent {
    fn intersect(&self, r: &Ray, t_min: f64, t_max: f64) -> Option<IntersectRecord>;
}

pub struct IntersectList {
    list: Vec<Box<dyn IntersectEvent>>,
}

impl IntersectList {
    pub fn new(list: Vec<Box<dyn IntersectEvent>>) -> IntersectList {
        IntersectList { list }
    }
}

impl IntersectEvent for IntersectList {
    fn intersect(&self, r: &Ray, t_min: f64, t_max: f64) -> Option<IntersectRecord> {
        let mut t_nearest = t_max;
        let mut intersect: Option<IntersectRecord> = None;
        for list_item in self.list.iter() {
            if let Some(record) = list_item.intersect(r, t_min, t_nearest) {
                t_nearest = record.t;
                intersect = Some(record);
            }
        }
        intersect
    }
}
