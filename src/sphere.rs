use crate::hitable::{Hitable, HitRecord};
use crate::{Point3, Ray};

#[derive(Copy, Clone)]
pub struct Sphere{
    center: Point3,
    radius: f64
}

impl Sphere {
    pub fn new(center: Point3, radius: f64) -> Sphere {
        Sphere {
            center, radius
        }
    }
}

impl Hitable for Sphere {
    fn hit(&self,r: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord>
    {
        let oc = r.origin() - self.center;
        let a = r.direction().dot(r.direction());
        let b = oc.dot(r.direction());
        let c = oc.dot(oc) - self.radius * self.radius;
        let discriminant = b * b - a * c;

        if discriminant > 0. {
            let temp = (-b - (b*b - a*c)).sqrt() / a;
            if temp < t_max && temp > t_min {
                let hit_record = HitRecord::new(
                    temp,
                    r.point_at_parameter(temp),
                    (r.point_at_parameter(temp) - self.center) / self.radius
                );
                return Option::Some(hit_record);
            }
            let temp = (-b + (b*b - a * c).sqrt()) /a;
            if temp < t_max && temp > t_min {
                let hit_record = HitRecord::new(
                    temp,
                    r.point_at_parameter(temp),
                    (r.point_at_parameter(temp) - self.center) / self.radius
                        );
                return Option::Some(hit_record);
            }
        };
        Option::None

    }
}

