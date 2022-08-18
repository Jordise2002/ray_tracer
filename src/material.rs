use crate::hitable::HitRecord;
use crate::{Color, Ray, Vec3};

pub trait Material {
    fn scatter(&self, r: &Ray, hit_record: &HitRecord) ->Option<(Color, Ray)>;
}

pub struct Lambertian {
    albedo: Color
}

impl Lambertian {
    pub fn new(a: Color) -> Lambertian {
        Lambertian { albedo: a }
    }
}

impl Material for Lambertian {
    fn scatter(&self, _r: &Ray, hit_record: &HitRecord) -> Option<(Color, Ray)> {
        let mut scatter_direction = hit_record.get_normal() + Vec3::random_in_hemisphere();
        if scatter_direction.near_zero() {
            //If the random element is too close to zero they may be some
            //problems later on so we will delete all together
            scatter_direction = hit_record.get_normal();
        }
        let scatter_ray = Ray::new(hit_record.get_point(), scatter_direction);
        Some((self.albedo, scatter_ray))
    }
}

pub struct Metal {
    albedo: Color,
    fuzz: f64
}

impl Metal {
    pub(crate) fn new(albedo: Color, fuzz: f64) -> Metal {
        let mut fuzz = fuzz;
        if fuzz > 1. {
            fuzz = 1.;
        }
        Metal {albedo, fuzz}
    }
}
impl Material for Metal {
    fn scatter(&self, r: &Ray, hit_record: &HitRecord) -> Option<(Color, Ray)> {
        let reflected = r.direction().reflect(hit_record.get_normal()).normalized();
        let scattered = Ray::new(hit_record.get_point(), reflected + self.fuzz * Vec3::random_in_unit_sphere());

        if scattered.direction().dot(hit_record.get_normal()) > 0. {
            Some((self.albedo, scattered))
        }
        else {
            None
        }
    }
}

pub struct Dielectric {
    ri: f64
}

impl Dielectric{
    fn new(ri: f64) -> Dielectric {
        Dielectric {
            ri
        }
    }
}

impl Material for Dielectric {
    fn scatter(&self, r: &Ray, hit_record: &HitRecord) -> Option<(Color, Ray)> {
        let refraction_ratio = if hit_record.get_front_face() {
            1.0 / self.ri
        }
        else {
            self.ri
        };
        let unit_direction = r.direction().normalized();
        let refracted = unit_direction.refracted(hit_record.get_normal(), refraction_ratio);
        let scatered = Ray::new(hit_record.get_point(), refracted);
        Some((Color::new(1.,1.,1.), scatered))

    }
}