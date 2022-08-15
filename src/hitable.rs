use crate::{Point3, Ray, vec3, Vec3};

pub struct HitRecord {
    t:f64,
    p: Point3,
    normal: Vec3
}

pub trait hitable {
    fn hit(r: &Ray, t_min: f64, t_max: f64, rec: &HitRecord) -> Option<HitRecord>;
}