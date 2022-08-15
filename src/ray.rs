use crate::Vec3;

pub struct Ray{
    a: Vec3,// Point of origin of the Ray
    b: Vec3// Direction of the Ray
}

impl Ray {

    pub fn new(origin: Vec3, direction: Vec3) -> Ray {
        Ray{ a:origin, b:direction}
    }

    pub fn origin(&self) -> Vec3 {
        self.a
    }

    pub fn direction(&self) -> Vec3 {
        self.b
    }
    //returns a point in the ray at a given distance from origin
    pub fn point_at_parameter(&self, t:f64) -> Vec3 {
        self.a + self.b * t
    }

}