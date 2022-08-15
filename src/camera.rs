use crate::{Point3, Ray, Vec3};

pub struct Camera {
    origin: Point3,
    lower_left_corner: Point3,
    vertical: Vec3,
    horizontal: Vec3
}

impl Camera {
    pub fn new(origin: Point3, lower_left_corner: Point3, vertical: Vec3, horizontal: Vec3) -> Camera {
        Camera {
            origin,
            lower_left_corner,
            vertical,
            horizontal
        }
    }

    pub fn get_ray(&self, u: f64, v: f64) -> Ray {
        Ray::new(self.origin, self.lower_left_corner + u * self.horizontal + v * self.vertical)
    }

}