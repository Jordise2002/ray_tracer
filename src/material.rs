use rand::Rng;
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
    pub fn new(ri: f64) -> Dielectric {
        Dielectric {
            ri
        }
    }
    fn reflectance(cosine:f64, ref_idx:f64) -> f64 {
        let r0 = ((1.0-ref_idx) / (1.0 + ref_idx)).powi(2);
        r0 + (1.0 - r0) * (1.0 - cosine).powi(5)
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
        let cos_theta = ((-1.0) * unit_direction).dot(hit_record.get_normal()).min(1.0);
        let sin_theta = (1.0 - cos_theta.powi(2)).sqrt();

        let mut rng = rand::thread_rng();
        let cannot_refract = refraction_ratio * sin_theta > 1.0;
        let will_reflect = rng.gen::<f64>() < Self::reflectance(cos_theta, refraction_ratio);

        let direction = if cannot_refract || will_reflect {
            unit_direction.reflect(hit_record.get_normal())
        }
        else {
            unit_direction.refract(hit_record.get_normal(), refraction_ratio)
        };

        let scattered = Ray::new(hit_record.get_point(), direction);

        Some((Color::new(1.,1.,1.), scattered))

    }
}