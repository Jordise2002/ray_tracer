use crate::hitable::{Hitable, HitRecord};
use crate::{Point3, Ray};
use std::rc::Rc;
use crate::material::Material;


pub struct Sphere{
    center: Point3,
    radius: f64,
    material: Rc<dyn Material>
}

impl Sphere {
    pub fn new(center: Point3, radius: f64, material: Rc<dyn Material>) -> Sphere {
        Sphere {
            center, radius, material
        }
    }
}

impl Hitable for Sphere {
    fn hit(&self,r: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord>
    {
        let oc = r.origin() - self.center;
        let a = r.direction().squared_length();
        let half_b = oc.dot(r.direction());
        let c = oc.squared_length() - self.radius.powi(2);

        let discriminant = half_b.powi(2) - a * c;

        if discriminant < 0.0 {
            return None;
        }

        let sqrtd = discriminant.sqrt();
        let mut root = (-half_b -sqrtd) / a;
        if root < t_min || t_max < root {
            root = (-half_b + sqrtd) / a;
            if root < t_min || t_max < root {
                return None;
            }
        }


        let mut rec = HitRecord::new(root, r.point_at_parameter(root), (r.point_at_parameter(root)-self.center)/self.radius, false, self.material.clone());
        let outward_normal = (rec.get_point() - self.center) / self.radius;
        rec.set_face_normal(r, outward_normal);
        Some(rec)
    }
}

