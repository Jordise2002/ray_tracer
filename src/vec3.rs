
use std::ops;
use std::fmt;
use std::fmt::Display;
use std::ops::Range;
use rand::{Rng, thread_rng};

#[derive(Copy, Clone)]
pub struct Vec3{
    e: [f64;3]
}

pub type Point3 = Vec3;
pub type Color = Vec3;

impl Vec3{

    pub fn new(x:f64, y:f64, z:f64) -> Vec3 {
        Vec3{ e: [x,y,z] }
    }

    pub fn x(self) -> f64 {
        self[0]
    }

    pub fn y(self) -> f64 {
        self[1]
    }

    pub fn z(self) -> f64 {
        self[2]
    }
    fn random_vector(r: Range <f64>) -> Vec3
    {
        let mut rng = thread_rng();
        Vec3::new(rng.gen_range(r.clone()), rng.gen_range(r.clone()), rng.gen_range(r.clone()))
    }

    pub fn random_in_unit_sphere() -> Point3 {
        loop {
            let p = Vec3::random_vector(-1.0..1.0);
            if p.length() < 1.0
            {
                return p;
            }
        }
    }

    pub fn random_in_hemisphere() -> Point3 {
        let in_unit = Self::random_in_unit_sphere();
        if in_unit.squared_length() > 0. {
            in_unit
        }
        else {
            -1. * in_unit
        }
    }

    pub fn near_zero(self) -> bool {
        let eps:f64 = 1.0e-8;
        self[0] < eps && self[1] < eps && self[2] < eps
    }

    pub fn reflect(self, n: Vec3) -> Vec3 {
        self - 2. * self.dot(n) * n
    }

    pub fn dot(self, other: Vec3) -> f64 {
        self[0] * other[0] + self[1] * other[1] + self[2] * other[2]
    }

    pub fn length(self) -> f64 {
        self.dot(self).sqrt()
    }

    pub fn squared_length(self) -> f64 {
        self.dot(self)
    }
    pub fn cross(self, other: Vec3) -> Vec3 {
        Vec3{
            e : [
                self[1] * other[2] - self[2] * other[1],
                self[2] * other[0] - self[0] * other[2],
                self[0] * other[1] - self[1] * other[0]
            ]
        }
    }

    pub fn normalized(self) -> Vec3 {
        self / self.length()
    }

    pub fn format_color(self, samples_per_pixel:u64) -> String {
        format!("{} {} {}", (256.0 * (self[0] / (samples_per_pixel as f64)).sqrt().clamp(0.0, 0.999)) as u64,
                             (256.0 * (self[1] / (samples_per_pixel as f64)).sqrt().clamp(0.0, 0.999)) as u64,
                              (256.0 * (self[2] / (samples_per_pixel as f64)).sqrt().clamp(0.0, 0.999)) as u64)
    }

    pub fn refract(self, n: Vec3, etai_over_etat: f64) -> Vec3 {
        let cos_theta = ((-1.0) * self).dot(n).min(1.0);
        let r_out_perp = etai_over_etat * (self + cos_theta * n);
        let r_out_parallel = -(1.0 - r_out_perp.length().powi(2)).abs().sqrt() * n;
        r_out_perp + r_out_parallel
    }
}

impl Display for Vec3 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({}, {}, {})", self[0], self[1], self[2])
    }
}

impl ops::Index<usize> for Vec3 {
    type Output = f64;

    fn index(&self, index: usize) -> &Self::Output {
        &self.e[index]
    }
}

impl ops::IndexMut<usize> for Vec3 {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.e[index]
    }
}

impl ops::Add for Vec3 {
    type Output = Vec3;

    fn add(self, other: Vec3) -> Vec3{
        Vec3 {
            e: [self.x() + other.x(), self.y() + other.y(), self.z() + other.z()]
        }
    }
}

impl ops::AddAssign for Vec3 {
    fn add_assign(&mut self, rhs: Self) {
        *self = Vec3 {
            e: [self.x() + rhs.x(), self.y() + rhs.y(), self.z() + rhs.z()]
        }
    }
}

impl ops::Sub for Vec3 {
    type Output = Vec3;
    fn sub(self, other: Vec3) ->Vec3 {
        Vec3 {
            e: [self.x() - other.x(), self.y() - other.y(), self.z() - other.z()]
        }
    }
}

impl ops::SubAssign for Vec3 {
    fn sub_assign(&mut self, rhs: Self) {
        *self = Vec3 {
            e: [self.x() - rhs.x(), self.y() - rhs.y(), self.z() - rhs.z()]
        }
    }
}

impl ops::Mul<f64> for Vec3 {
    type Output = Vec3;

    fn mul(self, rhs: f64) -> Self::Output {
        Vec3 {
            e: [self.x() * rhs, self.y() * rhs, self.z() * rhs]
        }
    }
}

impl ops::MulAssign<f64> for Vec3 {
    fn mul_assign(&mut self, rhs: f64) {
        *self = Vec3 {
            e: [self.x() * rhs, self.y() * rhs, self.z() * rhs]
        }
    }
}

impl ops::Mul<Vec3> for f64 {
    type Output = Vec3;

    fn mul(self, rhs: Vec3) -> Self::Output {
        Vec3 {
            e: [self * rhs.x(), self * rhs.y(), self * rhs.z()]
        }
    }
}
impl ops::Mul<Vec3> for Vec3 {
    type Output = Vec3;
    fn mul(self, rhs: Vec3) -> Self::Output {
        Vec3::new(self.x() * rhs.x(), self.y() * rhs.y(), self.z() * rhs.z())
    }
}
impl ops::Div<f64> for Vec3 {
    type Output = Vec3;

    fn div(self, rhs: f64) -> Self::Output {
        Vec3 {
            e: [self.x() / rhs, self.y() / rhs, self.z() / rhs]
        }
    }
}

impl ops::DivAssign<f64> for Vec3 {
    fn div_assign(&mut self, rhs: f64) {
        *self = Vec3 {
            e : [self.x() / rhs, self.y() / rhs, self.z() / rhs]
        }
    }
}
