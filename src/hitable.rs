use crate::{Point3, Ray, Vec3, material::Material};
use std::rc::Rc;

pub struct HitRecord {
    t:f64,
    p: Point3,
    normal: Vec3,
    front_face: bool,
    material: Rc<dyn Material>
}

impl HitRecord {
    pub fn new(t:f64, p:Point3, normal: Vec3, front_face:bool, material: Rc<dyn Material>) -> HitRecord {
        HitRecord{
            t,
            p,
            normal,
            front_face,
            material
        }
    }

    pub fn get_front_face(&self) -> bool {
        self.front_face
    }
    pub fn get_material(&self) -> Rc<dyn Material> {
        self.material.clone()
    }
    pub fn get_t(&self) -> f64 {
        self.t
    }

    pub fn get_point(&self) -> Point3 {
        self.p
    }

    pub fn get_normal(&self) -> Vec3 {
        self.normal
    }

    pub fn set_face_normal(&mut self, r:&Ray, outward_normal: Vec3) -> () {
        self.front_face = r.direction().dot(outward_normal) < 0.0;
        self.normal = if self.front_face {
            outward_normal
        }
        else {
            (-1.) * outward_normal
        };
    }
}

pub trait Hitable {
    fn hit(&self,r: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord>;
}