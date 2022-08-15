use crate::hitable::{Hitable, HitRecord};
use crate::Ray;

pub type HitableList = Vec<Box<dyn Hitable>>;

impl Hitable for HitableList {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let mut aux_hit_record = None;
        let mut closest_so_far = t_max;

        for object in self {
            if let Some(rec) = object.hit(r, t_min, closest_so_far) {
                closest_so_far = rec.getT();
                aux_hit_record = Some(rec);
            }
        }
        aux_hit_record
    }
}