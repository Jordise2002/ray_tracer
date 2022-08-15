use std::io::{stderr, Write};
use crate::hitable_list::HitableList;
use crate::vec3::{Color, Point3, Vec3};
use crate::ray::Ray;
use sphere::Sphere;
use crate::hitable::Hitable;
use rand::{Rng, thread_rng};
use crate::camera::Camera;

mod vec3;
mod ray;
mod hitable;
mod sphere;
mod hitable_list;
mod camera;

const MAX_FLOAT: f64 = 10000000.0;

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

fn color(r: &Ray, world_list: &HitableList) -> Color {
    let hit = world_list.hit(r, 0.,MAX_FLOAT );
    match(hit) {
        Some(rec)=>{
            0.5 * Vec3::new(rec.get_normal().x() + 1., rec.get_normal().y() + 1., rec.get_normal().z() + 1.)
        },
        None => {
            let unit_direction = r.direction().normalized();
            let t = 0.5 * (unit_direction.y() + 1.);
            (1. - t) * Color::new(1.,1.,1.) + t * Color::new(0.5, 0.7, 1.)
        }
    }
}
fn main() {
    //IMAGE
    const ASPECT_RATIO : f64 = 16.0 / 9.0;
    const IMAGE_WIDTH:u64 = 256;
    const IMAGE_HEIGHT:u64 = ((IMAGE_WIDTH as f64) / ASPECT_RATIO) as u64;
    const SAMPLES_PER_PIXEL:u64 = 100;

    //CAMERA
    let viewport_height = 2.0;
    let viewport_witdth = ASPECT_RATIO * viewport_height;
    let focal_length = 1.0;

    let origin = Point3::new(0.,0.,0.);
    let horizontal = Vec3::new(viewport_witdth, 0.,0.);
    let vertical = Vec3::new(0., viewport_height, 0.);
    let lower_left_corner = origin - horizontal / 2.0 - vertical / 2.0 - Vec3::new(0., 0., focal_length);

    let camera = Camera::new(origin,lower_left_corner,vertical, horizontal);

    //OBJECTS
    let mut world_list = HitableList::new();
    world_list.push(Box::new(Sphere::new(Point3::new(0., 0., -1.), 0.5)));
    world_list.push(Box::new(Sphere::new(Point3::new(0.,-100.5,-1.), 100.0)));
    println!("P3");
    println!("{} {}", IMAGE_WIDTH, IMAGE_HEIGHT);
    println!("255");

    //Initialize RNG
    let mut rng = thread_rng();
    for j in (0..IMAGE_HEIGHT).rev() {
        eprint!("\rScanlines Remaining: {:3}", IMAGE_HEIGHT -j -1);
        stderr().flush().unwrap();

        for i in 0..IMAGE_WIDTH {
            let mut col = Color::new(0.,0.,0.);
            for s in 0..SAMPLES_PER_PIXEL {
                let u = (i as f64 + rng.gen::<f64>()) / (IMAGE_WIDTH -1) as f64;
                let v = (j as f64 + rng.gen::<f64>()) / (IMAGE_HEIGHT -1) as f64;
                let r = camera.get_ray(u,v);
                col += color(&r, &world_list);
            }

            col /= SAMPLES_PER_PIXEL as f64;

            println!("{}", col.format_color())
        }
    }
}