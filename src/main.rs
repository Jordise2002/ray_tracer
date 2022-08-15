use std::io::{stderr, Write};
use crate::vec3::{Color, Point3, Vec3};
use crate::ray::Ray;
mod vec3;
mod ray;
mod hitable;
mod sphere;

fn hit_sphere(center: Vec3, radius: f64, r: &Ray) -> f64{
    let oc = r.origin() - center;
    let a = r.direction().dot(r.direction());
    let b = 2.0 * oc.dot(r.direction());
    let c = oc.dot(oc) - radius * radius;
    let discriminant = b * b - 4.0 * a * c;
    if discriminant < 0.0 {
        -1.
    }
    else {
        (-b -discriminant.sqrt()) / (2. * a)
    }
}

fn color(r: &Ray) -> Color {
    let hit =  hit_sphere(Point3::new(0., 0.,  -1.), 0.5, r);
    if hit > 0. {
        let n = (r.point_at_parameter(hit) - Point3::new(0., 0., -1.)).normalized();
        return 0.5 * Color::new(n.x() + 1., n.y() + 1., n.z() + 1.);
    }
    let unit_direction = r.direction().normalized();
    let t =  0.5 * (unit_direction.y() + 1.0);
    Color::new(1.,1.,1.) * (1.0 -t) + Color::new(0.5, 0.7,1.) * t
}

fn main() {
    //IMAGE
    const ASPECT_RATIO : f64 = 16.0 / 9.0;
    const IMAGE_WIDTH:u64 = 256;
    const IMAGE_HEIGHT:u64 = ((IMAGE_WIDTH as f64) / ASPECT_RATIO) as u64;

    //CAMERA
    let viewport_height = 2.0;
    let viewport_witdth = ASPECT_RATIO * viewport_height;
    let focal_length = 1.0;

    let origin = Point3::new(0.,0.,0.);
    let horizontal = Vec3::new(viewport_witdth, 0.,0.);
    let vertical = Vec3::new(0., viewport_height, 0.);
    let lower_left_corner = origin - horizontal / 2.0 - vertical / 2.0 - Vec3::new(0., 0., focal_length);

    println!("P3");
    println!("{} {}", IMAGE_WIDTH, IMAGE_HEIGHT);
    println!("255");

    for j in (0..IMAGE_HEIGHT).rev() {
        eprint!("\rScanlines Remaining: {:3}", IMAGE_HEIGHT -j -1);
        stderr().flush().unwrap();

        for i in 0..IMAGE_WIDTH {
            let u = (i as f64) / ((IMAGE_WIDTH - 1) as f64);
            let v = (j as f64) / ((IMAGE_HEIGHT - 1) as f64);

            let r = Ray::new(origin, lower_left_corner + u * horizontal + v * vertical - origin);

            let pixel_color = color(&r);

            println!("{}", pixel_color.format_color())
        }
    }
}