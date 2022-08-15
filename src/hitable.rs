use crate::{Point3, Ray, Vec3};

#[derive(Copy, Clone)]
pub struct HitRecord {
    t:f64,
    p: Point3,
    normal: Vec3
}

impl HitRecord {
    pub fn new(t:f64, p:Point3, normal: Vec3) -> HitRecord {
        HitRecord{
            t,
            p,
            normal
        }
    }

    pub fn getT(self) -> f64 {
        self.t
    }

    pub fn get_point(self) -> Point3 {
        self.p
    }

    pub fn get_normal(self) -> Vec3 {
        self.normal
    }
}

pub trait Hitable {
    fn hit(&self,r: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord>;
}